// src/fs/migration_executor.rs
use crate::console::messages::{print_done_to_same_string, print_move_file};
use crate::errors::app_error::AppError;

use path_clean::PathClean;
use std::fs;
use std::path::PathBuf;

use super::fs_entry_migration::FsEntryMigration;

pub fn process_migration(migration: &FsEntryMigration, root_dir: &PathBuf) -> Result<(), AppError> {
    let source_path = build_migration_source_path(migration, root_dir);
    if !source_path.exists() {
        return Err(AppError::FileError(format!(
            "Source file does not exist: {}",
            source_path.display()
        )));
    }

    let destination_path = build_migration_destination_path(migration, root_dir);
    print_move_file(source_path.display(), destination_path.display());

    if let Some(parent) = destination_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| AppError::FileError(format!("Failed to create directory: {}", e)))?;
    }

    fs::rename(&source_path, &destination_path)
        .map_err(|e| AppError::FileError(format!("Failed to rename file: {}", e)))?;

    print_done_to_same_string();
    Ok(())
}

pub fn build_migration_source_path(migration: &FsEntryMigration, root_dir: &PathBuf) -> PathBuf {
    PathBuf::from(root_dir)
        .join(&migration.source_arg)
        .join(&migration.source_relative_path)
        .join(&migration.source_file_name)
        .clean()
}

pub fn build_migration_destination_path(
    migration: &FsEntryMigration,
    root_dir: &PathBuf,
) -> PathBuf {
    PathBuf::from(root_dir)
        .join(&migration.destination_arg)
        .join(&migration.destination_relative_path)
        .join(&migration.source_file_name)
        .clean()
}
