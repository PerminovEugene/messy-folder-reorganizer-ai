use crate::{errors::app_error::AppError, fs::migration::rollback::rollback};

pub async fn start_rollback() -> Result<(), AppError> {
    rollback()?;
    Ok(())
}
