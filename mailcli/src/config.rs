use serde_derive::Deserialize;
use std::fs::{read_to_string, File};
use std::io::Write;
use toml::from_str;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub username: String,
    pub password: String,
    pub smtp: Option<String>,
}

impl Config {
    pub fn new() {
        let mut config_file = match File::create(
            dirs::home_dir().unwrap().to_str().unwrap().to_owned() + "/.config/mailcli.toml",
        ) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("failed to create new config file due to: {e}! exiting");
                std::process::exit(1);
            }
        };

        writeln!(
            &mut config_file,
            "username = ''
password = ''
#stmp = 'uncomment and replace with custom mail server, otherwise gmail is assumed'")
            .expect("failed to write default config");
    }

    pub fn load() -> Config {
        let config_contents = match read_to_string(
            dirs::home_dir().unwrap().to_str().unwrap().to_owned() + "/.config/mailcli.toml",
        ) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("failed to read config file due to {e}");
                Config::new();
                println!(
                    "creating new one and exiting, please fill it out at ~/.config/mailcli.toml"
                );
                std::process::exit(0);
            }
        };
        let conf = match from_str(&config_contents) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("invalid config!\n{e}");
                std::process::exit(1);
            }
        };
        conf
    }
}
