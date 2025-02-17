use std::fs;

use crate::configuration::init::get_initial_prompt_file_path;

pub fn read_initial_prompt() -> String {
    let config_path = get_initial_prompt_file_path();

    fs::read_to_string(config_path).unwrap_or_else(|_| "".to_string())
}
