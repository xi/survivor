extern crate libc;

pub fn on_ctrlc(handler: fn(i32) -> ()) {
    unsafe {
        libc::signal(libc::SIGINT, handler as usize);
    }
}
