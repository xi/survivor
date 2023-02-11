#[path = "term.rs"] mod term;
#[path = "signal.rs"] mod signal;
#[path = "random.rs"] mod random;

use std::{thread, time};
use std::sync::atomic::{AtomicBool, Ordering};

const TICK: time::Duration = time::Duration::from_millis(30);
const Y_FACTOR: f32 = 1.4;

static RUNNING: AtomicBool = AtomicBool::new(true);

fn on_ctrlc(_sig: i32) {
    RUNNING.fetch_and(false, Ordering::Relaxed);
}

fn fill(screen: &mut term::Screen, color: [u8; 3]) {
    for y in 0..screen.height {
        for x in 0..screen.width {
            screen.set(x, y, color);
        }
    }
}

fn clear(screen: &mut term::Screen) {
    fill(screen, [0, 0, 0]);
}

fn circle(screen: &mut term::Screen, cx: f32, cy: f32, r: f32, color: [u8; 3]) {
    let r2 = r * r;

    let y0 = (cy - r).max(0.0) as usize;
    let x0 = (cx - r).max(0.0) as usize;

    let y1 = (cy + r + 1.0).min(screen.height as f32) as usize;
    let x1 = (cx + r + 1.0).min(screen.width as f32) as usize;

    for y in y0..y1 {
        let dy = (y as f32 - cy) * Y_FACTOR;
        let y2 = dy * dy;
        for x in x0..x1 {
            let dx = x as f32 - cx;
            if dx * dx + y2 <= r2 {
                screen.set(x, y, color);
            }
        }
    }
}

struct Monster {
    x: f32,
    y: f32,
    speed: f32,
}

fn main() {
    let mut screen = term::Screen::new();
    let mut rng = random::Rng::new();
    let width = screen.width;
    let height = screen.height;
    let mut monsters: Vec<Monster> = vec![];

    let mut player_x = 0.0;
    let mut player_y = 0.0;

    signal::on_ctrlc(on_ctrlc);

    while RUNNING.load(Ordering::Relaxed) {
        clear(&mut screen);
        circle(&mut screen, width as f32 / 2.0, height as f32 / 2.0, 15.0, [0x00, 0x00, 0xff]);

        for monster in monsters.iter_mut() {
            let dx = player_x - monster.x;
            let dy = player_y - monster.y;
            let d = (dx * dx + dy * dy).sqrt();
            monster.x += dx / d * monster.speed / TICK.as_millis() as f32 * 1000.0;
            monster.y += dy / d * monster.speed / TICK.as_millis() as f32 * 1000.0;

            let sx = monster.x - player_x + width as f32 / 2.0;
            let sy = monster.y - player_y + height as f32 / 2.0;
            circle(&mut screen, sx, sy, 10.0, [0xff, 0x00, 0x00]);
        }

        if rng.gen_range(0, 3) == 0 {
            let (spawn_x, spawn_y) = match rng.gen_range(0, 4) {
                0 => (
                    rng.gen_range(0, width) as f32,
                    -10.0,
                ),
                1 => (
                    width as f32 + 10.0,
                    rng.gen_range(0, height) as f32,
                ),
                2 => (
                    rng.gen_range(0, width) as f32,
                    height as f32 + 10.0,
                ),
                3 => (
                    -10.0,
                    rng.gen_range(0, height) as f32,
                ),
                _ => unreachable!(),
            };

            monsters.push(Monster {
                x: spawn_x + player_x - width as f32 / 2.0,
                y: spawn_y + player_y - height as f32 / 2.0,
                speed: 0.02,
            });
        }

        screen.render();
        thread::sleep(TICK);
    }
}

