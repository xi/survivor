// https://vampire-survivors.fandom.com/wiki/Enemies

use crate::sprites;

pub struct EnemyType {
    pub speed: f32,
    pub size: f32,
    pub health: f32,
    pub power: f32,
    pub sprite: &'static sprites::Sprite,
}

pub struct Enemy {
    pub x: f32,
    pub y: f32,
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
    speed: 12.0,
    size: 7.0,
    health: 20.0,
    power: 20.0,
    sprite: &sprites::EYE,
};

const GHOST: EnemyType = EnemyType {
    speed: 18.0,
    size: 9.0,
    health: 15.0,
    power: 15.0,
    sprite: &sprites::GHOST,
};

const MUMMY: EnemyType = EnemyType {
    speed: 8.0,
    size: 9.0,
    health: 30.0,
    power: 25.0,
    sprite: &sprites::MUMMY,
};

const HOOD: EnemyType = EnemyType {
    speed: 14.0,
    size: 9.0,
    health: 30.0,
    power: 40.0,
    sprite: &sprites::HOOD,
};

const PLANTGUY: EnemyType = EnemyType {
    speed: 10.0,
    size: 9.0,
    health: 50.0,
    power: 30.0,
    sprite: &sprites::PLANTGUY,
};

const CRAWL: EnemyType = EnemyType {
    speed: 5.0,
    size: 9.0,
    health: 60.0,
    power: 40.0,
    sprite: &sprites::CRAWL,
};

const SHADOW: EnemyType = EnemyType {
    speed: 20.0,
    size: 9.0,
    health: 50.0,
    power: 50.0,
    sprite: &sprites::SHADOW,
};

pub fn get_enemy(x: f32, y: f32, i: usize) -> Enemy {
    let n = 50;

    let t: &EnemyType = if i < 1 * n {
        &SNAKE
    } else if i < 3 * n {
        if i % 2 == 0 {
            &SNAKE
        } else {
            &SKELETON
        }
    } else if i < 4 * n {
        &BAT
    } else if i < 6 * n {
        &ZOMBIE
    } else if i < 8 * n {
        if i % 2 == 0 {
            &BAT
        } else {
            &SKELETON
        }
    } else if i < 9 * n {
        if i % 2 == 0 {
            &BAT
        } else {
            &EYE
        }
    } else if i < 11 * n {
        &GHOST
    } else if i < 13 * n {
        if i % 2 == 0 {
            &BAT2
        } else {
            &ZOMBIE
        }
    } else if i < 15 * n {
        if i % 2 == 0 {
            &ZOMBIE
        } else {
            &MUMMY
        }
    } else if i < 18 * n {
        &HOOD
    } else if i < 20 * n {
        if i % 2 == 0 {
            &SNAKE
        } else {
            &PLANTGUY
        }
    } else if i < 22 * n {
        if i % 2 == 0 {
            &SKELETON2
        } else {
            &CRAWL
        }
    } else if i < 23 * n {
        &CRAWL
    } else {
        &SHADOW
    };

    return Enemy {
        x: x,
        y: y,
        health: t.health,
        t: t,
    };
}
