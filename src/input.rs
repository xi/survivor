use std::io::Read;

extern crate libc;

pub struct Input {
    termios: libc::termios,
}

impl Input {
    pub fn new() -> Input {
        let mut t = libc::termios {
            c_iflag: 0,
            c_oflag: 0,
            c_cflag: 0,
            c_lflag: 0,
            c_line: 0,
            c_cc: [0; 32],
            c_ispeed: 0,
            c_ospeed: 0,
        };
        unsafe {
            libc::tcgetattr(0, &mut t);
        }

        let input = Input { termios: t };
        input.cbreak();
        return input;
    }

    fn cbreak(&self) {
        let mut t = self.termios.clone();
        t.c_lflag &= !(libc::ICANON|libc::ECHO);
        t.c_cc[libc::VMIN] = 0;
        t.c_cc[libc::VTIME] = 0;
        unsafe {
            libc::tcsetattr(0, libc::TCSADRAIN, &t);
        }
    }

    pub fn getch(&self) -> Option<u8> {
        let mut stdin = std::io::stdin();
        let mut buf = [0];
        stdin.read(&mut buf[..]).ok()?;
        return if buf[0] == 0 { None } else { Some(buf[0]) };
    }
}

impl Drop for Input {
    fn drop(&mut self) {
        unsafe {
            libc::tcsetattr(0, libc::TCSADRAIN, &self.termios);
        }
    }
}
