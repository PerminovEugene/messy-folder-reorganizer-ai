use crate::{
    configuration::init::rewrite_app_system_file, console::messages::print_migration_plan_saved,
    fs::consts::PLAN_FILE_NAME,
};

use super::fs_entry_migration::FsEntryMigration;

pub fn save_migrations_to_file(files_data: Vec<FsEntryMigration>) {
    let string_data = serde_json::to_string_pretty(&files_data).unwrap();
    rewrite_app_system_file(PLAN_FILE_NAME, string_data);
    print_migration_plan_saved();
}
