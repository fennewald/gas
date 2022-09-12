use termion;
use termion::raw::IntoRawMode;

use std::io::{stdout, Read};
use std::{thread, time};

mod point;
mod universe;
mod term;

use point::Point;

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut stdin = termion::async_stdin().bytes();
    let frame = time::Duration::from_millis(16);

    let mut u = universe::Universe::new();
    u.update_dims();

    for _ in 0..1000 {
        u.add_rand();
    }

    loop {
        u.frame(&mut stdout);
        if let Some(Ok(b'q')) = stdin.next() {
            break;
        }
        //thread::sleep(frame);
    }
}
