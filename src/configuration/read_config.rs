use std::fs;

use super::config::Config;

pub fn read_config() -> Config {
    let config_path = "config.toml";

    // Read the file, return empty string if not found
    let toml_content = fs::read_to_string(config_path).unwrap_or_else(|_| "".to_string());

    // Parse the TOML file, filling in defaults for missing fields
    let config: Config = toml::from_str(&toml_content).unwrap_or_else(|_| Config::default());

    config
}
