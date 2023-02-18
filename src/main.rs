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
        if y < 0 {
            continue;
        }
        if y >= screen.height as i64 {
            break;
        }
        for dx in 0..sprites::WIDTH {
            let x = x0 + dx as i64;
            if x < 0 {
                continue;
            }
            if x >= screen.width as i64 {
                break;
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

    let y0 = screen.convert_y(cy - r).max(0).min(screen.height as i64 - 1) as usize;
    let x0 = screen.convert_x(cx - r).max(0).min(screen.width as i64 - 1) as usize;

    let y1 = screen.convert_y(cy + r).max(0).min(screen.height as i64 - 1) as usize;
    let x1 = screen.convert_x(cx + r).max(0).min(screen.width as i64 - 1) as usize;

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

struct Player {
    pub x: f32,
    pub y: f32,
    pub dir: Dir,
    pub face: Dir,
    pub speed: f32,
    pub size: f32,
    pub health: f32,
    pub health_max: f32,
    pub health_recover: f32,
    pub power: f32,
    pub damage_radius: f32,
    pub diamond_radius: f32,
    pub xp: usize,
    pub last_level: usize,
    pub next_level: usize,

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

    let mut player = Player {
        x: 0.0,
        y: 0.0,
        dir: Dir::Stop,
        face: Dir::Right,
        speed: 30.0,
        size: 9.0,
        health: 50.0,
        health_max: 50.0,
        health_recover: 0.0,
        power: 10.0,
        damage_radius: 30.0,
        diamond_radius: 15.0,
        xp: 0,
        last_level: 0,
        next_level: 10,
    };

    unsafe {
        libc::signal(libc::SIGINT, quit as usize);
    }

    let mut time0 = time::Instant::now();

    while RUNNING.load(Ordering::Relaxed) {
        let time1 = time::Instant::now();
        let dt = (time1 - time0).as_secs_f32();

        match input.getch() {
            Some(b'w' | b'A') => { player.dir = Dir::Up },
            Some(b'a' | b'D') => { player.dir = Dir::Left; player.face = Dir::Left },
            Some(b's' | b'B') => { player.dir = Dir::Down },
            Some(b'd' | b'C') => { player.dir = Dir::Right; player.face = Dir::Right },
            Some(b' ') => { player.dir = Dir::Stop },
            Some(b'q') => { quit(0) },
            _ => {},
        }

        // move
        match player.dir {
            Dir::Up => { player.y -= player.speed * dt },
            Dir::Right => { player.x += player.speed * dt },
            Dir::Down => { player.y += player.speed * dt },
            Dir::Left => { player.x -= player.speed * dt },
            Dir::Stop => {},
        }

        for i in 0..enemies.len() {
            let enemy = &enemies[i];

            let dxp = player.x - enemy.x;
            let dyp = player.y - enemy.y;
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

        // recover
        player.health = (player.health + player.health_recover * dt).min(player.health_max);

        // despawn
        enemies = enemies.into_iter().filter(|e| {
            (e.y - player.y).abs() < height
            && (e.x - player.x).abs() < width
        }).collect();

        // interact with enemies
        for enemy in enemies.iter_mut() {
            let dx = player.x - enemy.x;
            let dy = player.y - enemy.y;

            let size = enemy.t.size + player.size;
            if dx * dx + dy * 2.0 * dy * 2.0 < size * size {
                player.health -= enemy.t.power * dt;
            }

            if dx * dx + dy * dy < player.damage_radius * player.damage_radius {
                enemy.health -= player.power * dt;
            }
        }

        enemies = enemies.into_iter().filter(|enemy| {
            if enemy.health <= 0.0 {
                diamonds.push(Diamond {
                    x: enemy.x,
                    y: enemy.y,
                });
                return false;
            } else {
                return true;
            }
        }).collect();

        if player.health < 0.0 {
            println!("\nyou died (score: {})", player.xp);
            break;
        }

        // interact with diamonds
        diamonds = diamonds.into_iter().filter(|diamond| {
            let dx = player.x - diamond.x;
            let dy = player.y - diamond.y;
            let d = (dx * dx + dy * dy).sqrt();
            if d < player.diamond_radius {
                player.xp += 1;
                return false;
            } else{
                return true;
            }
        }).collect();

        while player.xp >= player.next_level {
            player.last_level = player.next_level;
            player.next_level *= 2;

            match rng.gen_range(0, 7) {
                PERK_POWER => { player.power *= 1.1; },
                PERK_HEALTH => { player.health_max *= 1.1; },
                PERK_SPEED => { player.speed *= 1.1; },
                PERK_RADIUS => { player.damage_radius *= 1.1; },
                PERK_HEAL => { player.health = player.health_max; },
                PERK_RECOVER => { player.health_recover += 0.2 },
                PERK_ATTRACT => { player.diamond_radius *= 1.1; },
                _ => unreachable!(),
            }
        }

        // spawn
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
                spawn_x + player.x - width / 2.0,
                spawn_y + player.y - height / 2.0,
                i_enemy,
            ));
            i_enemy += 1;
        }

        // render
        clear(&mut screen);
        circle(&mut screen, width / 2.0, height / 2.0, player.damage_radius, [0x00, 0xff, 0x00]);

        for diamond in diamonds.iter() {
            let sx = diamond.x - player.x + width / 2.0;
            let sy = diamond.y - player.y + height / 2.0;
            sprite(&mut screen, sx, sy, &sprites::DIAMOND, false);
        }

        enemies.sort_unstable_by_key(|e| e.y as i32);
        let mut player_rendered = false;
        for enemy in enemies.iter() {
            if !player_rendered && enemy.y > player.y {
                sprite(&mut screen, width / 2.0, height / 2.0, &sprites::HERO, player.face == Dir::Left);
                player_rendered = true;
            }

            let sx = enemy.x - player.x + width / 2.0;
            let sy = enemy.y - player.y + height / 2.0;
            sprite(&mut screen, sx, sy, enemy.t.sprite, enemy.x > player.x);
        }
        if !player_rendered {
            sprite(&mut screen, width / 2.0, height / 2.0, &sprites::HERO, player.face == Dir::Left);
        }

        bar(&mut screen, 0, (player.xp - player.last_level) as f32 / (player.next_level - player.last_level) as f32, [0x00, 0x00, 0xff]);
        let h = screen.height;
        bar(&mut screen, h - 3, player.health / player.health_max, [0xff, 0x00, 0x00]);

        screen.render();

        // sleep
        let time2 = time::Instant::now();
        if TICK > time2 - time1 {
            thread::sleep(TICK - (time2 - time1));
        }
        let time3 = time::Instant::now();
        print!("{:?}", 1.0 / (time3 - time1).as_secs_f64());
        time0 = time1;
    }
}
