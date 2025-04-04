use path_clean::PathClean;
use std::{fs, path::PathBuf};

use crate::{
    configuration::init::get_app_config_folder,
    console::messages::{print_files_reorganization_done, print_move_file},
    errors::app_error::AppError,
    fs::consts::PLAN_FILE_NAME as MIGRATIONS_FILE_NAME,
};

use super::fs_entry_migration::FsEntryMigration;

pub fn start_migrations() -> Result<(), AppError> {
    let plan_file_path = get_app_config_folder().join(MIGRATIONS_FILE_NAME);
    let plan_content = fs::read_to_string(plan_file_path)
        .map_err(|e| AppError::FileError(format!("Error reading migrations file: {}", e)))?;

    let migrations: Vec<FsEntryMigration> = serde_json::from_str(&plan_content)
        .map_err(|e| AppError::FileError(format!("Error parsing migrations file: {}", e)))?;

    for migration in migrations {
        process_migration(&migration)?;
    }

    print_files_reorganization_done();
    Ok(())
}

fn process_migration(migration: &FsEntryMigration) -> Result<(), AppError> {
    let source_path = build_migration_source_path(migration);
    if !source_path.exists() {
        return Err(AppError::FileError(format!(
            "Source file does not exist: {}",
            source_path.display()
        )));
    }

    let destination_path = build_migration_destination_path(migration);

    print_move_file(source_path.display(), destination_path.display());

    if let Some(parent) = destination_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| AppError::FileError(format!("Failed to create directory: {}", e)))?;
    }

    fs::rename(&source_path, &destination_path)
        .map_err(|e| AppError::FileError(format!("Failed to rename file: {}", e)))?;

    Ok(())
}

fn build_migration_source_path(migration: &FsEntryMigration) -> PathBuf {
    PathBuf::from(&migration.source)
        .join(&migration.source_inner_path)
        .join(&migration.file_name)
        .clean()
}

fn build_migration_destination_path(migration: &FsEntryMigration) -> PathBuf {
    PathBuf::from(&migration.destination)
        .join(&migration.destination_inner_path)
        .join(&migration.file_name)
        .clean()
}
