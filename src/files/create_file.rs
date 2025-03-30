use colored::Colorize;

use crate::{
    configuration::init::get_app_config_folder,
    console::messages::{print_migration_plan_saved, print_source_files_metadata_saved},
    file_info::FileInfo,
    files::consts::{PLAN_FILE_NAME, SOURCE_FILE_NAME},
};
use std::{
    fs::{self, File},
    io::Write,
};

use super::file_info::FilesReorganisationPlan;

// source file name will be used for rollback later when it will be added
pub fn create_source_file(files_data: &Vec<FileInfo>) {
    let path = get_app_config_folder().join(SOURCE_FILE_NAME);

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
    print_source_files_metadata_saved()
}

pub fn create_plan_file(files_data: String) {
    let path = get_app_config_folder().join(PLAN_FILE_NAME);

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
    print_migration_plan_saved();
}

pub fn save_files_reorganisation_plan(files_data: Vec<FilesReorganisationPlan>) {
    let string_data = serde_json::to_string_pretty(&files_data).unwrap();
    create_plan_file(string_data);
}
