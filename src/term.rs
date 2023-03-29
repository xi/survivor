extern crate libc;

fn get_terminal_size() -> (usize, usize) {
    let w: libc::winsize;
    unsafe {
        w = std::mem::zeroed();
        libc::ioctl(1, libc::TIOCGWINSZ, &w);
    }
    return (w.ws_col as usize, w.ws_row as usize);
}

mod ti {
    pub fn cnorm() {
        print!("\x1b[?25h");
    }
    pub fn civis() {
        print!("\x1b[?25l");
    }
    pub fn cup(x: usize, y: usize) {
        print!("\x1b[{};{}H", x + 1, y + 1);
    }
    pub fn ed() {
        print!("\x1b[2J");
    }
    pub fn setab(color: [u8; 3]) {
        print!("\x1b[48;2;{};{};{}m", color[0], color[1], color[2]);
    }
    pub fn setaf(color: [u8; 3]) {
        print!("\x1b[38;2;{};{};{}m", color[0], color[1], color[2]);
    }
    pub fn sgr0() {
        println!("\x1b[0m");
    }
}

fn sextant(block: u32) -> char {
    if block == 0b000000 {
        return ' ';
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

    let lightness: Vec<u8> = colors
        .iter()
        .map(|c| (c[0] >> 3) + (c[1] >> 1) + (c[2] >> 5))
        .collect();
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

impl Screen {
    pub fn new() -> Self {
        let mut screen = Self {
            width: 0,
            height: 0,
            pixels: vec![],
        };
        screen.init();
        return screen;
    }

    pub fn init(&mut self) {
        self.resize();
        ti::civis();
        ti::ed();
    }

    pub fn restore(&self) {
        ti::cnorm();
        ti::sgr0();
    }

    pub fn resize(&mut self) {
        let (w, h) = get_terminal_size();
        self.width = w * 2;
        self.height = (h - 1) * 3;
        self.pixels = vec![vec![[0, 0, 0]; self.width]; self.height];
    }

    pub fn set(&mut self, x: usize, y: usize, color: [u8; 3]) {
        self.pixels[y][x] = color;
    }

    pub fn render(&mut self) {
        let mut prev_bg = [0x00, 0x00, 0x00];
        let mut prev_fg = [0xff, 0xff, 0xff];

        ti::cup(0, 0);
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
                    ti::setab(bg);
                    prev_bg = bg;
                }
                if fg != prev_fg {
                    ti::setaf(fg);
                    prev_fg = fg;
                }
                print!("{}", sextant(block));
            }
            if y != self.height / 3 - 1 {
                print!("\n");
            }
        }

        ti::sgr0();
    }
}

impl Drop for Screen {
    fn drop(&mut self) {
        self.restore();
    }
}
