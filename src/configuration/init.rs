use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use crate::configuration::consts::{
    CONFIGURATION_FILE, CONFIGURATION_FOLDER, INITIAL_PROMPT_FILE, PROMPTS_FOLDER,
};
use crate::configuration::embedded_assets::{CONFIG_FILE_BYTES, INITIAL_PROMPT_FILE_BYTES};
use crate::console::messages::print_generate_config_file;

pub fn init() {
    create_application_config_folder();

    let config_file_path = get_config_file_path(); // e.g. ~/.messy-folder-reorganizer-ai/config.toml
    let initial_prompt_file_path = get_initial_prompt_file_path(); // e.g. ~/.messy-folder-reorganizer-ai/prompts/initial_prompt.txt

    create_application_file_if_missing(&config_file_path, CONFIG_FILE_BYTES);
    create_application_file_if_missing(&initial_prompt_file_path, INITIAL_PROMPT_FILE_BYTES);
}

fn create_application_config_folder() {
    let home_dir = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let config_dir = format!("{}/.messy-folder-reorganizer-ai", home_dir);
    if !Path::new(&config_dir).exists() {
        fs::create_dir_all(&config_dir).expect("Failed to create config directory");
    }

    let prompts_dir = format!("{}/.messy-folder-reorganizer-ai/prompts", home_dir);
    if !Path::new(&prompts_dir).exists() {
        fs::create_dir_all(&prompts_dir).expect("Failed to create prompts directory");
    }
}

pub fn get_config_file_path() -> PathBuf {
    let home_dir = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    Path::new(&home_dir)
        .join(CONFIGURATION_FOLDER)
        .join(CONFIGURATION_FILE)
}

pub fn get_initial_prompt_file_path() -> PathBuf {
    let home_dir = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    Path::new(&home_dir)
        .join(CONFIGURATION_FOLDER)
        .join(PROMPTS_FOLDER)
        .join(INITIAL_PROMPT_FILE)
}

fn create_application_file_if_missing(config_file_path: &Path, embedded_content: &[u8]) {
    if !config_file_path.exists() {
        if let Some(parent) = config_file_path.parent() {
            fs::create_dir_all(parent).expect("Failed to create parent directories");
        }

        fs::write(config_file_path, embedded_content).expect("Failed to write embedded content");
        print_generate_config_file(config_file_path.to_str().unwrap().to_string());
    }
}
