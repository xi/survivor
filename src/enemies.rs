// https://vampire-survivors.fandom.com/wiki/Enemies

pub struct Enemy {
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
    pub speed: f32,
    pub inertia: f32,
    pub size: f32,
    pub health: f32,
    pub power: f32,
    pub xp: u64,
}

pub fn skeleton(x: f32, y: f32) -> Enemy {
    return Enemy {
        x: x,
        y: y,
        dx: 0.0,
        dy: 0.0,
        inertia: 0.1,
        speed: 10.0,
        size: 10.0,
        health: 10.0,
        power: 5.0,
        xp: 1,
    };
}
