use std::fs;

use crate::file_info::convert_path_meta_to_file_info;
use crate::file_info::FileInfo;

pub fn fill_up_files_data_by_path(path: &str, files_data: &mut Vec<FileInfo>) {
    match fs::read_dir(path) {
        Ok(read_dir_res) => {
            for dir in read_dir_res.flatten() {
                let file_meta = match dir.metadata() {
                    Ok(meta) => meta,
                    Err(err) => {
                        println!("Error reading metadata for {:?}: {:?}", dir.path(), err);
                        continue;
                    }
                };
                if file_meta.is_file() {
                    let file_info = convert_path_meta_to_file_info(&dir.path(), file_meta);
                    files_data.push(file_info);
                } else {
                    println!("Processing directory: {:?}", dir.path());
                    fill_up_files_data_by_path(dir.path().to_str().unwrap(), files_data);
                }
            }
        }
        Err(err) => {
            println!("Error reading directory {:?}: {:?}", path, err);
        }
    }
}
