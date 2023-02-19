extern crate libc;

mod enemies;
mod game;
mod input;
mod random;
mod sprites;
mod term;
mod win;

use std::sync::atomic::{AtomicBool, Ordering};
use std::{thread, time};

const TICK: time::Duration = time::Duration::from_millis(33);

const BLACK: [u8; 3] = [0x00, 0x00, 0x00];
const RED: [u8; 3] = [0xff, 0x00, 0x00];
const BLUE: [u8; 3] = [0x00, 0x00, 0xff];

static RUNNING: AtomicBool = AtomicBool::new(true);

fn quit(_sig: i32) {
    RUNNING.fetch_and(false, Ordering::Relaxed);
}

fn main() {
    let input = input::Input::new();
    let mut screen = term::Screen::new();
    let mut game = game::Game::new(&screen);

    unsafe {
        libc::signal(libc::SIGINT, quit as usize);
    }

    let mut time0 = time::Instant::now();

    while RUNNING.load(Ordering::Relaxed) {
        let time1 = time::Instant::now();

        match input.getch() {
            Some(b'w' | b'A') => game.player.dir = game::Dir::Up,
            Some(b'a' | b'D') => {
                game.player.dir = game::Dir::Left;
                game.player.face = game::Dir::Left
            }
            Some(b's' | b'B') => game.player.dir = game::Dir::Down,
            Some(b'd' | b'C') => {
                game.player.dir = game::Dir::Right;
                game.player.face = game::Dir::Right
            }
            Some(b' ') => game.player.dir = game::Dir::Stop,
            Some(b'q') => quit(0),
            _ => {}
        }

        game.step((time1 - time0).as_secs_f32());
        game.render(&mut screen);

        let xp_bar = (screen.width as f32 * (game.player.xp - game.player.last_level) as f32
            / (game.player.next_level - game.player.last_level) as f32)
            as usize;
        for x in 0..screen.width {
            let c = if x <= xp_bar { BLUE } else { BLACK };
            for y in 0..3 {
                screen.set(x, y, c);
            }
        }

        let health_bar =
            (screen.width as f32 * game.player.health / game.player.health_max) as usize;
        for x in 0..screen.width {
            let c = if x <= health_bar { RED } else { BLACK };
            for y in (screen.height - 3)..screen.height {
                screen.set(x, y, c);
            }
        }

        screen.render();

        if game.player.health < 0.0 {
            println!("\nyou died (score: {})", game.player.xp);
            break;
        }

        // sleep
        let time2 = time::Instant::now();
        if TICK > time2 - time1 {
            thread::sleep(TICK - (time2 - time1));
        }
        let time3 = time::Instant::now();
        print!("{:?}", 1.0 / (time3 - time1).as_secs_f64());
        time0 = time1;
    }
}
