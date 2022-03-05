use serde::{Deserialize, Serialize};
use std::{
    fs::{self, read_to_string, File},
    io::Write,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub token: String,
    pub blacklist_users: Vec<u64>,
    pub blacklist_channels: Vec<u64>,
    pub blacklist_roles: Vec<u64>,
    pub user_whitelist: Option<Vec<u64>>,
}

impl Config {
    pub fn load() -> Config {
        let config = match read_to_string("./config.json") {
            Ok(v) => v,
            Err(_) => panic!("invalid or missing config!"),
        };
        let conf = match serde_json::from_str(&config) {
            Ok(v) => v,
            Err(e) => panic!("invalid config!: {e}"),
        };
        conf
    }

    pub fn default() -> Result<(), std::io::Error> {
        let mut conf = match File::create("./config.json") {
            Ok(v) => v,
            Err(e) => panic!("failed to generate default config {e}"),
        };

        let data = r#"
{
    "token": "",
    "blacklist_users": [],
    "blacklist_channels": [],
    "blacklist_roles": [],
    "user_whitelist": []
}
            "#;

        conf.write_all(data.as_bytes())?;

        Ok(())
    }

    pub fn reset() -> Result<(), std::io::Error> {
        fs::remove_file("./config.json")?;
        Config::default()?;
        Ok(())
    }
}
