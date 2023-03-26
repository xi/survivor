use crate::game::{Dir, Pos};
use crate::sprites;

pub struct ProjectileType {
    pub speed: f32,
    pub size: f32,
    pub damage: f32,
    pub sprite: &'static sprites::Sprite,
}

pub struct Projectile {
    pub p: Pos,
    pub dir: Dir,
    pub t: &'static ProjectileType,
}

pub const KNIFE: ProjectileType = ProjectileType {
    speed: 200.0,
    size: 6.0,
    damage: 30.0,
    sprite: &sprites::KNIFE,
};
