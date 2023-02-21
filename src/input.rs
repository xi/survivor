extern crate libc;

use std::io::Read;

pub struct Input {
    termios: libc::termios,
}

impl Input {
    pub fn new() -> Self {
        let mut t: libc::termios;
        unsafe {
            t = std::mem::zeroed();
            libc::tcgetattr(0, &mut t);
        }

        let input = Self { termios: t };
        input.cbreak();
        return input;
    }

    fn cbreak(&self) {
        let mut t = self.termios.clone();
        t.c_lflag &= !(libc::ICANON | libc::ECHO);
        t.c_cc[libc::VMIN] = 0;
        t.c_cc[libc::VTIME] = 0;
        unsafe {
            libc::tcsetattr(0, libc::TCSADRAIN, &t);
        }
    }

    fn _getch(&self) -> Option<u8> {
        let mut stdin = std::io::stdin();
        let mut buf = [0];
        let count = stdin.read(&mut buf[..]).ok()?;
        return if count == 0 { None } else { Some(buf[0]) };
    }

    pub fn getch(&self) -> Option<u8> {
        let c0 = self._getch()?;

        if c0 == 27 {
            self._getch()?;
            let c2 = self._getch()?;
            return Some(c2);
        } else {
            return Some(c0);
        }
    }
}

impl Drop for Input {
    fn drop(&mut self) {
        unsafe {
            libc::tcsetattr(0, libc::TCSADRAIN, &self.termios);
        }
    }
}
