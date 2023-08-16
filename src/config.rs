use serde_derive::Deserialize;
use config_file::FromConfigFile;

#[derive(Deserialize)]
pub struct Config {
    pub width: usize,
    pub height: usize,
    pub initial_field_size: usize,
    pub seed: String
}

impl Config {
    pub fn init_from_file() -> Self {
        let mut config = Config::from_config_file("./config.toml").unwrap();
        
        if config.seed.len() > 32 {
            config.seed = String::from(truncate(&config.seed, 32));
        }
        else if config.seed.len() < 32 {
            while config.seed.len() < 32 {
                config.seed.push_str("0");
            }
        }

        config
    }
}

fn truncate(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => &s[..idx],
    }
}