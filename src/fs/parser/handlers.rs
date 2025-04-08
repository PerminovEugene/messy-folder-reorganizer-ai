use std::ffi::OsStr;
use std::path::Path;

use crate::console::messages::{
    print_done_to_same_string, print_processing_directory, print_processing_file,
};
use crate::fs::file_info::{build_fs_entry, FsEntry};

use super::config::CollectFilesMetaConfig;

pub fn handle_file_entry(
    file_name_os: &OsStr,
    relative_path: &Path,
    metadata: std::fs::Metadata,
    config: &CollectFilesMetaConfig,
    files_data: &mut Vec<FsEntry>,
) {
    if config.process_files {
        let file_name = file_name_os.to_string_lossy().to_string();
        print_processing_file(&file_name);

        let file_info = build_fs_entry(file_name, relative_path, metadata, false);
        files_data.push(file_info);
        print_done_to_same_string();
    }
}

pub fn handle_folder_entry(
    file_name_os: &OsStr,
    relative_path: &Path,
    metadata: std::fs::Metadata,
    config: &CollectFilesMetaConfig,
    files_data: &mut Vec<FsEntry>,
) {
    if config.process_folders {
        let path: std::path::PathBuf = relative_path.join(file_name_os);
        print_processing_directory(path.display());

        let file_name = file_name_os.to_string_lossy().to_string();

        let file_info = build_fs_entry(file_name, relative_path, metadata, false);
        files_data.push(file_info);
        print_done_to_same_string();
    }
}
