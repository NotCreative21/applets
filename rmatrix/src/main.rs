use termion::terminal_size;
use std::{thread, time};

use rmatrix::*;

fn main() {

    let (h, w) = terminal_size().unwrap();

    let color: String = "Green".to_string();
    
    /*
    if envargs[2] != None {
        color = envargs[2];
    }*/

    let mut frame = create_frame(w, h, color);

    loop {
        frame = draw_frame(frame);
        thread::sleep(time::Duration::from_millis(10));
        frame = print_frame(frame);
    }
}
