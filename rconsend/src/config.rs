use std::fs;
use std::fs::File;
use std::io::Write;
use anyhow::Result;
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub servers: Vec<Server>
}

#[derive(Serialize, Deserialize)]
pub struct Server {
    pub name: String,
    pub address: String,
    pub password: String
}

impl Config {
    pub fn load() -> Config {
        let file = match fs::read_to_string(
            dirs::home_dir()
                .unwrap()
                .to_owned()
                .to_str()
                .unwrap()
                .to_owned() 
                + "/.config/sean/rconsend.json") 
        {
            Ok(v) => v,
            Err(e) => {
                eprintln!("failed to load: {e}");
                eprintln!("does the config exist?");

                Config::new().unwrap();
                std::process::exit(1);
            }
        };
        let config: Config = serde_json::from_str(&file).expect("invalid config!");
        config.servers.iter()
            .for_each(|x| println!("loaded: {}", x.address));
        config
    }

    fn new() -> Result<()> {
        let path = dirs::home_dir()
                .unwrap()
                .to_owned()
                .to_str()
                .unwrap()
                .to_owned() 
                + "/.config/sean/rconsend.json";
        let mut new_config = File::create(path)?;

        let default = r#"{
    "servers": [
        {
            "name": "cmp" 
            "address": "192.168.1.95",
            "password": "quacon_cmp"
        }
    ]
}"#.as_bytes();
        new_config.set_len(default.len() as u64)?;
        new_config.write(default)?;

        Ok(())
    }
}
