use crate::{app_core::rollback_processor::rollback, errors::app_error::AppError};

pub async fn start_rollback() -> Result<(), AppError> {
    rollback();
    Ok(())
}
