use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use crate::configuration::consts::CONFIGURATION_FILE;
use crate::configuration::consts::CONFIGURATION_FOLDER;
use crate::configuration::consts::INITIAL_PROMPT_FILE;
use crate::configuration::consts::PROMPTS_FOLDER;

pub fn init() {
    let assets = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");
    let in_project_config_path = assets.join(CONFIGURATION_FILE);
    let in_project_initial_prompt_path = assets.join(PROMPTS_FOLDER).join(INITIAL_PROMPT_FILE);

    create_application_config_folder();

    let config_file_path = get_config_file_path();
    create_application_config_file(&config_file_path, in_project_config_path);

    let initial_prompt_file_path = get_initial_prompt_file_path();
    create_application_config_file(&initial_prompt_file_path, in_project_initial_prompt_path);
}

pub fn get_config_file_path() -> PathBuf {
    let home_dir: String = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let config_path = Path::new(home_dir.as_str())
        .join(CONFIGURATION_FOLDER)
        .join(CONFIGURATION_FILE);
    config_path
}

pub fn get_initial_prompt_file_path() -> PathBuf {
    let home_dir: String = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let promp_path = Path::new(home_dir.as_str())
        .join(CONFIGURATION_FOLDER)
        .join(PROMPTS_FOLDER)
        .join(INITIAL_PROMPT_FILE);
    promp_path
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

fn create_application_config_file(config_file_path: &Path, source: PathBuf) {
    if !config_file_path.exists() {
        // Ensure parent directory exists
        if let Some(parent) = config_file_path.parent() {
            fs::create_dir_all(parent).expect("Failed to create parent directories");
        }

        // Copy the file
        println!("{:?}", source);
        println!("{:?}", std::env::current_dir());
        let content = fs::read(source).unwrap();
        fs::write(config_file_path, content).expect("Failed to copy config file");

        println!("Initialized {:?} file.", config_file_path);
    }
}
