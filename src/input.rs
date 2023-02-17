use std::io::Read;

extern crate libc;

pub struct Input {
    termios: libc::Termios,
}

impl Input {
    pub fn new() -> Input {
        let t = libc::Termios::new();
        unsafe {
            libc::ioctl(0, libc::TCGETA, &t);
        }

        let input = Input { termios: t };
        input.cbreak();
        input.nonblock();
        return input;
    }

    fn cbreak(&self) {
        let mut t = self.termios.clone();
        t.c_lflag &= !(libc::ICANON|libc::ECHO);
        unsafe {
            libc::ioctl(0, libc::TCSETAW, &t);
        }
    }

    fn nonblock(&self) {
        unsafe {
            let fl = libc::fcntl(0, libc::F_GETFL);
            libc::fcntl(0, libc::F_SETFL, fl|libc::O_NONBLOCK);
        }
    }

    pub fn getch(&self) -> Option<u8> {
        let mut stdin = std::io::stdin();
        let mut buf = [0];
        let count = stdin.read(&mut buf[..]).ok()?;
        return Some(buf[0]);
    }
}

impl Drop for Input {
    fn drop(&mut self) {
        unsafe {
            libc::ioctl(0, libc::TCSETAW, &self.termios);
        }
    }
}
