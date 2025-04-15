use crate::{
    configuration::init::{get_app_migrations_folder, rewrite_app_system_path},
    fs::consts::{MIGRATIONS_PLAN_FILE_FORMAT, MIGRATIONS_PLAN_FILE_NAME_BASE},
};
use std::{fs, path::PathBuf};

use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::{console::messages::print_migration_plan_saved, errors::app_error::AppError};

use super::fs_entry_migration::FsEntryMigration;

pub fn save_migrations_to_file(
    migraions: Vec<FsEntryMigration>,
    session_id: &String,
) -> Result<(), AppError> {
    let migrations_config = build_migrations_config(migraions)?;
    let migrations_string_data = serde_json::to_string_pretty(&migrations_config).unwrap();
    let migrations_file_name = build_migration_file_name(session_id);
    let migrations_path = get_app_migrations_folder().join(&migrations_file_name);

    rewrite_app_system_path(&migrations_path, migrations_string_data);
    print_migration_plan_saved(&migrations_file_name);
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

pub fn read_migrations_config(session_id: &String) -> Result<MigrationsConfig, AppError> {
    let name = build_migration_file_name(session_id);
    let migrations_file_path = get_app_migrations_folder().join(name);
    let migrations_file_content = fs::read_to_string(migrations_file_path)
        .map_err(|e| AppError::FileError(format!("Error reading migrations file: {}", e)))?;

    let config: MigrationsConfig = serde_json::from_str(&migrations_file_content)
        .map_err(|e| AppError::FileError(format!("Error parsing migrations file: {}", e)))?;

    Ok(config)
}

fn build_migration_file_name(session_id: &String) -> String {
    format!(
        "{}_{}.{}",
        MIGRATIONS_PLAN_FILE_NAME_BASE, session_id, MIGRATIONS_PLAN_FILE_FORMAT
    )
}
