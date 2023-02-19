use crate::sprites;
use crate::term::Screen;

pub struct Window {
    pub height: usize,
    pub width: usize,
    pub dx: usize,
    pub dy: usize,
}

impl Window {
    pub fn set(&self, screen: &mut Screen, x: usize, y: usize, color: [u8; 3]) {
        screen.set(x + self.dx, y + self.dy, color);
    }

    pub fn fill(&self, screen: &mut Screen, color: [u8; 3]) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.set(screen, x, y, color);
            }
        }
    }

    pub fn sprite(
        &self,
        screen: &mut Screen,
        cx: f32,
        cy: f32,
        sprite: &sprites::Sprite,
        invert: bool,
    ) {
        let x0 = screen.convert_x(cx) - sprites::WIDTH as i64 / 2;
        let y0 = screen.convert_y(cy) + sprites::WIDTH as i64 / 2 - sprites::HEIGHT as i64;

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
                    self.set(screen, x as usize, y as usize, c);
                }
            }
        }
    }

    pub fn circle(&self, screen: &mut Screen, cx: f32, cy: f32, r: f32, color: [u8; 3]) {
        let r2 = r * r;

        let y0 = screen.convert_y(cy - r).max(0).min(self.height as i64 - 1) as usize;
        let x0 = screen.convert_x(cx - r).max(0).min(self.width as i64 - 1) as usize;

        let y1 = screen.convert_y(cy + r).max(0).min(self.height as i64 - 1) as usize;
        let x1 = screen.convert_x(cx + r).max(0).min(self.width as i64) as usize;

        for y in y0..=y1 {
            let dy = screen.iconvert_y(y) - cy;
            let y2 = dy * dy;
            for x in x0..=x1 {
                let dx = screen.iconvert_x(x) - cx;
                if dx * dx + y2 <= r2 {
                    self.set(screen, x, y, color);
                }
            }
        }
    }
}
