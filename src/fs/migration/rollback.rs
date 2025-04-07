use std::path::PathBuf;

use crate::console::messages::{
    print_done_to_same_string, print_file_not_found, print_move_file,
    print_skipped_failed_migration, print_skipped_to_same_string,
};
use crate::errors::app_error::AppError;
use crate::fs::migration::logger::read_migration_log;

pub fn rollback() -> Result<(), AppError> {
    let entries = read_migration_log().map_err(|e| {
        AppError::FileError(format!("Failed to read migration log for rollback: {e}"))
    })?;

    for entry in entries.iter().rev() {
        let from = PathBuf::from(&entry.from);
        let to = PathBuf::from(&entry.to);
        print_move_file(to.display(), from.display());
        if entry.status == "success" {
            if to.exists() {
                std::fs::create_dir_all(from.parent().unwrap()).ok();
                std::fs::rename(&to, &from).map_err(|e| {
                    AppError::FileError(format!("Failed to rollback file {}: {e}", to.display()))
                })?;
                print_done_to_same_string();
            } else {
                print_skipped_to_same_string();
                print_file_not_found(to.display());
            }
        } else {
            print_skipped_to_same_string();
            print_skipped_failed_migration(from.display(), to.display());
        }
    }

    Ok(())
}
