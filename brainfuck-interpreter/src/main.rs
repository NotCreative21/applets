use std::{env, fs::{File, self}, io::Write};

fn run(data: &[u8]) -> String{
    let (mut pi, mut ps) = (0, 0);
    let mut s = [0u8; 1024];
    let mut o = String::new();

    while pi < data.len() {
        match data[pi] {
            b'[' => {
                if s[ps] == 0 {
                    let mut lc = 1;
                    pi += 1;
                    while lc > 0 {
                        match data[pi] {
                            b'[' => lc += 1,
                            b']' => lc -= 1,
                            _ => {}
                        }
                        pi += 1;
                    }
                }
            },
            b']' => { 
                if s[ps] != 0 {
                    let mut lc = 1;
                    pi -= 1;
                    while lc > 0 {
                        match data[pi] {
                            b'[' => lc -= 1,
                            b']' => lc += 1,
                            _ => {}
                        }

                        pi -= 1;
                    }
                }
            },
            b'>' => ps = ps.wrapping_add(1),
            b'<' => ps = ps.wrapping_sub(1),
            b'+' => s[ps] = s[ps].wrapping_add(1),
            b'-' => s[ps] = s[ps].wrapping_sub(1),
            b'.' => o.push(s[ps] as char),
            _ => panic!("invalid brain fuck detected!"),
        }
        pi += 1;
    }
    o
}

fn parse_pair(args: Vec<String>, target: &str) -> Option<String> {
    let mut start: usize = 500;

    for (e, i) in args.iter().enumerate() {
        if *i == target {
            start = e;
        }
    } 

    if start == 500 || args.len() < start + 2 { return None; }

    Some(args[start + 1].clone())

}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() < 1 {
        panic!(r#"invalid usage!
example usage:
    brainfuck-interpreter -i input.txt -o output.txt
    brainfuck-interpreter -i input.txt
    brainfuck-interpreter "+++[>+++"..."#);
    }

    let mut command = String::new();

    if &args[0] == "-i" {
        let write = match parse_pair(args.clone(), "-i") {
            Some(v) => v,
            None => panic!("failed to parse arguments, please specify output file")
        };
        command = fs::read_to_string(write)
            .expect("failed to read input file");
    } else {
        command = args.join(" ");
    }

    let len = command.len() - 1;
    let command = &command[..len];
    println!("running: {}", command);

    if args.len() > 3 && &args[2] == "-o" {
        let file = match parse_pair(args.clone(), "-o") {
            Some(v) => v,
            None => panic!("failed to find output file, exiting")
        };
        println!("{file}");
        let _ = fs::remove_file(&file);
        let mut output = File::create(file)
            .expect("could not create output file");

        output.write(run(command.as_bytes()).as_bytes())
            .expect("could not write output");
        std::process::exit(0);
    }

    println!("{}", run(command.as_bytes()));
}

