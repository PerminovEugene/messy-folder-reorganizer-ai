use regex::Regex;
use std::fs::{self, FileType};
use std::path::Path;

use crate::configuration::ignore_list::is_ignored;
use crate::console::errors::{print_error_message, print_error_to_same_string};
use crate::console::messages::{
    print_done_to_same_string, print_ignoring_entry, print_ignoring_unix_uniq_entry_type,
    print_reading_directory_entries, print_skipped_to_same_string,
};
use crate::errors::app_error::AppError;
use crate::fs::file_info::FsEntry;

use super::config::CollectFilesMetaConfig;
use super::handlers::{handle_file_entry, handle_folder_entry};
use super::utils::get_dir_entry_and_metadata;

#[cfg(unix)]
use std::os::unix::fs::FileTypeExt;

/// Main function to collect file system entries and metadata.
pub fn collect_fs_entries_data(
    base_path: &Path,
    relative_path: &Path,
    fs_entries: &mut Vec<FsEntry>,
    ignore_patterns: &[Regex],
    config: &CollectFilesMetaConfig,
) -> Result<(), AppError> {
    let processed_path = base_path.join(relative_path);

    print_reading_directory_entries(processed_path.display());

    let read_dir_iter = match fs::read_dir(&processed_path) {
        Ok(rd) => rd,
        Err(e) => {
            if config.continue_on_fs_errors {
                print_skipped_to_same_string();
                print_error_message(e.to_string());
                return Ok(());
            } else {
                print_error_to_same_string();
                print_error_message(e.to_string());
                return Err(AppError::FileError(format!(
                    "Error reading directory {:?}: {}",
                    processed_path, e
                )));
            }
        }
    };

    print_done_to_same_string();

    for dir_entry_result in read_dir_iter {
        let Some((dir, metadata)) = get_dir_entry_and_metadata(dir_entry_result, config)? else {
            continue;
        };

        let entry_name_os = dir.file_name();
        let entry_name = entry_name_os.to_string_lossy();
        let is_file = metadata.is_file();
        let file_type = metadata.file_type();
        let is_symlink = file_type.is_symlink();

        if let Some(reason) = should_skip_entry(entry_name.as_ref(), &file_type, ignore_patterns) {
            match reason {
                SkipReason::Ignored | SkipReason::Symlink => {
                    print_ignoring_entry(
                        is_file,
                        is_symlink,
                        entry_name_os.to_string_lossy().to_string(),
                    );
                }
                SkipReason::UnixSpecial => {
                    print_ignoring_unix_uniq_entry_type(
                        entry_name_os.to_string_lossy().to_string(),
                    );
                }
            }
            continue;
        }

        if is_file {
            handle_file_entry(&entry_name_os, relative_path, metadata, config, fs_entries);
        } else {
            handle_folder_entry(&entry_name_os, relative_path, metadata, config, fs_entries);

            if config.recursive {
                let new_relative_path = relative_path.join(&entry_name_os);

                collect_fs_entries_data(
                    base_path,
                    new_relative_path.as_path(),
                    fs_entries,
                    ignore_patterns,
                    config,
                )?;
            }
        }
    }

    Ok(())
}

fn is_unix_uniq_type(file_type: &FileType) -> bool {
    #[cfg(unix)]
    {
        file_type.is_block_device()
            || file_type.is_char_device()
            || file_type.is_fifo()
            || file_type.is_socket()
    }

    #[cfg(not(unix))]
    {
        false
    }
}

enum SkipReason {
    Ignored,
    Symlink,
    UnixSpecial,
}

fn should_skip_entry(
    entry_name: &str,
    file_type: &FileType,
    ignore_patterns: &[Regex],
) -> Option<SkipReason> {
    if is_ignored(entry_name, ignore_patterns) {
        return Some(SkipReason::Ignored);
    }

    if file_type.is_symlink() {
        return Some(SkipReason::Symlink);
    }

    if is_unix_uniq_type(file_type) {
        return Some(SkipReason::UnixSpecial);
    }

    None
}
