use std::fs;
use std::path::{Path, PathBuf};

use crate::configuration::consts::{
    CONFIGURATION_FOLDER, EMBEDDINGS_GENERATION_CONFIGURATION_FILE, INITIAL_PROMPT_FILE,
    LLM_CONFIGURATION_FILE, PROMPTS_FOLDER, RAG_ML_CONFIGURATION_FILE,
};
use crate::configuration::embedded_assets::{
    EMBEDDINGS_MODEL_CONFIG_FILE_BYTES, GENERATE_FOLDER_NAME_PROMPT_FILE_BYTES,
    LLM_MODEL_CONFIG_FILE_BYTES, RAG_ML_CONFIG_FILE_BYTES,
};
use crate::console::messages::print_generate_config_file;
use crate::files::path::get_home_path;

/*
  Initializes the application configuration by creating:
    .app-name-folder at home path,
    copying config toml files and prompts from assets to .app-name-folder
  if these files are missing
*/
pub fn init() {
    create_application_config_folder();

    let embedding_config_file_path = get_config_file_path(EMBEDDINGS_GENERATION_CONFIGURATION_FILE); // e.g. ~/.messy-folder-reorganizer-ai/config.toml
    create_application_file_if_missing(
        &embedding_config_file_path,
        EMBEDDINGS_MODEL_CONFIG_FILE_BYTES,
    );

    let llm_config_file_path = get_config_file_path(LLM_CONFIGURATION_FILE);
    create_application_file_if_missing(&llm_config_file_path, LLM_MODEL_CONFIG_FILE_BYTES);

    let rag_ml_config_file_path = get_config_file_path(RAG_ML_CONFIGURATION_FILE);
    create_application_file_if_missing(&rag_ml_config_file_path, RAG_ML_CONFIG_FILE_BYTES);

    let generate_folder_prompt_file_path = get_generate_folder_prompt_file_path(); // e.g. ~/.messy-folder-reorganizer-ai/prompts/prompt.txt
    create_application_file_if_missing(
        &generate_folder_prompt_file_path,
        GENERATE_FOLDER_NAME_PROMPT_FILE_BYTES,
    );
}

fn create_application_config_folder() {
    let app_config_dir = get_app_config_folder();
    if !app_config_dir.exists() {
        fs::create_dir_all(app_config_dir).expect("Failed to create config directory");
    }

    let app_prompts_config_dir = get_app_prompt_config_folder();
    if !app_prompts_config_dir.exists() {
        fs::create_dir_all(&app_prompts_config_dir).expect("Failed to create prompts directory");
    }
}

pub fn get_app_config_folder() -> PathBuf {
    get_home_path().join(CONFIGURATION_FOLDER)
}

pub fn get_app_prompt_config_folder() -> PathBuf {
    get_home_path()
        .join(CONFIGURATION_FOLDER)
        .join(PROMPTS_FOLDER)
}

pub fn get_config_file_path(file_name: &str) -> PathBuf {
    get_app_config_folder().join(file_name)
}

pub fn get_generate_folder_prompt_file_path() -> PathBuf {
    get_app_prompt_config_folder().join(INITIAL_PROMPT_FILE)
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
