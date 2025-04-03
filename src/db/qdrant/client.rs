use qdrant_client::Qdrant;

use crate::errors::app_error::AppError;

pub async fn init(address: &str) -> Result<Qdrant, AppError> {
    let client = Qdrant::from_url(address).build()?;
    Ok(client)
}
