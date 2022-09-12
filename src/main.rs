use termion;
use termion::raw::IntoRawMode;

use std::io::{stdout, Read, Write};
use std::time::{Duration, Instant};

mod point;
mod universe;
mod term;

use point::Point;

const FPS: f64 = 30f64;

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut stdin = termion::async_stdin().bytes();

    let tgt_frame_time = Duration::from_secs_f64(1.0 / FPS);

    let mut u = universe::Universe::new();
    u.update_dims();

    for _ in 0..5000 {
        u.add_rand();
    }

    let mut last_frame = Instant::now();
    let mut ticks_since_last = 1;

    loop {
        let now = Instant::now();
        let time_since_last = now - last_frame;
        if time_since_last >= tgt_frame_time {
            u.frame(&mut stdout);
            write!(
                stdout,
                "{}{} Ticks per frame, {:?}",
                termion::cursor::Goto(1, 1),
                ticks_since_last,
                time_since_last
            ).unwrap();
            stdout.flush().unwrap();
            last_frame = now;
            ticks_since_last = 1;
        } else {
            ticks_since_last += 1;
            u.tick();
        }
        if let Some(Ok(b'q')) = stdin.next() {
            break;
        }
    }
}
