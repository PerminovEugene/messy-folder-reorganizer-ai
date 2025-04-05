use path_clean::PathClean;
use serde::Serialize;
use serde_json::to_string;
use std::io::Write;
use std::{
    fs::{self, OpenOptions},
    path::PathBuf,
};

use crate::console::errors::{print_error_message, print_error_to_same_string};
use crate::console::messages::print_done_to_same_string;
use crate::{
    configuration::init::{get_app_config_folder, get_migrations_log_file_path},
    console::messages::{print_files_reorganization_done, print_move_file},
    errors::app_error::AppError,
    fs::consts::PLAN_FILE_NAME as MIGRATIONS_FILE_NAME,
};

use super::fs_entry_migration::FsEntryMigration;

pub fn start_migrations(continue_on_fs_errors: bool) -> Result<(), AppError> {
    let migrations_file_path = get_app_config_folder().join(MIGRATIONS_FILE_NAME);
    let migrations_file_content = fs::read_to_string(migrations_file_path)
        .map_err(|e| AppError::FileError(format!("Error reading migrations file: {}", e)))?;

    let migrations: Vec<FsEntryMigration> = serde_json::from_str(&migrations_file_content)
        .map_err(|e| AppError::FileError(format!("Error parsing migrations file: {}", e)))?;

    clean_up_previous_logs()?;

    for migration in migrations {
        match process_migration(&migration) {
            Ok(_) => save_successfull_migration_log(&migration)?,
            Err(e) => {
                print_error_to_same_string();

                save_failed_migration_log(&migration, e.to_string())?;
                if !continue_on_fs_errors {
                    return Err(e);
                } else {
                    let message = e.to_string();
                    print_error_message(message);
                }
            }
        };
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

    print_done_to_same_string();

    Ok(())
}

#[derive(Serialize)]
struct FsEntryMigrationResult {
    from: String,
    to: String,
    status: String,
    err_message: Option<String>,
}

fn save_successfull_migration_log(migration: &FsEntryMigration) -> Result<(), AppError> {
    let result = FsEntryMigrationResult {
        from: build_migration_source_path(migration).display().to_string(),
        to: build_migration_destination_path(migration)
            .display()
            .to_string(),
        status: "success".to_string(),
        err_message: None,
    };

    let log_path: PathBuf = get_migrations_log_file_path();
    append_migration_result(&log_path, &result)
        .map_err(|e| AppError::FileError(format!("Failed to write migration success log: {}", e)))
}

fn save_failed_migration_log(
    migration: &FsEntryMigration,
    error_message: String,
) -> Result<(), AppError> {
    let result = FsEntryMigrationResult {
        from: build_migration_source_path(migration).display().to_string(),
        to: build_migration_destination_path(migration)
            .display()
            .to_string(),
        status: "failed".to_string(),
        err_message: Some(error_message),
    };

    let log_path = get_migrations_log_file_path();
    append_migration_result(&log_path, &result)
        .map_err(|e| AppError::FileError(format!("Failed to write migration failed log: {}", e)))
}

fn append_migration_result(path: &PathBuf, result: &FsEntryMigrationResult) -> std::io::Result<()> {
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;

    let line = to_string(result)?; // serialize to compact JSON
    writeln!(file, "{}", line)?; // add newline

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

pub fn clean_up_previous_logs() -> Result<(), AppError> {
    let log_path: PathBuf = get_migrations_log_file_path();

    if log_path.exists() {
        fs::remove_file(&log_path).map_err(|e| {
            AppError::FileError(format!("Failed to write migration failed log: {}", e))
        })?;
    }

    Ok(())
}
