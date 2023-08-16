use serde_derive::Deserialize;
use config_file::FromConfigFile;

#[derive(Deserialize)]
pub struct Config {
    pub width: usize,
    pub height: usize,
    pub spawn_rate: f64,
    pub initial_field_size: usize
}

// fn default_config() -> Config {
//     Config {
//         width: 100,
//         height: 30,
//         spawn_rate: 1.0,
//         initial_field_size: 10
//      } 
// }


impl Config {
    pub fn init_from_file() -> Self {
        let config = Config::from_config_file("./config.json").unwrap();
        config
    }
}