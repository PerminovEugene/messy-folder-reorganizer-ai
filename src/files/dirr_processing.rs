use std::fs;
use std::path::Path;

use crate::file_info::convert_path_meta_to_file_info;
use crate::file_info::FileInfo;

pub fn fill_up_files_data_by_path(
    base_path: &str,
    recursive: bool,
    files_data: &mut Vec<FileInfo>,
) {
    let base_path = Path::new(base_path);

    match fs::read_dir(base_path) {
        Ok(read_dir_res) => {
            for dir in read_dir_res.flatten() {
                let file_meta = match dir.metadata() {
                    Ok(meta) => meta,
                    Err(err) => {
                        eprintln!("Error reading metadata for {:?}: {:?}", dir.path(), err);
                        continue;
                    }
                };

                // Compute relative path from base_path, ensuring it is correctly stored
                let relative_path = match dir.path().strip_prefix(base_path) {
                    Ok(p) => p.to_path_buf(),           // Convert &Path to PathBuf
                    Err(_) => dir.path().to_path_buf(), // Keep full path if stripping fails
                };

                if file_meta.is_file() {
                    let file_info = convert_path_meta_to_file_info(&relative_path, file_meta);
                    files_data.push(file_info);
                } else if recursive {
                    println!("Processing directory: {:?}", relative_path);
                    if let Some(sub_path) = dir.path().to_str() {
                        fill_up_files_data_by_path(sub_path, recursive, files_data);
                    }
                }
            }
        }
        Err(err) => {
            eprintln!("Error reading directory {:?}: {:?}", base_path, err);
        }
    }
}
