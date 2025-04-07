// src/fs/migrator.rs
use crate::console::errors::{print_error_message, print_error_to_same_string};
use crate::console::messages::print_files_reorganization_done;

use crate::configuration::init::get_app_config_folder;
use crate::errors::app_error::AppError;
use crate::fs::consts::PLAN_FILE_NAME as MIGRATIONS_FILE_NAME;

use std::fs;

use super::executor::process_migration;
use super::fs_entry_migration::FsEntryMigration;
use super::logger::{
    clean_up_previous_logs, save_failed_migration_log, save_successful_migration_log,
};

pub fn start_migrations(continue_on_fs_errors: bool) -> Result<(), AppError> {
    let migrations_file_path = get_app_config_folder().join(MIGRATIONS_FILE_NAME);
    let migrations_file_content = fs::read_to_string(migrations_file_path)
        .map_err(|e| AppError::FileError(format!("Error reading migrations file: {}", e)))?;

    let migrations: Vec<FsEntryMigration> = serde_json::from_str(&migrations_file_content)
        .map_err(|e| AppError::FileError(format!("Error parsing migrations file: {}", e)))?;

    clean_up_previous_logs()?;

    for migration in migrations {
        match process_migration(&migration) {
            Ok(_) => save_successful_migration_log(&migration)?,
            Err(e) => {
                print_error_to_same_string();

                save_failed_migration_log(&migration, e.to_string())?;
                if !continue_on_fs_errors {
                    return Err(e);
                } else {
                    print_error_message(e.to_string());
                }
            }
        };
    }

    print_files_reorganization_done();
    Ok(())
}
