use colored::Colorize;

use crate::{
    configuration::consts::CONFIGURATION_FOLDER,
    file_info::FileInfo,
    files::consts::{PLAN_FILE_NAME, SOURCE_FILE_NAME},
};
use std::{
    env,
    fs::{self, File},
    io::Write,
    path::Path,
};

// source file name will be used for rollback later when it will be added
pub fn create_source_file(files_data: &Vec<FileInfo>) {
    let home_dir: String = env::var("HOME").unwrap_or_else(|_| ".".to_string());

    let path = Path::new(home_dir.as_str())
        .join(CONFIGURATION_FOLDER)
        .join(SOURCE_FILE_NAME);

    if path.exists() {
        if let Err(err) = fs::remove_file(&path) {
            println!("Error deleting old source file: {:?}", err);
            return;
        }
    }

    match File::create(path) {
        Ok(mut file) => {
            let json_data = match serde_json::to_string_pretty(&files_data) {
                Ok(json) => json,
                Err(err) => {
                    println!("Error serializing JSON: {:?}", err);
                    return;
                }
            };

            if let Err(err) = file.write_all(json_data.as_bytes()) {
                println!("Error writing to file: {:?}", err);
            }
        }
        Err(err) => {
            println!("{:?}", err);
            panic!("{}", "Error creating source data file".red());
        }
    }
    println!("{}", "💾 Initial file metadata has been saved".green());
    println!();
}

pub fn create_plan_file(files_data: String) {
    let home_dir: String = env::var("HOME").unwrap_or_else(|_| ".".to_string());

    let path = Path::new(home_dir.as_str())
        .join(CONFIGURATION_FOLDER)
        .join(PLAN_FILE_NAME);

    if path.exists() {
        if let Err(err) = fs::remove_file(&path) {
            println!("Error deleting old plan file: {:?}", err);
            return;
        }
    }

    match File::create(path) {
        Ok(mut file) => {
            if let Err(err) = file.write_all(files_data.as_bytes()) {
                println!("Error writing to file: {:?}", err);
            }
        }
        Err(err) => println!("Error creating file: {:?}", err),
    }
    println!("{}", "💾 The new file location plan has been saved".green());
    println!();
}
