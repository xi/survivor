#![allow(non_camel_case_types)]

pub type c_int = i32;
pub type c_uint = u32;
pub type c_ulong = u64;
pub type c_void = u8;
pub type c_uchar = u8;
pub type size_t = usize;
pub type ssize_t = isize;
pub type Ioctl = c_ulong;
pub type tcflag_t = c_uint;

pub const SIGINT: c_int = 2;
pub const TIOCGWINSZ: Ioctl = 0x5413;
pub const ICANON: tcflag_t = 0x00000002;
pub const ECHO: tcflag_t = 0x00000008;
pub const TCSADRAIN: c_int = 1;
pub const VTIME: usize = 5;
pub const VMIN: usize = 6;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: c_uchar,
    pub c_cc: [c_uchar; 32],
    pub c_ispeed: c_uint,
    pub c_ospeed: c_uint,
}

extern "C" {
    pub fn ioctl(fd: c_int, req: c_ulong, ...) -> c_int;
    pub fn tcgetattr(fd: c_int, termios: *mut termios) -> c_int;
    pub fn tcsetattr(fd: c_int, optional_actions: c_int, termios: *const termios) -> c_int;
    pub fn signal(signum: c_int, handler: size_t) -> size_t;
    pub fn getrandom(buf: *mut c_void, buflen: size_t, flags: c_uint) -> ssize_t;
}
