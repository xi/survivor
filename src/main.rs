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
static NEED_RESIZE: AtomicBool = AtomicBool::new(false);

fn quit(_sig: libc::c_int) {
    RUNNING.store(false, Ordering::Relaxed);
}

fn resize(_sig: libc::c_int) {
    NEED_RESIZE.store(true, Ordering::Relaxed);
}

fn render_bar(screen: &mut term::Screen, value: f32, y0: usize, color: [u8; 3]) {
    let x0 = (screen.width as f32 * value) as usize;
    for x in 0..screen.width {
        let c = if x <= x0 { color } else { BLACK };
        for dy in 0..3 {
            screen.set(x, y0 + dy, c);
        }
    }
}

fn render_xp_bar(player: &game::Player, screen: &mut term::Screen) {
    let value = (player.xp - player.last_level) / (player.next_level - player.last_level);
    render_bar(screen, value, 0, BLUE);
}

fn render_health_bar(player: &game::Player, screen: &mut term::Screen) {
    let value = player.health / player.health_max;
    render_bar(screen, value, screen.height - 3, RED);
}

fn main() {
    let input = input::Input::new();
    let mut screen = term::Screen::new();
    let mut game = game::Game::new();

    unsafe {
        libc::signal(libc::SIGINT, quit as libc::sighandler_t);
        libc::signal(libc::SIGWINCH, resize as libc::sighandler_t);
    }

    let mut time0 = time::Instant::now();

    while RUNNING.load(Ordering::Relaxed) {
        let time1 = time::Instant::now();
        let dt = (time1 - time0).as_secs_f32();

        while let Some(c) = input.getch() {
            match c {
                b'w' | b'A' => game.player.dir = game::Dir::Up,
                b'a' | b'D' => {
                    game.player.dir = game::Dir::Left;
                    game.player.face = game::Dir::Left
                }
                b's' | b'B' => game.player.dir = game::Dir::Down,
                b'd' | b'C' => {
                    game.player.dir = game::Dir::Right;
                    game.player.face = game::Dir::Right
                }
                b' ' => game.player.dir = game::Dir::Stop,
                b'q' => quit(0),
                _ => {}
            }
        }

        if NEED_RESIZE.load(Ordering::Relaxed) {
            screen.resize();
            NEED_RESIZE.store(false, Ordering::Relaxed);
        }

        let mut win = win::Window {
            width: screen.width,
            height: screen.height - 6,
            dx: 0,
            dy: 3,
            screen: &mut screen,
        };
        let width = win::iconvert_x(win.width);
        let height = win::iconvert_y(win.height);
        game.step(dt, width, height);
        game.render(&mut win);

        render_xp_bar(&game.player, &mut screen);
        render_health_bar(&game.player, &mut screen);

        screen.render();
        print!("{:?}", 1.0 / dt);

        if game.player.health < 0.0 {
            println!("\nyou died (score: {})", game.player.xp as usize);
            break;
        }

        let time2 = time::Instant::now();
        if TICK > time2 - time1 {
            thread::sleep(TICK - (time2 - time1));
        }
        time0 = time1;
    }
}
