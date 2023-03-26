use crate::enemies;
use crate::random;
use crate::sprites;
use crate::weapons;
use crate::win;

const MAX_ENEMIES: usize = 200;

const PERK_POWER: usize = 0;
const PERK_HEALTH: usize = 1;
const PERK_SPEED: usize = 2;
const PERK_RADIUS: usize = 3;
const PERK_HEAL: usize = 4;
const PERK_RECOVER: usize = 5;
const PERK_ATTRACT: usize = 6;
const PERK_XP: usize = 7;

#[derive(PartialEq)]
pub enum Dir {
    Up,
    Right,
    Down,
    Left,
}

pub struct Pos {
    pub x: f32,
    pub y: f32,
}

pub struct Player {
    pub p: Pos,
    pub dir: Option<Dir>,
    pub face: Dir,
    pub speed: f32,
    pub size: f32,
    pub health: f32,
    pub health_max: f32,
    pub health_recover: f32,
    pub power: f32,
    pub damage_radius: f32,
    pub diamond_radius: f32,
    pub xp: f32,
    pub xp_factor: f32,
    pub last_level: f32,
    pub next_level: f32,
}

impl Player {
    pub fn new() -> Self {
        return Self {
            p: Pos { x: 0.0, y: 0.0 },
            dir: None,
            face: Dir::Right,
            speed: 30.0,
            size: 9.0,
            health: 50.0,
            health_max: 50.0,
            health_recover: 0.0,
            power: 10.0,
            damage_radius: 30.0,
            diamond_radius: 15.0,
            xp: 0.0,
            xp_factor: 1.0,
            last_level: 0.0,
            next_level: 10.0,
        };
    }
}

impl Player {
    pub fn recover(&mut self, dt: f32) {
        self.health = (self.health + self.health_recover * dt).min(self.health_max);
    }

    pub fn levelup(&mut self, rng: &mut random::Rng) {
        while self.xp >= self.next_level {
            self.last_level = self.next_level;
            self.next_level *= 1.3;

            match rng.gen_range(0, 8) {
                PERK_POWER => self.power *= 1.1,
                PERK_HEALTH => self.health_max *= 1.1,
                PERK_SPEED => self.speed *= 1.1,
                PERK_RADIUS => self.damage_radius *= 1.1,
                PERK_HEAL => self.health = self.health_max,
                PERK_RECOVER => self.health_recover += 0.2,
                PERK_ATTRACT => self.diamond_radius *= 1.1,
                PERK_XP => self.xp_factor *= 1.1,
                _ => unreachable!(),
            }
        }
    }
}

pub struct Game {
    pub player: Player,
    pub diamonds: Vec<Pos>,
    pub enemies: Vec<enemies::Enemy>,
    pub projectiles: Vec<weapons::Projectile>,
    pub i_enemy: usize,
    rng: random::Rng,
}

impl Game {
    pub fn new() -> Self {
        return Self {
            enemies: vec![],
            projectiles: vec![],
            diamonds: vec![],
            i_enemy: 0,
            player: Player::new(),
            rng: random::Rng::new(),
        };
    }

    fn move_player(&mut self, dt: f32) {
        match self.player.dir {
            Some(Dir::Up) => self.player.p.y -= self.player.speed * dt,
            Some(Dir::Right) => self.player.p.x += self.player.speed * dt,
            Some(Dir::Down) => self.player.p.y += self.player.speed * dt,
            Some(Dir::Left) => self.player.p.x -= self.player.speed * dt,
            None => {},
        };
    }

    fn move_enemies(&mut self, dt: f32) {
        for i in 0..self.enemies.len() {
            let enemy = &self.enemies[i];

            let dxp = self.player.p.x - enemy.p.x;
            let dyp = self.player.p.y - enemy.p.y;
            let dp = (dxp * dxp + dyp * dyp).sqrt();

            let mut dx = dxp / dp;
            let mut dy = dyp / dp;

            for j in 0..self.enemies.len() {
                if i != j {
                    let other = &self.enemies[j];

                    let dxm = other.p.x - enemy.p.x;
                    let dym = other.p.y - enemy.p.y;
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
            enemy.p.x += dx * enemy.t.speed * dt;
            enemy.p.y += dy * enemy.t.speed * dt;
        }
    }

    fn spawn_enemies(&mut self, dt: f32, width: f32, height: f32) {
        let sprite_height = win::iconvert_y(sprites::HEIGHT);
        let sprite_width = win::iconvert_x(sprites::WIDTH);

        if self.enemies.len() < MAX_ENEMIES && self.rng.gen_f32() < dt * 2.0 {
            let (spawn_x, spawn_y) = match self.rng.gen_range(0, 4) {
                0 => (self.rng.gen_f32() * width, -sprite_height),
                1 => (width + sprite_width, self.rng.gen_f32() * height),
                2 => (self.rng.gen_f32() * width, height + sprite_height),
                3 => (-sprite_width, self.rng.gen_f32() * height),
                _ => unreachable!(),
            };

            self.enemies.push(enemies::get_enemy(
                spawn_x + self.player.p.x - width / 2.0,
                spawn_y + self.player.p.y - height / 2.0,
                self.i_enemy,
            ));
            self.i_enemy += 1;
        }
    }

    fn despawn_enemies(&mut self, width: f32, height: f32) {
        self.enemies = std::mem::take(&mut self.enemies)
            .into_iter()
            .filter(|e| {
                (e.p.y - self.player.p.y).abs() < height && (e.p.x - self.player.p.x).abs() < width
            })
            .collect();
    }

    fn apply_damage(&mut self, dt: f32) {
        for enemy in self.enemies.iter_mut() {
            let dx = self.player.p.x - enemy.p.x;
            let dy = self.player.p.y - enemy.p.y;
            let dx2 = dx * dx;
            let dy2 = dy * dy;

            let size = enemy.t.size + self.player.size;
            if dx2 + dy2 * 4.0 < size * size {
                self.player.health -= enemy.t.power * dt;
            }

            if dx2 + dy2 < self.player.damage_radius * self.player.damage_radius {
                enemy.health -= self.player.power * dt;
            }
        }

        self.enemies = std::mem::take(&mut self.enemies)
            .into_iter()
            .filter(|enemy| {
                if enemy.health <= 0.0 {
                    self.diamonds.push(Pos {
                        x: enemy.p.x,
                        y: enemy.p.y,
                    });
                    return false;
                } else {
                    return true;
                }
            })
            .collect();
    }

    fn pick_diamonds(&mut self) {
        self.diamonds = std::mem::take(&mut self.diamonds)
            .into_iter()
            .filter(|diamond| {
                let dx = self.player.p.x - diamond.x;
                let dy = self.player.p.y - diamond.y;
                let d = dx * dx + dy * dy;
                if d < self.player.diamond_radius * self.player.diamond_radius {
                    self.player.xp += self.player.xp_factor;
                    return false;
                } else {
                    return true;
                }
            })
            .collect();
    }

    pub fn step(&mut self, dt: f32, width: f32, height: f32) {
        self.move_player(dt);
        self.move_enemies(dt);
        self.despawn_enemies(width, height);

        self.apply_damage(dt);
        self.pick_diamonds();

        self.player.recover(dt);
        self.player.levelup(&mut self.rng);
        self.spawn_enemies(dt, width, height);
    }

    pub fn render(&mut self, win: &mut win::Window) {
        let height = win::iconvert_y(win.height);
        let width = win::iconvert_x(win.width);
        let dx = width / 2.0 - self.player.p.x;
        let dy = height / 2.0 - self.player.p.y;

        win.fill([0x33, 0x88, 0x22]);
        win.circle(
            width / 2.0,
            height / 2.0,
            self.player.damage_radius,
            [0x00, 0xff, 0x00],
        );

        for diamond in self.diamonds.iter() {
            win.sprite(diamond.x + dx, diamond.y + dy, &sprites::DIAMOND, false);
        }

        let mut player_rendered = false;
        self.enemies.sort_unstable_by_key(|e| e.p.y as i32);
        for enemy in self.enemies.iter() {
            if !player_rendered && enemy.p.y > self.player.p.y {
                win.sprite(
                    width / 2.0,
                    height / 2.0,
                    &sprites::PLAYER,
                    self.player.face == Dir::Left,
                );
                player_rendered = true;
            }

            win.sprite(
                enemy.p.x + dx,
                enemy.p.y + dy,
                enemy.t.sprite,
                enemy.p.x > self.player.p.x,
            );
        }
        if !player_rendered {
            win.sprite(
                width / 2.0,
                height / 2.0,
                &sprites::PLAYER,
                self.player.face == Dir::Left,
            );
        }

        for projectile in self.projectiles.iter() {
            win.sprite(
                projectile.p.x + dx,
                projectile.p.y + dy,
                projectile.t.sprite,
                false,
            );
        }
    }
}
