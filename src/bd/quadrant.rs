use std::collections::HashMap;

use qdrant_client::qdrant::{
    with_payload_selector, CreateCollectionBuilder, Distance, PointStruct,
    ScalarQuantizationBuilder, SearchBatchPoints, SearchPoints, UpsertPointsBuilder, Value,
    VectorParamsBuilder, WithPayloadSelector,
};
use qdrant_client::{Payload, Qdrant, QdrantError};
use uuid::Uuid;

use crate::configuration::args::Args;

pub async fn add_vectors(
    args: &Args,
    ids: &[String],
    vectors: Vec<Vec<f32>>,
) -> Result<(), QdrantError> {
    let address = &args.qdrant_server_address.clone();
    let client: Qdrant = Qdrant::from_url(address).build()?;

    let collection_name = "dest";

    client.delete_collection(collection_name).await?;

    client
        .create_collection(
            CreateCollectionBuilder::new(collection_name)
                .vectors_config(VectorParamsBuilder::new(1024, Distance::Cosine))
                .quantization_config(ScalarQuantizationBuilder::default()),
        )
        .await?;

    let points: Vec<PointStruct> = ids
        .iter()
        .zip(vectors.iter().cloned())
        .map(|(path, vector)| {
            let id = Uuid::new_v4().as_u128() as u64;

            let mut payload_data: HashMap<String, Value> = HashMap::new();
            payload_data.insert("path".to_string(), Value::from(path.to_string())); // Исправлено: path.to_string()
            payload_data.insert("type".to_string(), Value::from("file"));

            let payload = Payload::from(payload_data);

            PointStruct::new(id, vector, payload)
        })
        .collect();

    match client
        .upsert_points(UpsertPointsBuilder::new(collection_name, points))
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Failed to add vectors: {}", e);
            Err(e)
        }
    }
}

pub struct SearchResultFacade {
    pub path: String,
    pub score: f32,
    pub vector: Vec<f32>,
}

pub async fn find_closest_pathes(
    args: &Args,
    vectors: Vec<Vec<f32>>,
) -> Result<Vec<SearchResultFacade>, QdrantError> {
    let client = Qdrant::from_url(&args.qdrant_server_address.clone()).build()?;

    let collection_name = "dest".to_string();

    let searches: Vec<SearchPoints> = vectors
        .iter()
        .map(|vector| SearchPoints {
            collection_name: collection_name.clone(),
            vector: vector.clone(),
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

    let result: Vec<SearchResultFacade> = search_result
        .result
        .iter()
        .zip(vectors.into_iter())
        .map(|(point, vector)| {
            let result = &point.result[0];

            let path = result
                .payload
                .get("path")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .unwrap_or_default();

            SearchResultFacade {
                path,
                score: result.score,
                vector,
            }
        })
        .collect();

    Ok(result)
}
