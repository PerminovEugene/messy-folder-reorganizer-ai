use regex::Regex;
use std::fs;
use std::path::Path;

use crate::console::messages::{print_ignoring_entry, print_processing_directory};
use crate::errors::app_error::AppError;
use crate::file_info::FileInfo;

use super::config::CollectFilesMetaConfig;
use super::handlers::{handle_file_entry, handle_folder_entry};
use super::utils::{get_dir_entry_and_metadata, is_ignored};

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
