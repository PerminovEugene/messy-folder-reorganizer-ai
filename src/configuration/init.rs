use std::env;
use std::fs;
use std::path::Path;

pub const IN_PROJECT_CONFIG_PATH: &str = include_str!("../../assets/config.toml");
pub const IN_PROJECT_INITIAL_PROMPT_PATH: &str =
    include_str!("../../assets/prompts/initial_sort_request.md");

pub fn init() {
    create_application_config_folder();

    let config_file_path = get_config_file_path();
    create_application_config_file(&config_file_path, IN_PROJECT_CONFIG_PATH);

    let initial_prompt_file_path = get_initial_prompt_file_path();
    create_application_config_file(&initial_prompt_file_path, IN_PROJECT_INITIAL_PROMPT_PATH);
}

pub fn get_config_file_path() -> String {
    let home_dir: String = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    format!("{}/.mess-cleaner-ai/config.toml", home_dir)
}

pub fn get_initial_prompt_file_path() -> String {
    let home_dir: String = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    format!(
        "{}/.mess-cleaner-ai/prompts/initial_sort_request.md",
        home_dir
    )
}

fn create_application_config_folder() {
    let home_dir = env::var("HOME").unwrap_or_else(|_| ".".to_string()); // Default to current dir if HOME is not found
    let config_dir = format!("{}/.mess-cleaner-ai", home_dir);
    if !Path::new(&config_dir).exists() {
        fs::create_dir_all(&config_dir).expect("Failed to create config directory");
    }

    let prompts_dir = format!("{}/.mess-cleaner-ai/prompts", home_dir);
    if !Path::new(&prompts_dir).exists() {
        fs::create_dir_all(&prompts_dir).expect("Failed to create prompts directory");
    }
}

fn create_application_config_file(config_file_path: &String, file_data: &str) {
    if !Path::new(&config_file_path).exists() {
        fs::write(config_file_path, file_data).expect("Failed to write config file");
        println!("Initialized {} file.", config_file_path);
    }
}
