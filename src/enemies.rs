// https://vampire-survivors.fandom.com/wiki/Enemies

use crate::game::Pos;
use crate::sprites;

pub struct EnemyType {
    pub speed: f32,
    pub size: f32,
    pub health: f32,
    pub power: f32,
    pub sprite: &'static sprites::Sprite,
}

pub struct Enemy {
    pub p: Pos,
    pub health: f32,
    pub t: &'static EnemyType,
}

const SNAKE: EnemyType = EnemyType {
    speed: 15.0,
    size: 8.0,
    health: 5.0,
    power: 5.0,
    sprite: &sprites::SNAKE,
};

const SKELETON: EnemyType = EnemyType {
    speed: 15.0,
    size: 9.0,
    health: 10.0,
    power: 10.0,
    sprite: &sprites::SKELETON,
};

const SKELETON2: EnemyType = EnemyType {
    speed: 15.0,
    size: 9.0,
    health: 20.0,
    power: 20.0,
    sprite: &sprites::SKELETON2,
};

const BAT: EnemyType = EnemyType {
    speed: 25.0,
    size: 8.0,
    health: 4.0,
    power: 4.0,
    sprite: &sprites::BAT,
};

const BAT2: EnemyType = EnemyType {
    speed: 25.0,
    size: 9.0,
    health: 30.0,
    power: 5.0,
    sprite: &sprites::BAT2,
};

const ZOMBIE: EnemyType = EnemyType {
    speed: 5.0,
    size: 9.0,
    health: 30.0,
    power: 20.0,
    sprite: &sprites::ZOMBIE,
};

const EYE: EnemyType = EnemyType {
    speed: 13.0,
    size: 7.0,
    health: 25.0,
    power: 25.0,
    sprite: &sprites::EYE,
};

const GHOST: EnemyType = EnemyType {
    speed: 18.0,
    size: 9.0,
    health: 30.0,
    power: 30.0,
    sprite: &sprites::GHOST,
};

const MUMMY: EnemyType = EnemyType {
    speed: 9.0,
    size: 9.0,
    health: 40.0,
    power: 30.0,
    sprite: &sprites::MUMMY,
};

const HOOD: EnemyType = EnemyType {
    speed: 18.0,
    size: 9.0,
    health: 40.0,
    power: 50.0,
    sprite: &sprites::HOOD,
};

const HOOD2: EnemyType = EnemyType {
    speed: 16.0,
    size: 9.0,
    health: 60.0,
    power: 70.0,
    sprite: &sprites::HOOD2,
};

const PLANTGUY: EnemyType = EnemyType {
    speed: 12.0,
    size: 9.0,
    health: 60.0,
    power: 40.0,
    sprite: &sprites::PLANTGUY,
};

const RADDISH: EnemyType = EnemyType {
    speed: 17.0,
    size: 9.0,
    health: 40.0,
    power: 60.0,
    sprite: &sprites::RADDISH,
};

const CRAWL: EnemyType = EnemyType {
    speed: 5.0,
    size: 9.0,
    health: 75.0,
    power: 50.0,
    sprite: &sprites::CRAWL,
};

const SHADOW: EnemyType = EnemyType {
    speed: 22.0,
    size: 9.0,
    health: 75.0,
    power: 75.0,
    sprite: &sprites::SHADOW,
};

pub fn get_enemy(x: f32, y: f32, i: usize) -> Enemy {
    let waves = [
        vec![&SNAKE],
        vec![&SNAKE, &SKELETON],
        vec![&SNAKE, &SKELETON],
        vec![&BAT],
        vec![&ZOMBIE],
        vec![&ZOMBIE],
        vec![&BAT, &SKELETON],
        vec![&BAT, &SKELETON],
        vec![&BAT, &EYE],
        vec![&GHOST],
        vec![&GHOST],
        vec![&BAT2, &ZOMBIE],
        vec![&BAT2, &ZOMBIE],
        vec![&MUMMY, &ZOMBIE],
        vec![&MUMMY, &ZOMBIE],
        vec![&HOOD],
        vec![&HOOD, &EYE],
        vec![&SNAKE, &PLANTGUY],
        vec![&SNAKE, &PLANTGUY],
        vec![&HOOD2],
        vec![&HOOD2, &HOOD],
        vec![&HOOD2],
        vec![&SNAKE, &RADDISH],
        vec![&RADDISH, &PLANTGUY],
        vec![&SKELETON2, &CRAWL],
        vec![&SKELETON2, &CRAWL],
        vec![&CRAWL],
        vec![&SHADOW],
        vec![&SHADOW],
        vec![&SHADOW],
    ];

    let wave = &waves[(i / 100) % waves.len()];
    let t = wave[i % wave.len()];

    return Enemy {
        p: Pos { x: x, y: y },
        health: t.health,
        t: t,
    };
}
