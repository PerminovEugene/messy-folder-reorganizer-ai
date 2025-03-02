use std::collections::HashMap;

use qdrant_client::qdrant::{
    CreateCollectionBuilder, Distance, PointStruct, ScalarQuantizationBuilder, UpsertPointsBuilder,
    Value, VectorParamsBuilder,
};
use qdrant_client::{Payload, Qdrant, QdrantError};
use serde_json::json;
use uuid::Uuid;

pub async fn add_vectors(ids: &Vec<&String>, vectors: Vec<Vec<f32>>) -> Result<(), QdrantError> {
    let client: Qdrant = Qdrant::from_url("http://localhost:6334").build()?;

    let collection_name = "test 2";

    // Check if collection exists, if not, create it

    // let collections_list = client.list_collections().await?;
    client.delete_collection(collection_name).await?;

    client
        .create_collection(
            CreateCollectionBuilder::new(collection_name)
                .vectors_config(VectorParamsBuilder::new(384, Distance::Cosine))
                .quantization_config(ScalarQuantizationBuilder::default()),
        )
        .await?;

    // let payload = Payload::new("test".to_string(), "test".to_string());
    let points: Vec<PointStruct> = ids
        .into_iter()
        .zip(vectors.into_iter())
        .map(|(path, vector)| {
            let id = Uuid::new_v4().as_u128() as u64; // Qdrant ожидает u64

            let mut payload_data: HashMap<String, Value> = HashMap::new();
            payload_data.insert("path".to_string(), Value::from(path.to_string())); // Исправлено: path.to_string()
            payload_data.insert("type".to_string(), Value::from("file"));

            let payload = Payload::from(payload_data);

            PointStruct::new(id, vector, payload)
        })
        .collect();

    println!("Inserting {} points", points.len());

    let insert_res = client
        .upsert_points(UpsertPointsBuilder::new(collection_name, points))
        .await?;

    println!("{:?}", insert_res);

    Ok(())
}
