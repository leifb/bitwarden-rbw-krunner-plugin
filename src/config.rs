use log::{debug, warn};
use serde::{Deserialize, Serialize};
use std::{env, fs};

const CONFIG_PATH: &str = ".config/bitwarden-rbw-krunner/config.toml";

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct Config {
    pub prefix: String,
    pub min_length: usize,
    pub show_password: bool,
    pub command_sync: String,
    pub command_switch_profile: String,
    pub discover_profiles: bool,
    pub initial_profile: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            prefix: "pw ".to_owned(),
            min_length: 1,
            show_password: false,
            command_sync: "pws".to_owned(),
            command_switch_profile: "pwp".to_owned(),
            discover_profiles: true,
            initial_profile: "".to_owned(),
        }
    }
}

impl Config {
    pub fn load() -> Result<Config, Box<dyn std::error::Error>> {
        let path = format!("{}/{}", env::var("HOME")?, CONFIG_PATH);
        let config = fs::read_to_string(path)
            .inspect_err(|e| warn!("Failed to read config file: {}", e))
            .unwrap_or_default();

        let config = toml::from_str::<Config>(&config)
            .inspect_err(|e| warn!("Failed to parse config file: {}", e))
            .unwrap_or_default();

        config.print();
        Ok(config)
    }

    pub fn print(&self) {
        if let Ok(c) = toml::to_string(&self) {
            debug!("Config:\n{}", c);
        }
    }
}
