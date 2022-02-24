mod numbers;
use std::time::Duration;

use chrono::Local;
use terminal_size::terminal_size;
use numbers::*;

fn find_digits(time: u8) -> [u8; 2] {
    [time / 10, time % 10]
}

fn match_digits(num: u8) -> &'static str {
    match num {
        0 => zero(),
        1 => one(),
        2 => two(),
        3 => three(),
        4 => four(),
        5 => five(),
        6 => six(),
        7 => seven(),
        8 => eight(),
        9 => nine(),
        _ => ""
    }
}


fn combine_strings(w: u16, h: u16, times: Vec<u8>) {
    let mut slices: Vec<Vec<&str>> = Vec::with_capacity(4);
    for i in times {
        let digit_vec = match_digits(i).split("\n").collect();
        slices.push(digit_vec);
    }
    let mut output = Vec::new();
    for position in 0..7 {
        for _ in 0..((w - 39) / 2) {
            output.push(" ");
        }
        for number in slices.to_owned().into_iter() {
            output.push(number[position]);
        }
        output.push("x");
    }
    for _ in 0..((h - 4) / 2) {
        output.push("x ");
    }
    let output = output.join("");
    let output: Vec<&str> = output.split("x").collect();
    println!("\x1B[2J\x1B[33m");
    for i in output {
        println!("{i}");
    }
    println!("\x1B[0m\x1b[?25l");
}

fn print_time(w: u16, h: u16) {
    let now = Local::now();
    let hour: u8 = now.to_string()[11..13].parse().unwrap();
    let minute: u8 = now.to_string()[14..16].parse().unwrap();
    let times = &[find_digits(hour), find_digits(minute)].concat();
    println!("{:?}", times);
    combine_strings(w, h, times.to_vec());
}

fn main() {
    loop {
        let (w, h) = terminal_size().expect("could not determine terminal size");
        print_time(w.0, h.0);
        std::thread::sleep(Duration::from_secs(5)); 
    }
}
