use crate::enemies;
use crate::random;
use crate::sprites;
use crate::term;
use crate::win;

const MAX_ENEMIES: usize = 100;

const PERK_POWER: usize = 0;
const PERK_HEALTH: usize = 1;
const PERK_SPEED: usize = 2;
const PERK_RADIUS: usize = 3;
const PERK_HEAL: usize = 4;
const PERK_RECOVER: usize = 5;
const PERK_ATTRACT: usize = 6;

#[derive(PartialEq)]
pub enum Dir {
    Up,
    Right,
    Down,
    Left,
    Stop,
}

pub struct Diamond {
    pub x: f32,
    pub y: f32,
}

pub struct Player {
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

impl Player {
    pub fn new() -> Self {
        return Self {
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
    }
}

pub struct Game {
    pub player: Player,
    pub diamonds: Vec<Diamond>,
    pub enemies: Vec<enemies::Enemy>,
    pub i_enemy: usize,
    pub win: win::Window,
    rng: random::Rng,
}

impl Game {
    pub fn new(screen: &term::Screen) -> Self {
        return Self {
            enemies: vec![],
            diamonds: vec![],
            i_enemy: 0,
            player: Player::new(),
            win: win::Window {
                width: screen.width,
                height: screen.height - 6,
                dx: 0,
                dy: 3,
            },
            rng: random::Rng::new(),
        };
    }

    pub fn step(&mut self, dt: f32) {
        let height = win::iconvert_y(self.win.height);
        let width = win::iconvert_x(self.win.width);
        let sprite_height = win::iconvert_y(sprites::HEIGHT);
        let sprite_width = win::iconvert_x(sprites::WIDTH);

        // move
        match self.player.dir {
            Dir::Up => self.player.y -= self.player.speed * dt,
            Dir::Right => self.player.x += self.player.speed * dt,
            Dir::Down => self.player.y += self.player.speed * dt,
            Dir::Left => self.player.x -= self.player.speed * dt,
            Dir::Stop => {}
        }

        for i in 0..self.enemies.len() {
            let enemy = &self.enemies[i];

            let dxp = self.player.x - enemy.x;
            let dyp = self.player.y - enemy.y;
            let dp = (dxp * dxp + dyp * dyp).sqrt();

            let mut dx = dxp / dp;
            let mut dy = dyp / dp;

            for j in 0..self.enemies.len() {
                if i != j {
                    let other = &self.enemies[j];

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

            let mut enemy = &mut self.enemies[i];
            enemy.x += dx * enemy.t.speed * dt;
            enemy.y += dy * enemy.t.speed * dt;
        }

        // recover
        self.player.health =
            (self.player.health + self.player.health_recover * dt).min(self.player.health_max);

        // despawn
        self.enemies = std::mem::take(&mut self.enemies)
            .into_iter()
            .filter(|e| (e.y - self.player.y).abs() < height && (e.x - self.player.x).abs() < width)
            .collect();

        // interact with enemies
        for enemy in self.enemies.iter_mut() {
            let dx = self.player.x - enemy.x;
            let dy = self.player.y - enemy.y;

            let size = enemy.t.size + self.player.size;
            if dx * dx + dy * 2.0 * dy * 2.0 < size * size {
                self.player.health -= enemy.t.power * dt;
            }

            if dx * dx + dy * dy < self.player.damage_radius * self.player.damage_radius {
                enemy.health -= self.player.power * dt;
            }
        }

        self.enemies = std::mem::take(&mut self.enemies)
            .into_iter()
            .filter(|enemy| {
                if enemy.health <= 0.0 {
                    self.diamonds.push(Diamond {
                        x: enemy.x,
                        y: enemy.y,
                    });
                    return false;
                } else {
                    return true;
                }
            })
            .collect();

        // interact with diamonds
        self.diamonds = std::mem::take(&mut self.diamonds)
            .into_iter()
            .filter(|diamond| {
                let dx = self.player.x - diamond.x;
                let dy = self.player.y - diamond.y;
                let d = dx * dx + dy * dy;
                if d < self.player.diamond_radius * self.player.diamond_radius {
                    self.player.xp += 1;
                    return false;
                } else {
                    return true;
                }
            })
            .collect();

        while self.player.xp >= self.player.next_level {
            self.player.last_level = self.player.next_level;
            self.player.next_level *= 2;

            match self.rng.gen_range(0, 7) {
                PERK_POWER => {
                    self.player.power *= 1.1;
                }
                PERK_HEALTH => {
                    self.player.health_max *= 1.1;
                }
                PERK_SPEED => {
                    self.player.speed *= 1.1;
                }
                PERK_RADIUS => {
                    self.player.damage_radius *= 1.1;
                }
                PERK_HEAL => {
                    self.player.health = self.player.health_max;
                }
                PERK_RECOVER => self.player.health_recover += 0.2,
                PERK_ATTRACT => {
                    self.player.diamond_radius *= 1.1;
                }
                _ => unreachable!(),
            }
        }

        // spawn
        if self.enemies.len() < MAX_ENEMIES && self.rng.gen_f32() < dt * 2.0 {
            let (spawn_x, spawn_y) = match self.rng.gen_range(0, 4) {
                0 => (self.rng.gen_f32() * width, -sprite_height),
                1 => (width + sprite_width, self.rng.gen_f32() * height),
                2 => (self.rng.gen_f32() * width, height + sprite_height),
                3 => (-sprite_width, self.rng.gen_f32() * height),
                _ => unreachable!(),
            };

            self.enemies.push(enemies::get_enemy(
                spawn_x + self.player.x - width / 2.0,
                spawn_y + self.player.y - height / 2.0,
                self.i_enemy,
            ));
            self.i_enemy += 1;
        }
        self.enemies.sort_unstable_by_key(|e| e.y as i32);
    }

    pub fn render(&self, screen: &mut term::Screen) {
        let height = win::iconvert_y(self.win.height);
        let width = win::iconvert_x(self.win.width);

        // render
        self.win.fill(screen, [0x33, 0x88, 0x22]);
        self.win.circle(
            screen,
            width / 2.0,
            height / 2.0,
            self.player.damage_radius,
            [0x00, 0xff, 0x00],
        );

        for diamond in self.diamonds.iter() {
            let sx = diamond.x - self.player.x + width / 2.0;
            let sy = diamond.y - self.player.y + height / 2.0;
            self.win.sprite(screen, sx, sy, &sprites::DIAMOND, false);
        }

        let mut player_rendered = false;
        for enemy in self.enemies.iter() {
            if !player_rendered && enemy.y > self.player.y {
                self.win.sprite(
                    screen,
                    width / 2.0,
                    height / 2.0,
                    &sprites::PLAYER,
                    self.player.face == Dir::Left,
                );
                player_rendered = true;
            }

            let sx = enemy.x - self.player.x + width / 2.0;
            let sy = enemy.y - self.player.y + height / 2.0;
            self.win
                .sprite(screen, sx, sy, enemy.t.sprite, enemy.x > self.player.x);
        }
        if !player_rendered {
            self.win.sprite(
                screen,
                width / 2.0,
                height / 2.0,
                &sprites::PLAYER,
                self.player.face == Dir::Left,
            );
        }
    }
}
