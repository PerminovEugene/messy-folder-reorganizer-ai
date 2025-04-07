use crate::fs::consts::MIGRATIONS_PLAN_FILE;
use std::{fs, path::PathBuf};

use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::{
    configuration::init::{get_app_config_folder, rewrite_app_system_file},
    console::messages::print_migration_plan_saved,
    errors::app_error::AppError,
};

use super::fs_entry_migration::FsEntryMigration;

pub fn save_migrations_to_file(migraions: Vec<FsEntryMigration>) -> Result<(), AppError> {
    let migrations_config = build_migrations_config(migraions)?;
    let string_data = serde_json::to_string_pretty(&migrations_config).unwrap();
    rewrite_app_system_file(MIGRATIONS_PLAN_FILE, string_data);
    print_migration_plan_saved();
    Ok(())
}

#[derive(Serialize, Deserialize)]
pub struct MigrationsConfig {
    pub created_at: String,
    pub root_dir: PathBuf,
    pub migrations: Vec<FsEntryMigration>,
}

fn build_migrations_config(
    migrations: Vec<FsEntryMigration>,
) -> Result<MigrationsConfig, AppError> {
    let root_dir = std::env::current_dir().map_err(|e| AppError::FileError(e.to_string()))?;
    let created_at = Utc::now().to_rfc3339();

    Ok(MigrationsConfig {
        migrations,
        root_dir,
        created_at,
    })
}

pub fn read_migrations_config() -> Result<MigrationsConfig, AppError> {
    let migrations_file_path = get_app_config_folder().join(MIGRATIONS_PLAN_FILE);
    let migrations_file_content = fs::read_to_string(migrations_file_path)
        .map_err(|e| AppError::FileError(format!("Error reading migrations file: {}", e)))?;

    let config: MigrationsConfig = serde_json::from_str(&migrations_file_content)
        .map_err(|e| AppError::FileError(format!("Error parsing migrations file: {}", e)))?;

    Ok(config)
}
