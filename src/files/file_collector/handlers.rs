use std::ffi::OsStr;
use std::path::Path;

use crate::{
    console::messages::print_processing_file,
    file_info::{convert_path_meta_to_file_info, FileInfo},
};

use super::config::CollectFilesMetaConfig;

pub fn handle_file_entry(
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

pub fn handle_folder_entry(
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
