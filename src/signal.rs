const SIGINT: i32 = 2;

extern "C" {
    fn signal(signal: i32, handler: usize) -> usize;
}

pub fn on_ctrlc(handler: fn(i32) -> ()) {
    unsafe {
        signal(SIGINT, handler as usize);
    }
}
