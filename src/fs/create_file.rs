use colored::Colorize;

use crate::{
    configuration::init::get_app_config_folder,
    console::messages::print_source_files_metadata_saved, file_info::FsEntry,
    fs::consts::SOURCE_FILE_NAME,
};
use std::{
    fs::{self, File},
    io::Write,
};

// source file name will be used for checking files migration quality later
pub fn create_source_file(files_data: &Vec<FsEntry>) {
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
