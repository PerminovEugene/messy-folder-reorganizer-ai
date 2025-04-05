use crate::configuration::args::ProcessArgs;
use crate::console::messages::{ask_for_files_migration, print_files_not_updated};
use crate::errors::app_error::AppError;
use crate::fs::migration::runner::start_migrations;

pub fn migrate_files(args: &ProcessArgs) -> Result<(), AppError> {
    if args.force_apply || ask_for_files_migration() {
        start_migrations(args.continue_on_fs_errors)?;
    } else {
        print_files_not_updated();
    }
    Ok(())
}
