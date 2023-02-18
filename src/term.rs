extern crate libc;

const Y_FACTOR: f32 = 1.4;

fn get_terminal_size() -> (usize, usize) {
    let w = [0u16; 4];
    unsafe {
        libc::ioctl(1, libc::TIOCGWINSZ, &w);
    }
    return (w[1] as usize, w[0] as usize);
}

fn toggle_cursor(show: bool) {
    if show {
        print!("\x1b[?25h");
    } else {
        print!("\x1b[?25l");
    }
}

fn set_bg(color: [u8; 3]) {
    print!("\x1b[48;2;{};{};{}m", color[0], color[1], color[2]);
}

fn set_fg(color: [u8; 3]) {
    print!("\x1b[38;2;{};{};{}m", color[0], color[1], color[2]);
}

fn block6(block: u32) -> char {
    if block == 0b000000 {
        return ' '
    } else if block < 0b010101 {
        return char::from_u32(0x1FB00 + block - 1).unwrap();
    } else if block == 0b010101 {
        return '\u{258C}';
    } else if block < 0b101010 {
        return char::from_u32(0x1FB00 + block - 2).unwrap();
    } else if block == 0b101010 {
        return '\u{2590}';
    } else if block < 0b111111 {
        return char::from_u32(0x1FB00 + block - 3).unwrap();
    } else {
        return '\u{2588}';
    }
}

fn color_avg(colors: &Vec<[u8; 3]>) -> [u8; 3] {
    let n = colors.len() as u16;
    if n == 0 {
        return [0, 0, 0];
    }
    return [
        (colors.iter().map(|c| c[0] as u16).sum::<u16>() / n) as u8,
        (colors.iter().map(|c| c[1] as u16).sum::<u16>() / n) as u8,
        (colors.iter().map(|c| c[2] as u16).sum::<u16>() / n) as u8,
    ];
}

fn get_block(colors: [[u8; 3]; 6]) -> (u32, [u8; 3], [u8; 3]) {
    let mut block = 0b000000;
    let mut lights = vec![];
    let mut darks = vec![];

    let lightness: Vec<u8> = colors.iter().map(|c| (c[0] >> 3) + (c[1] >> 1) + (c[2] >> 5)).collect();
    let mean: u8 = lightness.iter().map(|l| l / 6).sum();

    for i in 0..6 {
        if lightness[i] > mean {
            block |= 1 << i;
            lights.push(colors[i]);
        } else {
            darks.push(colors[i]);
        }
    }

    return (block, color_avg(&darks), color_avg(&lights));
}

pub struct Screen {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Vec<[u8; 3]>>,
}

impl Drop for Screen {
    fn drop(&mut self) {
        toggle_cursor(true);
    }
}

impl Screen {
    pub fn new() -> Screen {
        let (w, h) = get_terminal_size();
        let width = w * 2;
        let height = (h - 1) * 3;
        return Screen {
            width: width,
            height: height,
            pixels: vec![vec![[0, 0, 0]; width]; height],
        }
    }

    pub fn set(&mut self, x: usize, y: usize, color: [u8; 3]) {
        self.pixels[y][x] = color;
    }

    pub fn render(&mut self) {
        let mut prev_bg = [0x00, 0x00, 0x00];
        let mut prev_fg = [0xff, 0xff, 0xff];

        print!("\x1b[H");
        toggle_cursor(false);
        for y in 0..(self.height / 3) {
            for x in 0..(self.width / 2) {
                let (block, bg, fg) = get_block([
                    self.pixels[y * 3 + 0][x * 2 + 0],
                    self.pixels[y * 3 + 0][x * 2 + 1],
                    self.pixels[y * 3 + 1][x * 2 + 0],
                    self.pixels[y * 3 + 1][x * 2 + 1],
                    self.pixels[y * 3 + 2][x * 2 + 0],
                    self.pixels[y * 3 + 2][x * 2 + 1],
                ]);
                if bg != prev_bg {
                    set_bg(bg);
                    prev_bg = bg;
                }
                if fg != prev_fg {
                    set_fg(fg);
                    prev_fg = fg;
                }
                print!("{}", block6(block));
            }
            if y != self.height / 3 - 1 {
                print!("\n");
            }
        }

        println!("\x1b[0m");
    }

    pub fn convert_x(&self, x: f32) -> i64 {
        return x as i64;
    }

    pub fn convert_y(&self, y: f32) -> i64 {
        return (y / Y_FACTOR) as i64;
    }

    pub fn iconvert_x(&self, x: usize) -> f32 {
        return x as f32;
    }

    pub fn iconvert_y(&self, y: usize) -> f32 {
        return y as f32 * Y_FACTOR;
    }
}
