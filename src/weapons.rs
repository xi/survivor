use crate::game::{Dir, Pos};
use crate::sprites;

pub struct WeaponType {
    pub base_speed: f32,
    pub base_damage: f32,
    pub base_cooldown: f32,
    pub size: f32,
    pub sprite: &'static sprites::Sprite,
}

pub struct Projectile {
    pub p: Pos,
    pub dir: Dir,
}

pub struct Weapon {
    pub speed: f32,
    pub damage: f32,
    pub cooldown: f32,
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
            last: 0.0,
            t: t,
            projectiles: vec![],
        }
    }
}

pub const KNIFE: WeaponType = WeaponType {
    base_speed: 200.0,
    base_damage: 30.0,
    base_cooldown: 4.0,
    size: 6.0,
    sprite: &sprites::KNIFE,
};
