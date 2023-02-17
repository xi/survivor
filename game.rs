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
    size: f32,
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

    let mut time0 = time::Instant::now();

    while RUNNING.load(Ordering::Relaxed) {
        let time1 = time::Instant::now();

        clear(&mut screen);
        circle(&mut screen, width as f32 / 2.0, height as f32 / 2.0, 15.0, [0x00, 0x00, 0xff]);

        for i in 0..monsters.len() {
            let monster = &monsters[i];

            let dxp = player_x - monster.x;
            let dyp = player_y - monster.y;
            let dp = (dxp * dxp + dyp * dyp).sqrt();

            let mut dx = dxp / dp;
            let mut dy = dyp / dp;

            for j in 0..monsters.len() {
                if i != j {
                    let other = &monsters[j];

                    let dxm = other.x - monster.x;
                    let dym = other.y - monster.y;
                    let dm = (dxm * dxm + dym * dym).sqrt();

                    if dm < monster.size + other.size {
                        dx -= dxm / dm;
                        dy -= dym / dm;
                    }
                }
            }

            let mut monster = &mut monsters[i];
            let d = (dx * dx + dy * dy).sqrt();
            monster.x += dx / d * monster.speed * (time1 - time0).as_secs_f32();
            monster.y += dy / d * monster.speed * (time1 - time0).as_secs_f32();
        }
        monsters.sort_unstable_by_key(|m| m.y as i32);
        for monster in monsters.iter() {
            let sx = monster.x - player_x + width as f32 / 2.0;
            let sy = monster.y - player_y + height as f32 / 2.0;
            circle(&mut screen, sx, sy, monster.size, [0xff, 0x00, 0x00]);
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
                speed: 18.0,
                size: 10.0,
            });
        }

        screen.render();

        let time2 = time::Instant::now();
        if TICK > time2 - time1 {
            thread::sleep(TICK - (time2 - time1));
        }
        let time3 = time::Instant::now();
        print!("{:?}", 1.0 / (time3 - time1).as_secs_f64());
        time0 = time1;
    }
}

