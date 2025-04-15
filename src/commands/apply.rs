use crate::{
    configuration::args::ApplyArgs,
    console::messages::print_starting_apply_migrations,
    errors::app_error::AppError,
    fs::migration::{migrator::start_migrations, storage::read_migrations_config},
};

pub async fn apply_latest_migration_plan(args: ApplyArgs) -> Result<(), AppError> {
    let migrations_config = read_migrations_config(&args.session_id)?;
    print_starting_apply_migrations(migrations_config.created_at.clone());
    start_migrations(migrations_config, true)?;
    Ok(())
}
