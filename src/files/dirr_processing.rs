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
    base_path_str: &Path,           // destination or source path provided in args
    inner_path: &str,               // built during recursive calls. Use "./" for root call
    files_data: &mut Vec<FileInfo>, // result vector
    ignore_patterns: &Vec<Regex>,
    config: &CollectFilesMetaConfig,
) {
    let processed_path_buf = base_path_str.join(inner_path);
    let processed_path = processed_path_buf.as_path();

    match fs::read_dir(processed_path) {
        Ok(read_dir_res) => {
            print_processing_directory(processed_path.to_str().unwrap());

            for dir in read_dir_res.flatten() {
                let metadata = match dir.metadata() {
                    Ok(meta) => meta,
                    Err(err) => {
                        eprintln!("Error reading metadata for {:?}: {:?}", dir.path(), err);
                        continue;
                    }
                };

                let absolute_path = dir.path(); // begining from destination path in arg
                let full_relative_path = match absolute_path.strip_prefix(base_path_str) {
                    Ok(p) => p.to_path_buf(),
                    Err(_) => absolute_path.to_path_buf(),
                };
                let relative_path = inner_path.to_string();

                let file_name = full_relative_path
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown");

                if metadata.is_file() {
                    if is_ignored(file_name, ignore_patterns) {
                        print_ignoring_file(full_relative_path.to_str().unwrap());
                        continue;
                    }

                    print_processing_file(file_name);

                    if config.process_files {
                        let file_info = convert_path_meta_to_file_info(
                            &full_relative_path,
                            relative_path,
                            metadata,
                            false,
                        );
                        files_data.push(file_info);
                    }
                } else {
                    if is_ignored(file_name, ignore_patterns) {
                        print_ignoring_folder(full_relative_path.to_str().unwrap());
                        continue;
                    }

                    if config.process_folders {
                        let file_info = convert_path_meta_to_file_info(
                            &full_relative_path,
                            relative_path,
                            metadata,
                            false,
                        );
                        files_data.push(file_info);
                    }

                    if config.recursive {
                        if let Some(currently_processing_dir) = dir.file_name().to_str() {
                            let full_sub_path =
                                Path::new(inner_path).join(currently_processing_dir);
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
            eprintln!("Error reading directory {:?}: {:?}", processed_path, err);
            if !config.skip_problematic_dir {
                panic!("Error reading directory {:?}: {:?}", processed_path, err);
            }
        }
    }
}

fn is_ignored(file_path: &str, ignore_patterns: &[Regex]) -> bool {
    ignore_patterns
        .iter()
        .any(|pattern| pattern.is_match(file_path))
}
