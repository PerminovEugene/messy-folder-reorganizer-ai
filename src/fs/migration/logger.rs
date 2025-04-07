// src/fs/migration_logger.rs
use crate::configuration::init::get_migrations_log_file_path;
use crate::errors::app_error::AppError;

use serde::{Deserialize, Serialize};
use serde_json::to_string;
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
};

use std::fs::File;
use std::io::{BufRead, BufReader};

use super::{
    executor::{build_migration_destination_path, build_migration_source_path},
    fs_entry_migration::FsEntryMigration,
};

#[derive(Serialize, Deserialize)]
pub struct FsEntryMigrationResult {
    pub from: String,
    pub to: String,
    pub status: String,
    pub err_message: Option<String>,
}

pub fn save_successful_migration_log(migration: &FsEntryMigration) -> Result<(), AppError> {
    let result = FsEntryMigrationResult {
        from: build_migration_source_path(migration, &PathBuf::from(""))
            .display()
            .to_string(),
        to: build_migration_destination_path(migration, &PathBuf::from(""))
            .display()
            .to_string(),
        status: "success".to_string(),
        err_message: None,
    };

    let log_path: PathBuf = get_migrations_log_file_path();
    append_migration_result(&log_path, &result)
        .map_err(|e| AppError::FileError(format!("Failed to write migration success log: {}", e)))
}

pub fn save_failed_migration_log(
    migration: &FsEntryMigration,
    error_message: String,
) -> Result<(), AppError> {
    let result = FsEntryMigrationResult {
        from: build_migration_source_path(migration, &PathBuf::from(""))
            .display()
            .to_string(),
        to: build_migration_destination_path(migration, &PathBuf::from(""))
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
    let line = to_string(result)?;
    writeln!(file, "{}", line)?;
    Ok(())
}

pub fn clean_up_previous_logs() -> Result<(), AppError> {
    let log_path: PathBuf = get_migrations_log_file_path();

    if log_path.exists() {
        fs::remove_file(&log_path).map_err(|e| {
            AppError::FileError(format!("Failed to delete previous migration logs: {}", e))
        })?;
    }

    Ok(())
}

pub fn read_migration_log() -> std::io::Result<Vec<FsEntryMigrationResult>> {
    let path: PathBuf = get_migrations_log_file_path();
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut entries = Vec::new();
    for line_result in reader.lines() {
        let line = line_result?;
        match serde_json::from_str::<FsEntryMigrationResult>(&line) {
            Ok(entry) => entries.push(entry),
            Err(e) => {
                eprintln!("Failed to parse migration log line: {e}");
                continue;
            }
        }
    }

    Ok(entries)
}
