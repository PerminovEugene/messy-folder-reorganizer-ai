use crate::{
    console::messages::print_starting_rollack,
    errors::app_error::AppError,
    fs::migration::{rollback::rollback, storage::read_migrations_config},
};

pub async fn start_rollback() -> Result<(), AppError> {
    let config = read_migrations_config()?;
    print_starting_rollack(config.created_at);
    rollback()?;
    Ok(())
}
