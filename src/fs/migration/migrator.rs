use crate::console::errors::{print_error_message, print_error_to_same_string};
use crate::console::messages::print_files_reorganization_done;

use crate::errors::app_error::AppError;

use super::executor::process_migration;
use super::logger::{
    clean_up_previous_logs, save_failed_migration_log, save_successful_migration_log,
};
use super::storage::MigrationsConfig;

pub fn start_migrations(
    migrations_config: MigrationsConfig,
    continue_on_fs_errors: bool,
) -> Result<(), AppError> {
    let mut migrations = migrations_config.migrations;
    clean_up_previous_logs()?;
    for migration in migrations.iter_mut() {
        match process_migration(migration, &migrations_config.root_dir) {
            Ok(_) => save_successful_migration_log(migration)?,
            Err(e) => {
                print_error_to_same_string();

                save_failed_migration_log(migration, e.to_string())?;
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
