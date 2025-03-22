use core::panic;
use regex::Regex;
use std::fs;
use std::path::Path;

use crate::console::messages::print_ignoring_file;
use crate::console::messages::print_ignoring_folder;
use crate::console::messages::print_processing_directory;
use crate::console::messages::print_processing_file;
use crate::file_info::convert_path_meta_to_file_info;
use crate::file_info::FileInfo;

pub struct CollectFilesMetaConfig {
    pub skip_problematic_dir: bool,
    pub recursive: bool,
    pub process_folders: bool,
    pub process_files: bool,
}

pub fn collect_files_metadata(
    base_path_str: &str,
    inner_path: &str,
    files_data: &mut Vec<FileInfo>,
    ignore_patterns: &Vec<Regex>,
    config: &CollectFilesMetaConfig,
) {
    let base_path_buf = Path::new(base_path_str).join(inner_path);
    let base_path = base_path_buf.as_path();

    match fs::read_dir(base_path) {
        Ok(read_dir_res) => {
            print_processing_directory(base_path.to_str().unwrap());

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

                let file_name = &relative_path.file_name().unwrap();

                if file_meta.is_file() {
                    if is_ignored(file_name.to_str().unwrap(), ignore_patterns) {
                        print_ignoring_file(relative_path.to_str().unwrap());
                        continue;
                    }
                    print_processing_file(file_name.to_str().unwrap());
                    if config.process_files {
                        let file_info = convert_path_meta_to_file_info(&relative_path, file_meta);
                        files_data.push(file_info);
                    }
                } else {
                    if is_ignored(file_name.to_str().unwrap(), ignore_patterns) {
                        print_ignoring_folder(relative_path.to_str().unwrap());
                        continue;
                    }
                    if config.process_folders {
                        let file_info = convert_path_meta_to_file_info(&relative_path, file_meta);
                        files_data.push(file_info);
                    }
                    if config.recursive {
                        if let Some(sub_path) = dir.file_name().to_str() {
                            let full_sub_path = Path::new(inner_path).join(sub_path);
                            let full_sub_path_str = full_sub_path.to_str().unwrap();
                            collect_files_metadata(
                                base_path_str,
                                full_sub_path_str,
                                files_data,
                                ignore_patterns,
                                config,
                            );
                        }
                    }
                }
            }
        }
        Err(err) => {
            eprintln!("Error reading directory {:?}: {:?}", base_path, err);
            if !config.skip_problematic_dir {
                panic!("Error reading directory {:?}: {:?}", base_path, err);
            }
        }
    }
}

fn is_ignored(file_path: &str, ignore_patterns: &[Regex]) -> bool {
    ignore_patterns
        .iter()
        .any(|pattern| pattern.is_match(file_path))
}
