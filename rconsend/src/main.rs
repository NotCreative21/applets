use rcon_rs::Client;
use std::env;
use anyhow::Result;

mod config;
use config::*;

fn send_msg(target: String, msg: String) -> Result<(), ()> {
    let details: Vec<&str> = target.split(":").collect();
    let mut conn = Client::new(details[0], details[1]);
    conn.send(&msg, None)?;
    Ok(())
}

fn main() -> Result<(), ()> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() < 2 {
        eprintln!("invalid amount of arguments!
example usage:
    rconsend 192.168.0.0 say hi
    rconsend server-smp stop
    rconsend all shell");
        return Ok(());
    }

    let ip_target = match args[0][0..1].parse::<u8>() {
        Ok(_) => {
            true
        },
        _ => false,
    };

    if ip_target {
        let new_args: Vec<String> = env::args().skip(2).collect();
        send_msg(
            args[0].clone(),
            new_args.join(" ")
        )?;
        return Ok(());
    }

    let config = Config::load();

    match args[0].as_str() {
        "all" => {
            let args: Vec<String> = env::args().skip(2).collect();
            for i in config.servers {
                send_msg(
                    format!("{}:{}", i.address, i.password), 
                    args.join(" ")
                )?;
            }
        },
        _ => {
            let mut skip = true;
            config.servers.iter().for_each(|i| if i.name == args[0] { skip = false; });
            if skip { 
                eprintln!("invalid server!");
                return Ok(());
            }
            let new_args: Vec<String> = env::args().skip(2).collect();
            send_msg(args[0].clone(), new_args.join(" "))?;
        } 
    }
    Ok(())
}
