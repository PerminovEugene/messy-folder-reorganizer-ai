use std::collections::HashMap;

use qdrant_client::qdrant::value::Kind;
use qdrant_client::qdrant::{
    with_payload_selector, CreateCollectionBuilder, Distance, PointStruct,
    ScalarQuantizationBuilder, SearchBatchPoints, SearchPoints, UpsertPointsBuilder, Value,
    VectorParamsBuilder, WithPayloadSelector,
};
use qdrant_client::{Payload, Qdrant};
use uuid::Uuid;

use crate::configuration::args::Args;
use crate::errors::app_error::AppError;
use crate::files::file_info::FileInfo;

const COLLECTION_NAME: &str = "dest";

pub async fn add_vectors(
    args: &Args,
    ids: &[FileInfo],
    vectors: Vec<Vec<f32>>,
) -> Result<(), AppError> {
    let address = &args.qdrant_server_address.clone();
    let client = create_client(address).await?;

    client.delete_collection(COLLECTION_NAME).await?;

    let dimensions = vectors.first().unwrap().len() as u64;

    client
        .create_collection(
            CreateCollectionBuilder::new(COLLECTION_NAME)
                .vectors_config(VectorParamsBuilder::new(dimensions, Distance::Cosine))
                .quantization_config(ScalarQuantizationBuilder::default()),
        )
        .await?;

    let points: Vec<PointStruct> = ids
        .iter()
        .zip(vectors.iter().cloned())
        .map(|(file_info, vector)| {
            let id = Uuid::new_v4().as_u128() as u64;

            let mut payload_data: HashMap<String, Value> = HashMap::new();
            payload_data.insert(
                "file_name".to_string(),
                Value::from(file_info.file_name.to_string()),
            );
            payload_data.insert(
                "relative_path".to_string(),
                Value::from(file_info.relative_path.to_string()),
            );
            payload_data.insert(
                "is_root".to_string(),
                Value::from(file_info.is_root.to_string()),
            );

            let payload = Payload::from(payload_data);

            PointStruct::new(id, vector, payload)
        })
        .collect();

    client
        .upsert_points(UpsertPointsBuilder::new(COLLECTION_NAME, points))
        .await?;

    Ok(())
}

pub struct SearchResultFacade {
    pub relative_path: String,
    pub score: f32,
    pub vector: Vec<f32>,
    pub is_root: bool,
    pub file_name: String,
}

pub async fn find_closest_pathes(
    args: &Args,
    vectors: Vec<Vec<f32>>,
) -> Result<Vec<SearchResultFacade>, AppError> {
    let client = create_client(&args.qdrant_server_address.clone()).await?;

    let search_points: Vec<SearchPoints> = vectors
        .iter()
        .map(|vector| SearchPoints {
            collection_name: COLLECTION_NAME.to_string(),
            vector: vector.clone(),
            limit: 1,
            with_payload: Some(WithPayloadSelector {
                selector_options: Some(with_payload_selector::SelectorOptions::Enable(true)),
            }),
            ..Default::default()
        })
        .collect();

    let search_batch_points = SearchBatchPoints {
        search_points,
        collection_name: COLLECTION_NAME.to_string(),
        read_consistency: None,
        timeout: None,
    };

    let search_result = client.search_batch_points(search_batch_points).await?;

    let result: Vec<SearchResultFacade> = search_result
        .result
        .iter()
        .zip(vectors.into_iter())
        .map(|(point, vector)| {
            let result = &point.result[0];

            let relative_path: String = result
                .payload
                .get("relative_path")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .unwrap_or_default();

            let is_root: bool = result
                .payload
                .get("is_root")
                .and_then(|v| match &v.kind {
                    Some(Kind::BoolValue(b)) => Some(*b),
                    Some(Kind::StringValue(s)) => match s.to_lowercase().as_str() {
                        "true" => Some(true),
                        "false" => Some(false),
                        _ => None,
                    },
                    _ => None,
                })
                .unwrap_or_default();

            let file_name: String = result
                .payload
                .get("file_name")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .unwrap_or_default();

            SearchResultFacade {
                score: result.score,
                vector,
                file_name,
                relative_path,
                is_root,
            }
        })
        .collect();

    Ok(result)
}

async fn create_client(address: &str) -> Result<Qdrant, AppError> {
    let client = Qdrant::from_url(address).build()?;
    Ok(client)
}
