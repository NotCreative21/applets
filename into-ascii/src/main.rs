use image::imageops::{grayscale, resize, FilterType};
use image::{open, EncodableLayout};
use terminal_size::terminal_size;
use std::env;

fn find_pair(arg: String) -> Option<(u16, u16)> {
    let mut seperator = "x".to_string();
    if !arg.contains(&seperator) {
        for i in arg.chars() {
            match i.to_string().parse::<u16>() {
                Ok(_) => {},
                _ => {
                    seperator = i.to_string(); 
                    break;
                }
            }
        }
    }
    let split: Vec<&str> = arg.split(&seperator).collect();
    if split.len() != 2 { 
        println!("invalid dimensions, using terminal size"); 
        return None;
    }
    Some((split[0].parse::<u16>().unwrap(), split[1].parse::<u16>().unwrap()))
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();

    let image = grayscale(&open(args[0].clone()).unwrap());

    let (x, y) = terminal_size().unwrap();
    let (mut w, mut h) = (x.0, y.0);

    match args.len() {
        2.. => {
            let pair = find_pair(args[1].clone());
            match pair {
                Some(v) => {
                    w = v.0;
                    h = v.1;
                },
                None => {}
            };
        },
        _ => {}
    };

    let image = resize(&image, w.into(), h.into(), FilterType::Nearest);

    let chars = vec!["#", "&", "@", "$", "%", "*", ".", " "];

    let mut output = String::new();

    for (e, i) in image.as_bytes().iter().enumerate() {
        if e as u16 % w == 0 { 
            output.push_str("\n");
        }
        output.push_str(chars[*i as usize / 36]);
    }

    println!("{output}");

    Ok(())
}
