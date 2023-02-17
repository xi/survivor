extern crate libc;

pub struct Rng {
    state: u64,
}

impl Rng {
    pub fn new() -> Self {
        let mut bytes = [0; 8];
        unsafe {
            libc::getrandom(bytes.as_mut_ptr(), 8, 0x0001);
        }
        return Self { state: u64::from_ne_bytes(bytes) };
    }

    fn gen(&mut self) -> usize {
        // https://github.com/smol-rs/fastrand/blob/master/src/lib.rs
        let s = self.state.wrapping_add(0xA0761D6478BD642F);
        self.state = s;
        let t = u128::from(s) * u128::from(s ^ 0xE7037ED1A0B428DB);
        return (t as usize) ^ (t >> 64) as usize;
    }

    pub fn gen_range(&mut self, low: usize, high: usize) -> usize {
        return low + self.gen() % (high - low);
    }

    pub fn gen_f32(&mut self) -> f32 {
        // https://en.wikipedia.org/wiki/Single-precision_floating-point_format
        let u = self.gen() as u32;
        return f32::from_bits((127 << 23) | u >> 9) - 1.0;
    }
}
