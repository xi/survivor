extern crate ppm;

pub const HEIGHT: usize = 24;
pub const WIDTH: usize = 18;
pub type Sprite = [[[u8; 3]; WIDTH]; HEIGHT];

pub const PLAYER: Sprite = ppm::include_ppm!("player");
pub const DIAMOND: Sprite = ppm::include_ppm!("diamond");

pub const BAT: Sprite = ppm::include_ppm!("bat");
pub const BAT2: Sprite = ppm::include_ppm!("bat2");
pub const CRAWL: Sprite = ppm::include_ppm!("crawl");
pub const EYE: Sprite = ppm::include_ppm!("eye");
pub const GHOST: Sprite = ppm::include_ppm!("ghost");
pub const HOOD: Sprite = ppm::include_ppm!("hood");
pub const MUMMY: Sprite = ppm::include_ppm!("mummy");
pub const PLANTGUY: Sprite = ppm::include_ppm!("plantguy");
pub const SHADOW: Sprite = ppm::include_ppm!("shadow");
pub const SKELETON: Sprite = ppm::include_ppm!("skeleton");
pub const SKELETON2: Sprite = ppm::include_ppm!("skeleton2");
pub const SNAKE: Sprite = ppm::include_ppm!("snake");
pub const ZOMBIE: Sprite = ppm::include_ppm!("zombie");
