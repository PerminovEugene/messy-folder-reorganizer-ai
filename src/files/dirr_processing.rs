use regex::Regex;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

use crate::console::messages::{
    print_ignoring_entry, print_processing_directory, print_processing_file,
};
use crate::errors::app_error::AppError;
use crate::file_info::{convert_path_meta_to_file_info, FileInfo};

#[derive(Debug)]
pub struct CollectFilesMetaConfig {
    pub skip_problematic_dir: bool,
    pub recursive: bool,
    pub process_folders: bool,
    pub process_files: bool,
}

/*
  Collects files metadata and saves it to files_data vector.
*/
pub fn collect_files_metadata(
    base_path: &Path,
    relative_path: &Path,
    files_data: &mut Vec<FileInfo>,
    ignore_patterns: &[Regex],
    config: &CollectFilesMetaConfig,
) -> Result<(), AppError> {
    let processed_path = base_path.join(relative_path);

    let read_dir_iter = match fs::read_dir(&processed_path) {
        Ok(rd) => rd,
        Err(e) => {
            eprintln!("Error reading directory {:?}: {:?}", processed_path, e);
            return Err(AppError::FileError(format!(
                "Error reading directory {:?}: {}",
                processed_path, e
            )));
        }
    };

    print_processing_directory(processed_path.display());

    for dir_entry_result in read_dir_iter {
        let Some((dir, metadata)) = get_dir_entry_and_metadata(dir_entry_result, config)? else {
            continue;
        };

        let entry_name_os = dir.file_name();

        let is_file = metadata.is_file();

        let new_relative_path = relative_path.join(&entry_name_os);

        let entry_name = entry_name_os.to_string_lossy();
        if is_ignored(entry_name.as_ref(), ignore_patterns) {
            print_ignoring_entry(is_file, &new_relative_path.display().to_string());
            continue;
        }

        if is_file {
            handle_file_entry(&entry_name_os, relative_path, metadata, config, files_data);
        } else {
            handle_folder_entry(&entry_name_os, relative_path, metadata, config, files_data);

            if config.recursive {
                collect_files_metadata(
                    base_path,
                    new_relative_path.as_path(),
                    files_data,
                    ignore_patterns,
                    config,
                )?;
            }
        }
    }

    Ok(())
}

fn is_ignored(file_path: &str, ignore_patterns: &[Regex]) -> bool {
    ignore_patterns
        .iter()
        .any(|pattern| pattern.is_match(file_path))
}

fn get_dir_entry_and_metadata(
    dir_entry_result: Result<std::fs::DirEntry, std::io::Error>,
    config: &CollectFilesMetaConfig,
) -> Result<Option<(std::fs::DirEntry, std::fs::Metadata)>, AppError> {
    let dir = match dir_entry_result {
        Ok(entry) => entry,
        Err(e) => {
            if config.skip_problematic_dir {
                eprintln!("Error reading directory entry: {:?}", e);
                return Ok(None);
            } else {
                return Err(AppError::FileError(e.to_string()));
            }
        }
    };

    let metadata = match dir.metadata() {
        Ok(m) => m,
        Err(e) => {
            if config.skip_problematic_dir {
                eprintln!("Error reading metadata for {:?}: {:?}", dir.path(), e);
                return Ok(None);
            } else {
                return Err(AppError::FileError(e.to_string()));
            }
        }
    };

    Ok(Some((dir, metadata)))
}

fn handle_file_entry(
    file_name_os: &OsStr,
    relative_path: &Path,
    metadata: std::fs::Metadata,
    config: &CollectFilesMetaConfig,
    files_data: &mut Vec<FileInfo>,
) {
    if config.process_files {
        let file_name = file_name_os.to_string_lossy().to_string();
        print_processing_file(&file_name);

        let file_info = convert_path_meta_to_file_info(file_name, relative_path, metadata, false);
        files_data.push(file_info);
    }
}

fn handle_folder_entry(
    file_name_os: &OsStr,
    relative_path: &Path,
    metadata: std::fs::Metadata,
    config: &CollectFilesMetaConfig,
    files_data: &mut Vec<FileInfo>,
) {
    if config.process_folders {
        let file_name = file_name_os.to_string_lossy().to_string();

        let file_info = convert_path_meta_to_file_info(file_name, relative_path, metadata, false);
        files_data.push(file_info);
    }
}
