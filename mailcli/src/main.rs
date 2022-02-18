use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};
use std::env;

mod config;
use config::*;

fn main() {
    let config = Config::load();

    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() < 3 {
        println!(
            "invalid amount of arguments!
example usage:
    mailcli reciever@gaming.com subject contents"
        );
        std::process::exit(1);
    }

    tracing_subscriber::fmt::init();

    let email = Message::builder()
        .from(config.username.parse().unwrap())
        .to(args[1].parse().unwrap())
        .reply_to(args[1].parse().unwrap())
        .subject(args[2].clone())
        .body(args[3].clone())
        .unwrap();

    let creds = Credentials::new(config.username, config.password);

    let mail_server = match config.smtp {
        Some(v) => v,
        None => "smtp.gmail.com".to_string(),
    };

    // Open a remote connection to gmail
    let mailer = SmtpTransport::starttls_relay(&mail_server)
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}
