use std::collections::HashMap;

use qdrant_client::qdrant::qdrant_client::QdrantClient;
use qdrant_client::qdrant::{
    with_payload_selector, CreateCollectionBuilder, Distance, PointStruct, ReadConsistency,
    ScalarQuantizationBuilder, SearchBatchPoints, SearchPoints, UpsertPointsBuilder, Value,
    VectorParamsBuilder, WithPayloadSelector,
};
use qdrant_client::{Payload, Qdrant, QdrantError};
use uuid::Uuid;

pub async fn add_vectors(ids: &Vec<&String>, vectors: Vec<Vec<f32>>) -> Result<(), QdrantError> {
    let client: Qdrant = Qdrant::from_url("http://localhost:6334").build()?;

    let collection_name = "dest";

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

pub async fn find_closest_vectors(vectors: Vec<Vec<f32>>) -> Result<Vec<String>, QdrantError> {
    let client = Qdrant::from_url("http://localhost:6334").build()?;

    let collection_name = "dest".to_string();

    let searches: Vec<SearchPoints> = vectors
        .into_iter()
        .map(|vector| SearchPoints {
            collection_name: collection_name.clone(),
            vector,
            limit: 1,
            with_payload: Some(WithPayloadSelector {
                selector_options: Some(with_payload_selector::SelectorOptions::Enable(true)),
            }),
            ..Default::default()
        })
        .collect();

    let batch_search_req = SearchBatchPoints {
        search_points: searches,
        collection_name,
        read_consistency: None,
        timeout: None,
    };

    let search_result = client.search_batch_points(batch_search_req).await?;

    dbg!(&search_result);

    let paths: Vec<String> = search_result
        .result
        .iter()
        .map(|point| {
            let result = point.result.iter().next().unwrap();

            if result.score <= 0.3 {
                return "Unknown".to_string();
            }
            result
                .payload
                // .iter()
                // .filter(|scored_point| scored_point.score >= max_distance)
                .get("path")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .unwrap_or_default()
        })
        .collect();

    Ok(paths)
}
