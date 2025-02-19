use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use crate::configuration::consts::{
    CONFIGURATION_FILE, CONFIGURATION_FOLDER, INITIAL_PROMPT_FILE, PROMPTS_FOLDER,
};

pub fn init() {
    let assets = get_runtime_assets_dir();
    let in_project_config_path = assets.join(CONFIGURATION_FILE);
    let in_project_initial_prompt_path = assets.join(PROMPTS_FOLDER).join(INITIAL_PROMPT_FILE);

    create_application_config_folder();

    let config_file_path = get_config_file_path();
    create_application_config_file(&config_file_path, in_project_config_path);

    let initial_prompt_file_path = get_initial_prompt_file_path();
    create_application_config_file(&initial_prompt_file_path, in_project_initial_prompt_path);
}

/// Returns the correct path to the `assets/` directory for both `cargo run` and installed versions.
fn get_runtime_assets_dir() -> PathBuf {
    let exe_path = env::current_exe().unwrap_or_else(|_| PathBuf::from("."));

    // If running from Cargo (`target/debug/` or `target/release/`), go up twice to find `src/`
    if exe_path.to_string_lossy().contains("target/") {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")); // Root of the project
        return manifest_dir.join("assets");
    }

    // Otherwise, assume assets are in the same folder as the installed binary
    exe_path
        .parent()
        .map(|dir| dir.join("assets"))
        .unwrap_or_else(|| PathBuf::from("assets"))
}

pub fn get_config_file_path() -> PathBuf {
    let home_dir: String = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    Path::new(&home_dir)
        .join(CONFIGURATION_FOLDER)
        .join(CONFIGURATION_FILE)
}

pub fn get_initial_prompt_file_path() -> PathBuf {
    let home_dir: String = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    Path::new(&home_dir)
        .join(CONFIGURATION_FOLDER)
        .join(PROMPTS_FOLDER)
        .join(INITIAL_PROMPT_FILE)
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

fn create_application_config_file(config_file_path: &Path, source: PathBuf) {
    if !config_file_path.exists() {
        // Ensure parent directory exists
        if let Some(parent) = config_file_path.parent() {
            fs::create_dir_all(parent).expect("Failed to create parent directories");
        }

        // Copy the file
        println!("Copying from source: {:?}", source);
        println!("Copying to: {:?}", config_file_path);

        match fs::read(&source) {
            Ok(content) => {
                fs::write(config_file_path, content).expect("Failed to copy config file");
                println!("Initialized {:?} file.", config_file_path);
            }
            Err(e) => {
                println!("Warning: Could not read source file {:?} ({})", source, e);
                panic!("File one not found");
            }
        }
    }
}
