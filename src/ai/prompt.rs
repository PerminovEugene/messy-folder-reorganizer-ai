use std::fs;

use crate::configuration::{consts::INITIAL_PROMPT_FILE, init::get_app_prompts_folder_path};

pub fn read_prompt() -> String {
    let config_path = get_app_prompts_folder_path().join(INITIAL_PROMPT_FILE);

    fs::read_to_string(config_path).unwrap_or_else(|_| "".to_string())
}
