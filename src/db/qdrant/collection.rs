use qdrant_client::qdrant::{
    CreateCollectionBuilder, Distance, ScalarQuantizationBuilder, VectorParamsBuilder,
};
use qdrant_client::Qdrant;

use crate::errors::app_error::AppError;

pub async fn safe_create_collection(
    // TODO: add retry attempts if needed
    client: &Qdrant,
    collection_name: &str,
    dimensions: u64,
) -> Result<(), AppError> {
    let is_collection_exist = client.collection_exists(collection_name).await?;
    if !is_collection_exist {
        match client
            .create_collection(
                CreateCollectionBuilder::new(collection_name)
                    .vectors_config(VectorParamsBuilder::new(dimensions, Distance::Cosine))
                    .quantization_config(ScalarQuantizationBuilder::default()),
            )
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("{:?}", e);
                if e.to_string().contains("already exists") {
                    Ok(())
                } else {
                    // wrap original error for debugging
                    Err(AppError::QdrantCustom(e.to_string()))
                }
            }
        }
    } else {
        Ok(())
    }
}
