use crate::file::ConfigFile;
use serde_derive::Deserialize;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub master: bool,
    pub minion: bool,
    pub open_tag: String,
    pub close_tag: String,
    pub network: Network,
    pub files: Vec<ConfigFile>,
    pub polling_freq: Option<u32>,
}

#[derive(Deserialize, Debug)]
pub struct Network {
    pub bind_port: u16,
    pub bind_address: String,
}

impl Config {
    pub fn new(path: &Path) -> Self {
        match File::open(&path) {
            // There is no need to recover at this point
            // If we cannot read the config then just panic
            Err(_) => panic!("Failed to open the configuration file."),
            Ok(mut file) => {
                let mut config_file_content = String::new();
                file.read_to_string(&mut config_file_content)
                    .expect("Failed to read configuration file.");
                match toml::from_str(&config_file_content) {
                    Err(e) => {
                        // Same as above - just kill the program
                        panic!("Failed to parse config file. Error: {}", e)
                    }
                    Ok(config) => config,
                }
            }
        }
    }
}
