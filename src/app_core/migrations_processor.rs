use crate::console::messages::{ask_for_files_migration, print_files_not_updated};
use crate::errors::app_error::AppError;
use crate::fs::migration::migrator::start_migrations;
use crate::fs::migration::storage::read_migrations_config;

pub fn migrate_files(
    force_apply: bool,
    continue_on_fs_errors: bool,
    session_id: &String,
) -> Result<(), AppError> {
    if force_apply || ask_for_files_migration() {
        let migrations_config = read_migrations_config(session_id)?;
        start_migrations(migrations_config, continue_on_fs_errors, session_id)?;
    } else {
        print_files_not_updated();
    }
    Ok(())
}
