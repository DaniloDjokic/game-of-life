use serde_derive::Deserialize;
use config_file::FromConfigFile;

#[derive(Deserialize)]
pub struct Config {
    pub width: usize,
    pub height: usize,
    pub spawn_rate: f64,
    pub initial_field_size: usize
}

impl Config {
    pub fn init_from_file() -> Self {
        let config = Config::from_config_file("./config.toml").unwrap();
        config
    }
}