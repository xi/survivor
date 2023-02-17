#![allow(non_camel_case_types)]

pub type c_int = i32;
pub type c_uint = u32;
pub type c_ulong = u64;
pub type c_void = u8;
pub type size_t = usize;
pub type ssize_t = isize;
pub type Ioctl = c_ulong;
pub type tcflag_t = c_uint;

pub const SIGINT: c_int = 2;
pub const TIOCGWINSZ: Ioctl = 0x5413;
pub const TCGETA: Ioctl = 0x5405;
pub const TCSETAW: Ioctl = 0x5407;
pub const ICANON: tcflag_t = 0x00000002;
pub const ECHO: tcflag_t = 0x00000008;
pub const F_GETFL: c_int = 3;
pub const F_SETFL: c_int = 4;
pub const O_NONBLOCK: c_int = 2048;

#[derive(Clone)]
pub struct Termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: u8,
    pub c_cc: [u8; 8],
}

impl Termios {
    pub fn new() -> Termios {
        return Termios {
            c_iflag: 0,
            c_oflag: 0,
            c_cflag: 0,
            c_lflag: 0,
            c_line: 0,
            c_cc: [0; 8],
        };
    }
}

extern "C" {
    pub fn ioctl(fd: c_int, req: c_ulong, ...) -> c_int;
    pub fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;
    pub fn signal(signum: c_int, handler: size_t) -> size_t;
    pub fn getrandom(buf: *mut c_void, buflen: size_t, flags: c_uint) -> ssize_t;
}
