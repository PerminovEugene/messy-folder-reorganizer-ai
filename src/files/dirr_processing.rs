use colored::Colorize;
use core::panic;
use std::fs;
use std::path::Path;

use crate::file_info::convert_path_meta_to_file_info;
use crate::file_info::FileInfo;

pub fn fill_up_files_data_by_path(
    base_path_str: &str,
    inner_path: &str,
    recursive: bool,
    skip_problematic_dir: bool,
    files_data: &mut Vec<FileInfo>,
) {
    let base_path_buf = Path::new(base_path_str).join(inner_path);
    let base_path = base_path_buf.as_path();

    match fs::read_dir(base_path) {
        Ok(read_dir_res) => {
            println!("{} {:?}", "🔍 Processing directory:".green(), base_path);

            for dir in read_dir_res.flatten() {
                let file_meta = match dir.metadata() {
                    Ok(meta) => meta,
                    Err(err) => {
                        eprintln!("Error reading metadata for {:?}: {:?}", dir.path(), err);
                        continue;
                    }
                };

                // Compute relative path from base_path, ensuring it is correctly stored
                let relative_path = match dir.path().strip_prefix(base_path_str) {
                    Ok(p) => p.to_path_buf(),           // Convert &Path to PathBuf
                    Err(_) => dir.path().to_path_buf(), // Keep full path if stripping fails
                };

                if file_meta.is_file() {
                    let file_info = convert_path_meta_to_file_info(&relative_path, file_meta);
                    files_data.push(file_info);
                } else if recursive {
                    if let Some(sub_path) = dir.file_name().to_str() {
                        let full_sub_path = Path::new(inner_path).join(sub_path);
                        let full_sub_path_str = full_sub_path.to_str().unwrap();
                        fill_up_files_data_by_path(
                            base_path_str,
                            full_sub_path_str,
                            recursive,
                            skip_problematic_dir,
                            files_data,
                        );
                    }
                }
            }
        }
        Err(err) => {
            eprintln!("Error reading directory {:?}: {:?}", base_path, err);
            if !skip_problematic_dir {
                panic!("Error reading directory {:?}: {:?}", base_path, err);
            }
        }
    }
    println!();
}
