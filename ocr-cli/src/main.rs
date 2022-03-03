use leptess::{leptonica, tesseract};
use std::path::Path;
use std::env;
use std::time::Instant;

fn main() {
    let startup = Instant::now();

    let args: Vec<String> = env::args().into_iter().skip(1).collect();

    let mut api = tesseract::TessApi::new(Some("./eng.traineddata"), "eng").unwrap();

    let pix = leptonica::pix_read(Path::new(&args[0])).unwrap();
    api.set_image(&pix);

    // detect bounding boxes for words
    let boxes = api
        .get_component_images(leptess::capi::TessPageIteratorLevel_RIL_WORD, true)
        .unwrap();

    let count = boxes.get_n();

    if args.contains(&"-v".to_string()) {
        println!("\x1b[32mFound {count} textline image components.\x1b[mm");
    }

    let mut words = String::new();

    let mut total_confidence: usize = 0;

    // run OCR on each word bounding box
    for b in &boxes {
        api.set_rectangle(&b);
        let text = api.get_utf8_text().unwrap();
        let confi = api.mean_text_conf();
        if confi < 10 || text.len() < 1 { continue; }
        total_confidence += confi as usize;
        if args.contains(&"-v".to_string()) {
            print!("{confi}% {text}");
        }
        words.push_str(&(text[..text.len() - 1].to_owned() + " "));
    }

    if !args.contains(&"-r".to_string()) {
        println!(
            "\x1b[32maverage confidence: {}%\x1b[0m", 
            total_confidence / count
        );
    }

    println!("{words}");
    if args.contains(&"-v".to_string()) {
        println!("\x1b[32mdone in {:#?}\x1b[0m", startup.elapsed());
    }
}
