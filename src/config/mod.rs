use serde::Deserialize;
use std::env;
use std::path::Path;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub sites: Vec<Site>,
}

impl Default for Config {
    fn default() -> Self {
        Config { sites: vec![] }
    }
}

#[derive(Deserialize, Clone)]
pub struct Site {
    pub name: String,
    pub url: String,
}

fn get_config_path() -> String {
    let arg = env::args().nth(1);
    if arg.is_some() {
        return arg.unwrap()
    }
    let home_folder = env::var("HOME").unwrap();
    Path::new(&home_folder).
        join(".config/cordium/config.toml").to_str().unwrap().to_string()
} 

pub fn read_config() -> Config {
    let file_res = std::fs::read_to_string(get_config_path());
    if file_res.is_err() {
        return Config::default()
    }
    let c: Config = toml::from_str(file_res.unwrap().as_str()).unwrap_or_default();
    c
}
