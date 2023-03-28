use crate::game::{Dir, Pos};
use crate::sprites;

pub struct WeaponType {
    pub base_speed: f32,
    pub base_damage: f32,
    pub base_cooldown: f32,
    pub base_amount: u8,
    pub size: f32,
    pub sprite: &'static sprites::Sprite,
    pub _move: fn(&mut Projectile, &Pos, speed: f32, dt: f32) -> (),
}

pub struct Projectile {
    pub p: Pos,
    pub dir: Dir,
}

pub struct Weapon {
    pub speed: f32,
    pub damage: f32,
    pub cooldown: f32,
    pub amount: u8,
    pub last: f32,
    pub t: &'static WeaponType,
    pub projectiles: Vec<Projectile>,
}

impl Weapon {
    pub fn new(t: &'static WeaponType) -> Self {
        return Self {
            speed: t.base_speed,
            damage: t.base_damage,
            cooldown: t.base_cooldown,
            amount: t.base_amount,
            last: 0.0,
            t: t,
            projectiles: vec![],
        }
    }
}

pub fn move_straight(projectile: &mut Projectile, center: &Pos, speed: f32, dt: f32) {
    match projectile.dir {
        Dir::Up => projectile.p.y -= speed * dt,
        Dir::Right => projectile.p.x += speed * dt,
        Dir::Down => projectile.p.y += speed * dt,
        Dir::Left => projectile.p.x -= speed * dt,
    }
}

pub fn move_diagonal(projectile: &mut Projectile, center: &Pos, speed: f32, dt: f32) {
    let dx = projectile.p.x - center.x;
    let dy = projectile.p.y - center.y;
    let r = f32::sqrt(dx * dx + dy * dy);

    let r2 = r + speed * dt;
    projectile.p.x = center.x + dx / r * r2;
    projectile.p.y = center.y + dy / r * r2;
    projectile.dir = if dx < 0.0 { Dir::Left } else { Dir::Right };
}

pub fn move_parabola(projectile: &mut Projectile, center: &Pos, speed: f32, dt: f32) {
    let t = (projectile.p.x - center.x).abs() / speed;
    projectile.p.y += speed * (4.0 * t - 1.0) * dt;
    if projectile.p.x < center.x {
        projectile.p.x -= speed * dt;
        projectile.dir = Dir::Left;
    } else {
        projectile.p.x += speed * dt;
        projectile.dir = Dir::Right;
    }
}

pub fn move_spiral(projectile: &mut Projectile, center: &Pos, speed: f32, dt: f32) {
    let dx = projectile.p.x - center.x;
    let dy = projectile.p.y - center.y;
    let r = f32::sqrt(dx * dx + dy * dy);
    let angle = dy.atan2(dx);

    let r2 = r + 20.0 * dt;
    let angle2 = angle + speed * dt / r.max(1.0);

    let (sin, cos) = angle2.sin_cos();

    projectile.p.x = center.x + cos * r2;
    projectile.p.y = center.y + sin * r2;

    projectile.dir = if projectile.p.y > center.y { Dir::Left } else { Dir::Right };
}

pub const AXE: WeaponType = WeaponType {
    base_speed: 150.0,
    base_damage: 50.0,
    base_cooldown: 10.0,
    base_amount: 2,
    size: 7.0,
    sprite: &sprites::AXE,
    _move: move_parabola,
};

pub const KNIFE: WeaponType = WeaponType {
    base_speed: 200.0,
    base_damage: 30.0,
    base_cooldown: 4.0,
    base_amount: 1,
    size: 6.0,
    sprite: &sprites::KNIFE,
    _move: move_straight,
};

pub const STAR: WeaponType = WeaponType {
    base_speed: 250.0,
    base_damage: 20.0,
    base_cooldown: 3.0,
    base_amount: 3,
    size: 6.0,
    sprite: &sprites::STAR,
    _move: move_diagonal,
};

pub const WIND: WeaponType = WeaponType {
    base_speed: 100.0,
    base_damage: 40.0,
    base_cooldown: 9.0,
    base_amount: 1,
    size: 8.0,
    sprite: &sprites::WIND,
    _move: move_spiral,
};
