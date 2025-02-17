use std::fs;

use super::{config::Config, init::get_config_file_path};

pub fn read_config() -> Config {
    let config_path = get_config_file_path();

    let toml_content = fs::read_to_string(config_path).unwrap_or_else(|_| "".to_string());

    let config: Config = toml::from_str(&toml_content).unwrap_or_else(|_| Config::default());

    config
}
