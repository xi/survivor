use crate::game::Dir;
use crate::sprites;
use crate::term::Screen;

const ASPECT_RATIO: f32 = 1.4;

pub fn convert_x(x: f32) -> i64 {
    return x as i64;
}

pub fn convert_y(y: f32) -> i64 {
    return (y / ASPECT_RATIO) as i64;
}

pub fn iconvert_x(x: usize) -> f32 {
    return x as f32;
}

pub fn iconvert_y(y: usize) -> f32 {
    return y as f32 * ASPECT_RATIO;
}

pub struct Window<'a> {
    pub height: usize,
    pub width: usize,
    pub dx: usize,
    pub dy: usize,
    pub screen: &'a mut Screen,
}

impl Window<'_> {
    pub fn set(&mut self, x: usize, y: usize, color: [u8; 3]) {
        self.screen.set(x + self.dx, y + self.dy, color);
    }

    pub fn fill(&mut self, color: [u8; 3]) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.set(x, y, color);
            }
        }
    }

    pub fn _sprite(&mut self, cx: f32, cy: f32, sprite: &sprites::Sprite, invert: bool) {
        let x0 = convert_x(cx) - sprites::WIDTH as i64 / 2;
        let y0 = convert_y(cy) - (sprites::HEIGHT as i64 - sprites::WIDTH as i64 / 2);

        for dy in 0..sprites::HEIGHT {
            let y = y0 + dy as i64;
            if y < 0 {
                continue;
            }
            if y >= self.height as i64 {
                break;
            }
            for dx in 0..sprites::WIDTH {
                let x = x0 + dx as i64;
                if x < 0 {
                    continue;
                }
                if x >= self.width as i64 {
                    break;
                }
                let cx = if invert { sprites::WIDTH - dx - 1 } else { dx };
                let c = sprite[dy][cx];
                if c != sprite[0][0] {
                    self.set(x as usize, y as usize, c);
                }
            }
        }
    }

    pub fn _sprite_tilted(&mut self, cx: f32, cy: f32, sprite: &sprites::Sprite, invert: bool) {
        let x0 = convert_x(cx) - sprites::WIDTH as i64 / 2;
        let y0 = convert_y(cy) - sprites::WIDTH as i64 / 2;

        for dy in 0..sprites::WIDTH {
            let y = y0 + dy as i64;
            if y < 0 {
                continue;
            }
            if y >= self.height as i64 {
                break;
            }
            for dx in 0..sprites::HEIGHT {
                let x = x0 + dx as i64;
                if x < 0 {
                    continue;
                }
                if x >= self.width as i64 {
                    break;
                }
                let cy = if invert { sprites::WIDTH - dy - 1 } else { dy };
                let c = sprite[dx][cy];
                if c != sprite[0][0] {
                    self.set(x as usize, y as usize, c);
                }
            }
        }
    }

    pub fn sprite(&mut self, cx: f32, cy: f32, sprite: &sprites::Sprite, face: Dir) {
        match face {
            Dir::Up => self._sprite_tilted(cx, cy, sprite, true),
            Dir::Right => self._sprite(cx, cy, sprite, false),
            Dir::Down => self._sprite_tilted(cx, cy, sprite, false),
            Dir::Left => self._sprite(cx, cy, sprite, true),
        }
    }

    pub fn circle(&mut self, cx: f32, cy: f32, r: f32, color: [u8; 3]) {
        let r2 = r * r;

        let y0 = convert_y(cy - r).max(0).min(self.height as i64 - 1) as usize;
        let x0 = convert_x(cx - r).max(0).min(self.width as i64 - 1) as usize;

        let y1 = convert_y(cy + r).max(0).min(self.height as i64 - 1) as usize;
        let x1 = convert_x(cx + r).max(0).min(self.width as i64) as usize;

        for y in y0..=y1 {
            let dy = iconvert_y(y) - cy;
            let y2 = dy * dy;
            for x in x0..=x1 {
                let dx = iconvert_x(x) - cx;
                if dx * dx + y2 <= r2 {
                    self.set(x, y, color);
                }
            }
        }
    }
}
