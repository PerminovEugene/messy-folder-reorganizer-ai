use crate::{
    console::messages::print_starting_apply_migrations,
    errors::app_error::AppError,
    fs::migration::{migrator::start_migrations, storage::read_migrations_config},
};

pub async fn apply_latest_migration_plan() -> Result<(), AppError> {
    let migrations_config = read_migrations_config()?;
    print_starting_apply_migrations(migrations_config.created_at.clone());
    start_migrations(migrations_config, true)?;
    Ok(())
}
