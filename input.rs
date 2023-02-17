use std::io::Read;

const TCGETA: u64 = 0x5405;
const TCSETAW: u64 = 0x5407;
const ICANON: u16 = 0x0002;
const ECHO: u16 = 0x0008;
const GETFL: i32 = 3;
const SETFL: i32 = 4;
const NONBLOCK: i32 = 00004000;

extern "C" {
    fn ioctl(fd: i32, req: u64, ...) -> i32;
    fn fcntl(fd: i32, cmd: i32, ...) -> i32;
}

#[derive(Clone)]
struct Termios {
    _c_iflag: u16,
    _c_oflag: u16,
    _c_cflag: u16,
    c_lflag: u16,
    _c_line: u8,
    _c_cc: [u8; 8],
}

pub struct Input {
    termios: Termios,
}

impl Input {
    pub fn new() -> Input {
        let t = Termios {
            _c_iflag: 0,
            _c_oflag: 0,
            _c_cflag: 0,
            c_lflag: 0,
            _c_line: 0,
            _c_cc: [0; 8],
        };
        unsafe {
            ioctl(0, TCGETA, &t);
        }

        let input = Input { termios: t };
        input.cbreak();
        input.nonblock();
        return input;
    }

    fn cbreak(&self) {
        let mut t = self.termios.clone();
        t.c_lflag &= !(ICANON|ECHO);
        unsafe {
            ioctl(0, TCSETAW, &t);
        }
    }

    fn nonblock(&self) {
        unsafe {
            let fl = fcntl(0, GETFL);
            fcntl(0, SETFL, fl|NONBLOCK);
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
            ioctl(0, TCSETAW, &self.termios);
        }
    }
}
