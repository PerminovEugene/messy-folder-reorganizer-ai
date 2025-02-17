use std::env;
use std::fs;
use std::path::Path;

const DEFAULT_CONFIG: &str = include_str!("../../assets/config.toml");

pub fn init() {
    // Get user's home directory
    let home_dir = env::var("HOME").unwrap_or_else(|_| ".".to_string()); // Default to current dir if HOME is not found
    let config_dir = format!("{}/.mess-cleaner-ai", home_dir);
    let config_file_path = format!("{}/config.toml", config_dir);

    // Check if the directory exists, if not, create it
    if !Path::new(&config_dir).exists() {
        fs::create_dir_all(&config_dir).expect("Failed to create config directory");
    }

    // Check if the config file exists, if not, create it with default values
    if !Path::new(&config_dir).exists() {
        fs::create_dir_all(&config_dir).expect("Failed to create config directory");
    }

    // Write the config file only if it does not already exist
    if !Path::new(&config_file_path).exists() {
        fs::write(&config_file_path, DEFAULT_CONFIG).expect("Failed to write config file");
        println!("Config initialized at {}", config_file_path);
    } else {
        println!("Config already exists at {}", config_file_path);
    }
    println!();
}
