use rand::Rng;

pub struct Frame {
    pub pixels: Vec<Vec<u16>>,
    pub width: u16,
    pub height: u16,
    pub color: String,
}

pub fn create_frame(w: u16, h: u16, color: String) -> Frame {
    let mut term_frame = Frame {
        pixels: vec![vec![0; w.into()]; h.into()],
        width: w,
        height: h,
        color: color,
    };
    term_frame = draw_frame(term_frame);
    term_frame
}

pub fn draw_frame(mut frame: Frame) -> Frame {
    let mut rng = rand::thread_rng();
    for i in 1..frame.height {
        for e in 1..frame.width {
            if frame.pixels[i as usize][e as usize] != 0 {
                frame.pixels[(i - 1) as usize][e as usize] = frame.pixels[i as usize][e as usize];
                if rng.gen_range(0..3) == 0 {
                    frame.pixels[i as usize][e as usize] = frame.pixels[i as usize][e as usize] - 1;
                    if frame.pixels[i as usize][e as usize] == 1{
                        frame.pixels[i as usize][e as usize] = 0;
                    }
                }
            }
        }
    }
    if frame.pixels[rng.gen_range(1..frame.height) as usize][rng.gen_range(1..frame.width) as usize] == 0 {
        // max lenght
        frame.pixels[rng.gen_range(1..frame.height) as usize][rng.gen_range(1..frame.width) as usize] = rng.gen_range(0..18) as u16; 
    }
    frame
}

pub fn print_frame(frame: Frame) -> Frame {
    let mut buffer: String = String::new(); 
    for i in frame.pixels.iter().flat_map(|r| r.iter()) {
        let mut new: String = String::new();
        if *i as u16 == 1 {
            new = format!("{}{}", termion::color::Fg(termion::color::Green), i.to_string());
        }
        else if *i as u16 > 1 {
            new = format!("{}{}", termion::color::Fg(termion::color::LightGreen), i.to_string());
        }
        else {
            new = " ".to_string();
        }
        buffer.push_str(&new);
    } 
    println!("{}{}", termion::cursor::Hide, buffer);
    frame
}   
