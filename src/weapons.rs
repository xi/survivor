use crate::game::{Dir, Pos};
use crate::sprites;

pub const SPAWN_RADIUS: f32 = 16.0;

pub struct Projectile {
    pub p: Pos,
    pub dir: Dir,
}

pub struct Weapon {
    pub sprite: &'static sprites::Sprite,
    pub _move: fn(&mut Projectile, &Pos, speed: f32, dt: f32) -> (),
    pub speed: f32,
    pub damage: f32,
    pub cooldown: f32,
    pub size: f32,
    pub amount: u8,
    pub last: f32,
    pub projectiles: Vec<Projectile>,
}

pub fn move_straight(projectile: &mut Projectile, _center: &Pos, speed: f32, dt: f32) {
    match projectile.dir {
        Dir::Up => projectile.p.y -= speed * dt,
        Dir::Right => projectile.p.x += speed * dt,
        Dir::Down => projectile.p.y += speed * dt,
        Dir::Left => projectile.p.x -= speed * dt,
    }
}

pub fn move_diagonal(projectile: &mut Projectile, center: &Pos, speed: f32, dt: f32) {
    let mut dx = projectile.p.x - center.x;
    let dy = projectile.p.y - center.y;
    if dx == 0.0 && dy == 0.0 {
        dx = 1.0;
    }
    let r = f32::sqrt(dx * dx + dy * dy);

    let r2 = r + speed * dt;
    projectile.p.x = center.x + dx / r * r2;
    projectile.p.y = center.y + dy / r * r2;
    projectile.dir = if dx < 0.0 { Dir::Left } else { Dir::Right };
}

pub fn move_parabola(projectile: &mut Projectile, center: &Pos, speed: f32, dt: f32) {
    let dx = (projectile.p.x - center.x).abs();
    let mut t = (dx / speed).max(0.1);
    if dx < SPAWN_RADIUS && projectile.p.y > center.y {
        t = -t;
    }

    let acc = speed * 6.0;
    let dy = projectile.p.y - center.y;
    let y_speed_0 = (acc * t - dy / t).min(speed * 2.0).max(speed / 2.0);

    projectile.p.y += (acc * t - y_speed_0) * dt;
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

    let r2 = r + 25.0 * dt;
    let angle2 = angle + speed * dt / r.max(1.0);

    let (sin, cos) = angle2.sin_cos();

    projectile.p.x = center.x + cos * r2;
    projectile.p.y = center.y + sin * r2;

    projectile.dir = if projectile.p.y > center.y {
        Dir::Left
    } else {
        Dir::Right
    };
}

pub fn create_weapons() -> Vec<Weapon> {
    return vec![
        Weapon {
            sprite: &sprites::AXE,
            _move: move_parabola,
            speed: 150.0,
            damage: 50.0,
            cooldown: 10.0,
            size: 7.0,
            last: 0.0,
            amount: 0,
            projectiles: vec![],
        },
        Weapon {
            sprite: &sprites::KNIFE,
            _move: move_straight,
            speed: 200.0,
            damage: 30.0,
            cooldown: 4.0,
            size: 6.0,
            last: 0.0,
            amount: 0,
            projectiles: vec![],
        },
        Weapon {
            sprite: &sprites::STAR,
            _move: move_diagonal,
            speed: 250.0,
            damage: 20.0,
            cooldown: 3.0,
            size: 6.0,
            last: 0.0,
            amount: 0,
            projectiles: vec![],
        },
        Weapon {
            sprite: &sprites::WIND,
            _move: move_spiral,
            speed: 100.0,
            damage: 40.0,
            cooldown: 9.0,
            size: 8.0,
            last: 0.0,
            amount: 0,
            projectiles: vec![],
        },
    ];
}
