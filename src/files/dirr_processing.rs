use colored::Colorize;
use core::panic;
use regex::Regex;
use std::fs;
use std::path::Path;

use crate::file_info::convert_path_meta_to_file_info;
use crate::file_info::FileInfo;

pub fn collect_files_metadata(
    base_path_str: &str,
    inner_path: &str,
    skip_problematic_dir: bool,
    files_data: &mut Vec<FileInfo>,
    ignore_patterns: &Vec<Regex>,
    recursive: bool,
    process_folders: bool,
    process_files: bool,
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

                let file_name = &relative_path.file_name().unwrap();
                println!("{} {:?}", "📄 Processing file:".blue(), file_name);
                if is_ignored(file_name.to_str().unwrap(), &ignore_patterns) {
                    println!("{} {:?}", "🚫 Ignoring file:".yellow(), relative_path);
                    continue;
                }

                if file_meta.is_file() {
                    if process_files {
                        let file_info = convert_path_meta_to_file_info(&relative_path, file_meta);
                        files_data.push(file_info);
                    }
                } else {
                    if process_folders {
                        let file_info = convert_path_meta_to_file_info(&relative_path, file_meta);
                        files_data.push(file_info);
                    }
                    if recursive {
                        if let Some(sub_path) = dir.file_name().to_str() {
                            let full_sub_path = Path::new(inner_path).join(sub_path);
                            let full_sub_path_str = full_sub_path.to_str().unwrap();
                            collect_files_metadata(
                                base_path_str,
                                full_sub_path_str,
                                skip_problematic_dir,
                                files_data,
                                ignore_patterns,
                                recursive,
                                process_folders,
                                process_files,
                            );
                        }
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

fn is_ignored(file_path: &str, ignore_patterns: &[Regex]) -> bool {
    ignore_patterns
        .iter()
        .any(|pattern| pattern.is_match(file_path))
}
