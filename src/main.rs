mod term;
mod input;
mod random;
mod sprites;

extern crate libc;

use std::{thread, time};
use std::sync::atomic::{AtomicBool, Ordering};

const TICK: time::Duration = time::Duration::from_millis(30);

static RUNNING: AtomicBool = AtomicBool::new(true);

#[derive(PartialEq)]
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
    fill(screen, [0x33, 0x88, 0x22]);
}

fn sprite(screen: &mut term::Screen, cx: f32, cy: f32, sprite: sprites::Sprite, invert: bool) {
    let x0 = screen.convert_x(cx) - sprites::WIDTH as i64 / 2;
    let y0 = screen.convert_y(cy) + sprites::WIDTH as i64 / 2 - sprites::HEIGHT as i64;

    for dy in 0..sprites::HEIGHT {
        let y = y0 + dy as i64;
        if y < 0 || y >= screen.height as i64 {
            continue;
        }
        for dx in 0..sprites::WIDTH {
            let x = x0 + dx as i64;
            if x < 0 || x >= screen.width as i64 {
                continue;
            }
            let cx = if invert { sprites::WIDTH - dx - 1 } else { dx };
            let c = sprite[dy][cx];
            if c != sprite[0][0] {
                screen.set(x as usize, y as usize, c);
            }
        }
    }
}

fn bar(screen: &mut term::Screen, y: usize, value: f32, color: [u8; 3]) {
    let black = [0x00, 0x00, 0x00];

    for x in 0..screen.width {
        let fx = x as f32 / screen.width as f32;
        let c = if fx <= value { color } else { black };
        for dy in 0..3 {
            screen.set(x, y + dy, c);
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
    health: f32,
    attack: f32,
}

fn main() {
    let input = input::Input::new();
    let mut screen = term::Screen::new();
    let mut rng = random::Rng::new();
    let width = screen.iconvert_x(screen.width);
    let height = screen.iconvert_y(screen.height);
    let sprite_width = screen.iconvert_x(sprites::WIDTH);
    let sprite_height = screen.iconvert_y(sprites::HEIGHT);
    let mut monsters: Vec<Monster> = vec![];

    let mut player_x = 0.0;
    let mut player_y = 0.0;
    let mut player_dir = Dir::Stop;
    let mut player_face = Dir::Right;
    let player_speed = 30.0;
    let mut player_health = 50.0;
    let player_health_max = 50.0;
    let player_attack = 20.0;
    let player_attack_radius = 40.0;
    let player_exp = 0.3;

    unsafe {
        libc::signal(libc::SIGINT, quit as usize);
    }

    let mut time0 = time::Instant::now();

    while RUNNING.load(Ordering::Relaxed) {
        let time1 = time::Instant::now();
        let dt = (time1 - time0).as_secs_f32();

        clear(&mut screen);
        sprite(&mut screen, width / 2.0, height / 2.0, sprites::HERO, player_face == Dir::Left);

        match input.getch() {
            Some(b'w') => { player_dir = Dir::Up },
            Some(b'A') => { player_dir = Dir::Up },
            Some(b'a') => { player_dir = Dir::Left; player_face = Dir::Left },
            Some(b'D') => { player_dir = Dir::Left; player_face = Dir::Left },
            Some(b's') => { player_dir = Dir::Down },
            Some(b'B') => { player_dir = Dir::Down },
            Some(b'd') => { player_dir = Dir::Right; player_face = Dir::Right },
            Some(b'C') => { player_dir = Dir::Right; player_face = Dir::Right },
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

        monsters = monsters.into_iter().filter(|m| {
            (m.y - player_y).abs() < height
            && (m.x - player_x).abs() < width
        }).collect();

        for monster in monsters.iter_mut() {
            let dx = player_x - monster.x;
            let dy = player_y - monster.y;
            let d = (dx * dx + dy * dy).sqrt();

            if d < monster.size {
                player_health -= monster.attack * dt;
            }

            if d < player_attack_radius {
                monster.health -= player_attack * dt;
            }
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
                dx = dx * (1.0 - inertia) + monster.dx * inertia;
                dy = dy * (1.0 - inertia) + monster.dy * inertia;
            }

            let mut monster = &mut monsters[i];
            monster.x += dx * monster.speed * dt;
            monster.y += dy * monster.speed * dt;
            monster.dx = dx;
            monster.dy = dy;
        }

        if player_health < 0.0 {
            println!("\nyou died");
            break;
        }

        for monster in monsters.iter_mut() {
            if monster.health < 0.0 {
                monster.speed = 0.0;
            }
        }

        monsters.sort_unstable_by_key(|m| m.y as i32);
        for monster in monsters.iter() {
            let sx = monster.x - player_x + width / 2.0;
            let sy = monster.y - player_y + height / 2.0;
            sprite(&mut screen, sx, sy, sprites::SKELETON, monster.x > player_x);
        }

        if rng.gen_f32() < dt * 10.0 {
            let (spawn_x, spawn_y) = match rng.gen_range(0, 4) {
                0 => (
                    rng.gen_f32() * width,
                    -sprite_height,
                ),
                1 => (
                    width + sprite_width,
                    rng.gen_f32() * height,
                ),
                2 => (
                    rng.gen_f32() * width,
                    height + sprite_height,
                ),
                3 => (
                    -sprite_width,
                    rng.gen_f32() * height,
                ),
                _ => unreachable!(),
            };

            monsters.push(Monster {
                x: spawn_x + player_x - width / 2.0,
                y: spawn_y + player_y - height / 2.0,
                dx: 0.0,
                dy: 0.0,
                inertia: 0.1,
                speed: 10.0,
                size: 10.0,
                health: 10.0,
                attack: 5.0,
            });
        }

        bar(&mut screen, 0, player_exp, [0x00, 0x00, 0xff]);
        let h = screen.height;
        bar(&mut screen, h - 3, player_health / player_health_max, [0xff, 0x00, 0x00]);

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

