mod term;
mod input;
mod random;
mod enemies;

extern crate libc;
extern crate sprites;

use std::{thread, time};
use std::sync::atomic::{AtomicBool, Ordering};

const TICK: time::Duration = time::Duration::from_millis(33);

const PERK_POWER: usize = 0;
const PERK_HEALTH: usize = 1;
const PERK_SPEED: usize = 2;
const PERK_RADIUS: usize = 3;
const PERK_HEAL: usize = 4;
const PERK_RECOVER: usize = 5;
const PERK_ATTRACT: usize = 6;

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

fn sprite(screen: &mut term::Screen, cx: f32, cy: f32, sprite: &sprites::Sprite, invert: bool) {
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

fn circle(screen: &mut term::Screen, cx: f32, cy: f32, r: f32, color: [u8; 3]) {
    let r2 = r * r;

    let y0 = screen.convert_y(cy - r).max(0) as usize;
    let x0 = screen.convert_x(cx - r).max(0) as usize;

    let y1 = screen.convert_y(cy + r).min(screen.height as i64 - 1) as usize;
    let x1 = screen.convert_x(cx + r).min(screen.width as i64 - 1) as usize;

    for y in y0..=y1 {
        let dy = screen.iconvert_y(y) - cy;
        let y2 = dy * dy;
        for x in x0..=x1 {
            let dx = screen.iconvert_x(x) - cx;
            if dx * dx + y2 <= r2 {
                screen.set(x, y, color);
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

struct Diamond {
    pub x: f32,
    pub y: f32,
}

fn main() {
    let input = input::Input::new();
    let mut screen = term::Screen::new();
    let mut rng = random::Rng::new();
    let width = screen.iconvert_x(screen.width);
    let height = screen.iconvert_y(screen.height);
    let sprite_width = screen.iconvert_x(sprites::WIDTH);
    let sprite_height = screen.iconvert_y(sprites::HEIGHT);
    let mut enemies: Vec<enemies::Enemy> = vec![];
    let mut diamonds: Vec<Diamond> = vec![];
    let mut i_enemy = 0;

    let mut player_x = 0.0;
    let mut player_y = 0.0;
    let mut player_dir = Dir::Stop;
    let mut player_face = Dir::Right;
    let mut player_speed = 30.0;
    let player_size = 9.0;
    let mut player_health = 50.0;
    let mut player_health_max = 50.0;
    let mut player_health_recover = 0.0;
    let mut player_attack = 10.0;
    let mut player_attack_radius = 30.0;
    let mut player_xp = 0;
    let mut player_last_level = 0;
    let mut player_next_level = 10;
    let mut player_diamond_radius = 15.0;

    unsafe {
        libc::signal(libc::SIGINT, quit as usize);
    }

    let mut time0 = time::Instant::now();

    while RUNNING.load(Ordering::Relaxed) {
        let time1 = time::Instant::now();
        let dt = (time1 - time0).as_secs_f32();

        clear(&mut screen);
        circle(&mut screen, width / 2.0, height / 2.0, player_attack_radius, [0x00, 0xff, 0x00]);

        for diamond in diamonds.iter() {
            let sx = diamond.x - player_x + width / 2.0;
            let sy = diamond.y - player_y + height / 2.0;
            sprite(&mut screen, sx, sy, &sprites::DIAMOND, false);
        }

        sprite(&mut screen, width / 2.0, height / 2.0, &sprites::HERO, player_face == Dir::Left);

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

        enemies = enemies.into_iter().filter(|e| {
            (e.y - player_y).abs() < height
            && (e.x - player_x).abs() < width
        }).collect();
        while player_xp >= player_next_level {
            player_last_level = player_next_level;
            player_next_level *= 2;

            match rng.gen_range(0, 7) {
                PERK_POWER => { player_attack *= 1.1; },
                PERK_HEALTH => { player_health_max *= 1.1; },
                PERK_SPEED => { player_speed *= 1.1; },
                PERK_RADIUS => { player_attack_radius *= 1.1; },
                PERK_HEAL => { player_health = player_health_max; },
                PERK_RECOVER => { player_health_recover += 0.2 },
                PERK_ATTRACT => { player_diamond_radius *= 1.1; },
                _ => unreachable!(),
            }
        }

        for enemy in enemies.iter_mut() {
            let dx = player_x - enemy.x;
            let dy = player_y - enemy.y;
            let d = (dx * dx + dy * dy).sqrt();

            if d < enemy.t.size + player_size {
                player_health -= enemy.t.power * dt;
            }

            if d < player_attack_radius {
                enemy.health -= player_attack * dt;
            }
        }

        player_health = (player_health + player_health_recover * dt).min(player_health_max);

        diamonds = diamonds.into_iter().filter(|diamond| {
            let dx = player_x - diamond.x;
            let dy = player_y - diamond.y;
            let d = (dx * dx + dy * dy).sqrt();
            if d < player_diamond_radius {
                player_xp += 1;
                return false;
            } else{
                return true;
            }
        }).collect();

        for i in 0..enemies.len() {
            let enemy = &enemies[i];

            let dxp = player_x - enemy.x;
            let dyp = player_y - enemy.y;
            let dp = (dxp * dxp + dyp * dyp).sqrt();

            let mut dx = dxp / dp;
            let mut dy = dyp / dp;

            for j in 0..enemies.len() {
                if i != j {
                    let other = &enemies[j];

                    let dxm = other.x - enemy.x;
                    let dym = other.y - enemy.y;
                    let dm = (dxm * dxm + dym * dym).sqrt();

                    if dm < enemy.t.size + other.t.size {
                        dx -= dxm / dm;
                        dy -= dym / dm;
                    }
                }
            }

            let d = (dx * dx + dy * dy).sqrt();
            dx /= d;
            dy /= d;

            let mut enemy = &mut enemies[i];
            enemy.x += dx * enemy.t.speed * dt;
            enemy.y += dy * enemy.t.speed * dt;
        }

        if player_health < 0.0 {
            println!("\nyou died (score: {})", player_xp);
            break;
        }

        for enemy in enemies.iter() {
            if enemy.health < 0.0 {
                diamonds.push(Diamond {
                    x: enemy.x,
                    y: enemy.y,
                });
            }
        }
        enemies = enemies.into_iter().filter(|e| e.health > 0.0).collect();

        enemies.sort_unstable_by_key(|m| m.y as i32);
        for enemy in enemies.iter() {
            let sx = enemy.x - player_x + width / 2.0;
            let sy = enemy.y - player_y + height / 2.0;
            sprite(&mut screen, sx, sy, enemy.t.sprite, enemy.x > player_x);
        }

        if rng.gen_f32() < dt * 2.0 {
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

            enemies.push(enemies::get_enemy(
                spawn_x + player_x - width / 2.0,
                spawn_y + player_y - height / 2.0,
                i_enemy,
            ));
            i_enemy += 1;
        }

        bar(&mut screen, 0, (player_xp - player_last_level) as f32 / (player_next_level - player_last_level) as f32, [0x00, 0x00, 0xff]);
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
