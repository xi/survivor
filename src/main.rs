mod term;
mod input;
mod random;

extern crate libc;

use std::{thread, time};
use std::sync::atomic::{AtomicBool, Ordering};

const TICK: time::Duration = time::Duration::from_millis(30);

static RUNNING: AtomicBool = AtomicBool::new(true);

enum Dir { Up, Right, Down, Left, Stop }

fn quit(_sig: i32) {
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

    let y0 = screen.convert_y(cy - r).unwrap_or(0);
    let x0 = screen.convert_x(cx - r).unwrap_or(0);

    let y1 = screen.convert_y(cy + r).unwrap_or(screen.height - 1) + 1;
    let x1 = screen.convert_x(cx + r).unwrap_or(screen.width - 1) + 1;

    for y in y0..y1 {
        let dy = screen.iconvert_y(y) - cy;
        let y2 = dy * dy;
        for x in x0..x1 {
            let dx = screen.iconvert_x(x) - cx;
            if dx * dx + y2 <= r2 {
                screen.set(x, y, color);
            }
        }
    }
}

struct Monster {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    speed: f32,
    inertia: f32,
    size: f32,
}

fn main() {
    let input = input::Input::new();
    let mut screen = term::Screen::new();
    let mut rng = random::Rng::new();
    let width = screen.iconvert_x(screen.width);
    let height = screen.iconvert_y(screen.height);
    let mut monsters: Vec<Monster> = vec![];

    let mut player_x = 0.0;
    let mut player_y = 0.0;
    let mut player_dir = Dir::Up;
    let player_speed = 30.0;

    unsafe {
        libc::signal(libc::SIGINT, quit as usize);
    }

    let mut time0 = time::Instant::now();

    while RUNNING.load(Ordering::Relaxed) {
        let time1 = time::Instant::now();
        let dt = (time1 - time0).as_secs_f32();

        clear(&mut screen);
        circle(&mut screen, width / 2.0, height / 2.0, 15.0, [0x00, 0x00, 0xff]);

        match input.getch() {
            Some(b'w') => { player_dir = Dir::Up },
            Some(b'A') => { player_dir = Dir::Up },
            Some(b'a') => { player_dir = Dir::Left },
            Some(b'D') => { player_dir = Dir::Left },
            Some(b's') => { player_dir = Dir::Down },
            Some(b'B') => { player_dir = Dir::Down },
            Some(b'd') => { player_dir = Dir::Right },
            Some(b'C') => { player_dir = Dir::Right },
            Some(b' ') => { player_dir = Dir::Stop },
            Some(b'q') => { quit(0) },
            _ => {},
        }

        match player_dir {
            Dir::Up => { player_y -= player_speed * dt },
            Dir::Right => { player_x += player_speed * dt },
            Dir::Down => { player_y += player_speed * dt },
            Dir::Left => { player_x -= player_speed * dt },
            Dir::Stop => {},
        }

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

            let d = (dx * dx + dy * dy).sqrt();
            dx /= d;
            dy /= d;
            if dt < 0.000001 {
                dx = monster.dx;
                dy = monster.dy;
            } else {
                let inertia = monster.inertia.powf(dt);
                // println!("{}", inertia);
                dx = dx * (1.0 - inertia) + monster.dx * inertia;
                dy = dy * (1.0 - inertia) + monster.dy * inertia;
            }

            let mut monster = &mut monsters[i];
            monster.x += dx * monster.speed * dt;
            monster.y += dy * monster.speed * dt;
            monster.dx = dx;
            monster.dy = dy;
        }
        monsters.sort_unstable_by_key(|m| m.y as i32);
        for monster in monsters.iter() {
            let sx = monster.x - player_x + width / 2.0;
            let sy = monster.y - player_y + height / 2.0;
            circle(&mut screen, sx, sy, monster.size, [0xff, 0x00, 0x00]);
        }

        if rng.gen_f32() < dt * 10.0 {
            let inertia = rng.gen_f32();
            let size = 8.0 + inertia * 8.0;

            let (spawn_x, spawn_y) = match rng.gen_range(0, 4) {
                0 => (
                    rng.gen_f32() * width,
                    -size,
                ),
                1 => (
                    width + size,
                    rng.gen_f32() * height,
                ),
                2 => (
                    rng.gen_f32() * width,
                    height + size,
                ),
                3 => (
                    -size,
                    rng.gen_f32() * height,
                ),
                _ => unreachable!(),
            };

            monsters.push(Monster {
                x: spawn_x + player_x - width / 2.0,
                y: spawn_y + player_y - height / 2.0,
                dx: 0.0,
                dy: 0.0,
                inertia: inertia / 20.0,
                speed: 2.0 + rng.gen_f32() * 30.0,
                size: size,
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

