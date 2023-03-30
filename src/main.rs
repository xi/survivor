extern crate libc;

mod enemies;
mod game;
mod input;
mod random;
mod sprites;
mod term;
mod weapons;
mod win;

use std::sync::atomic::{AtomicBool, Ordering};
use std::{thread, time};

const TICK: time::Duration = time::Duration::from_millis(33);

const BLACK: [u8; 3] = [0x00, 0x00, 0x00];
const RED: [u8; 3] = [0xff, 0x00, 0x00];
const BLUE: [u8; 3] = [0x00, 0x00, 0xff];

static NEED_QUIT: AtomicBool = AtomicBool::new(false);
static NEED_RESIZE: AtomicBool = AtomicBool::new(false);
static NEED_STOP: AtomicBool = AtomicBool::new(false);

fn handle_signal(sig: libc::c_int) {
    let var = match sig {
        libc::SIGINT => &NEED_QUIT,
        libc::SIGWINCH => &NEED_RESIZE,
        libc::SIGTSTP => &NEED_STOP,
        _ => unreachable!(),
    };
    var.store(true, Ordering::Relaxed);
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

fn signal(sig: libc::c_int, handler: libc::sighandler_t) {
    let mut action: libc::sigaction;
    unsafe {
        action = std::mem::zeroed();
        action.sa_sigaction = handler;
        libc::sigemptyset(&mut action.sa_mask);
        action.sa_flags = libc::SA_RESTART;
        action.sa_restorer = None;
        libc::sigaction(sig, &action, std::ptr::null_mut());
    }
}

fn main() {
    let pid = std::process::id();
    let mut input = input::Input::new();
    let mut screen = term::Screen::new();
    let mut game = game::Game::new();

    signal(libc::SIGINT, handle_signal as libc::sighandler_t);
    signal(libc::SIGWINCH, handle_signal as libc::sighandler_t);
    signal(libc::SIGTSTP, handle_signal as libc::sighandler_t);

    let mut time0 = time::Instant::now();

    while !NEED_QUIT.load(Ordering::Relaxed) {
        if NEED_STOP.load(Ordering::Relaxed) {
            screen.restore();
            input.restore();
            unsafe {
                libc::kill(pid as libc::c_int, libc::SIGSTOP);
            }

            // when SIGCONT is received
            screen.init();
            input.cbreak();
            time0 = time::Instant::now();
            NEED_STOP.store(false, Ordering::Relaxed);
        }

        if NEED_RESIZE.load(Ordering::Relaxed) {
            screen.resize();
            NEED_RESIZE.store(false, Ordering::Relaxed);
        }

        let time1 = time::Instant::now();
        let dt = (time1 - time0).as_secs_f32();

        while let Some(c) = input.getch() {
            match c {
                b'w' | b'A' => game.player.dir = Some(game::Dir::Up),
                b'a' | b'D' => {
                    game.player.dir = Some(game::Dir::Left);
                    game.player.face = game::Dir::Left
                }
                b's' | b'B' => game.player.dir = Some(game::Dir::Down),
                b'd' | b'C' => {
                    game.player.dir = Some(game::Dir::Right);
                    game.player.face = game::Dir::Right
                }
                b' ' => game.player.dir = None,
                b'q' => NEED_QUIT.store(true, Ordering::Relaxed),
                _ => {}
            }
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
