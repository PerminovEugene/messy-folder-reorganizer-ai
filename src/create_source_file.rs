use crate::file_info::FileInfo;
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

pub fn create_source_file(files_data: &Vec<FileInfo>) {
    let file_name = "./source.json";
    let path = Path::new(file_name);

    if path.exists() {
        if let Err(err) = fs::remove_file(path) {
            println!("Error deleting old source file: {:?}", err);
            return;
        }
    }

    match File::create(file_name) {
        Ok(mut file) => {
            println!("File created: {}", file_name);
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
        Err(err) => println!("Error creating file: {:?}", err),
    }
    println!("Source files data is saved");
}
