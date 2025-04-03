use qdrant_client::qdrant::{
    CreateCollectionBuilder, Distance, ScalarQuantizationBuilder, VectorParamsBuilder,
};
use qdrant_client::Qdrant;

use crate::errors::app_error::AppError;

pub async fn reset(
    client: &Qdrant,
    collection_name: &str,
    dimensions: u64,
) -> Result<(), AppError> {
    client.delete_collection(collection_name).await?;
    client
        .create_collection(
            CreateCollectionBuilder::new(collection_name)
                .vectors_config(VectorParamsBuilder::new(dimensions, Distance::Cosine))
                .quantization_config(ScalarQuantizationBuilder::default()),
        )
        .await?;
    Ok(())
}
