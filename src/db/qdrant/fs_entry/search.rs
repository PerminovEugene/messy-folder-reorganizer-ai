use qdrant_client::qdrant::value::Kind;
use qdrant_client::qdrant::{
    with_payload_selector, SearchBatchPoints, SearchPoints, WithPayloadSelector,
};
use qdrant_client::Qdrant;

use crate::errors::app_error::AppError;

use super::meta::FS_ENTRY_COLLECTION_NAME;
use super::payload::FsEntryPayload;

pub struct ClosestFsEntry {
    /// Name of the matched file system entity (file or directory)
    pub name: String,
    /// Starting from destination path in args. Full relative path is relative_path + name
    pub relative_path: String,
    /// cosine similarity score
    pub score: f32,
    pub query_vector: Vec<f32>,
    /// if it's root then relative_path is empty and name is the same as destination top level folder name
    pub is_root: bool,
}

pub async fn find_closest_fs_entry(
    client: &Qdrant,
    vectors: Vec<Vec<f32>>,
) -> Result<Vec<ClosestFsEntry>, AppError> {
    let search_points: Vec<SearchPoints> = vectors
        .iter()
        .map(|vector| SearchPoints {
            collection_name: FS_ENTRY_COLLECTION_NAME.to_owned(),
            vector: vector.clone(),
            limit: 1,
            with_payload: Some(WithPayloadSelector {
                selector_options: Some(with_payload_selector::SelectorOptions::Enable(true)),
            }),
            ..Default::default()
        })
        .collect();

    let batch = SearchBatchPoints {
        search_points,
        collection_name: FS_ENTRY_COLLECTION_NAME.to_owned(),
        read_consistency: None,
        timeout: None,
    };

    let response = client.search_batch_points(batch).await?;

    let results = response
        .result
        .into_iter()
        .zip(vectors)
        .filter_map(|(batch_result, vector)| {
            let point = batch_result.result.first()?;
            Some(ClosestFsEntry {
                relative_path: extract_string(&point.payload, FsEntryPayload::RELATIVE_PATH),
                name: extract_string(&point.payload, FsEntryPayload::FILE_NAME),
                is_root: extract_bool(&point.payload, FsEntryPayload::IS_ROOT),
                score: point.score,
                query_vector: vector,
            })
        })
        .collect();

    Ok(results)
}

fn extract_string(
    payload: &std::collections::HashMap<String, qdrant_client::qdrant::Value>,
    key: &str,
) -> String {
    payload
        .get(key)
        .and_then(|v| v.as_str())
        .map(ToOwned::to_owned)
        .unwrap_or_default()
}

fn extract_bool(
    payload: &std::collections::HashMap<String, qdrant_client::qdrant::Value>,
    key: &str,
) -> bool {
    payload
        .get(key)
        .and_then(|v| match &v.kind {
            Some(Kind::BoolValue(b)) => Some(*b),
            Some(Kind::StringValue(s)) => match s.to_lowercase().as_str() {
                "true" => Some(true),
                "false" => Some(false),
                _ => None,
            },
            _ => None,
        })
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use qdrant_client::qdrant::value::Kind;
    use qdrant_client::qdrant::Value;
    use std::collections::HashMap;

    #[test]
    fn test_extract_string_success() {
        let mut payload = HashMap::new();
        payload.insert(
            "key".to_string(),
            Value {
                kind: Some(Kind::StringValue("hello".to_string())),
            },
        );

        let result = extract_string(&payload, "key");
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_extract_string_missing_key() {
        let payload: HashMap<String, Value> = HashMap::new();
        let result = extract_string(&payload, "missing");
        assert_eq!(result, "");
    }

    #[test]
    fn test_extract_string_wrong_type() {
        let mut payload = HashMap::new();
        payload.insert(
            "key".to_string(),
            Value {
                kind: Some(Kind::BoolValue(true)),
            },
        );

        let result = extract_string(&payload, "key");
        assert_eq!(result, ""); // not a string
    }

    #[test]
    fn test_extract_bool_from_bool_value() {
        let mut payload = HashMap::new();
        payload.insert(
            "key".to_string(),
            Value {
                kind: Some(Kind::BoolValue(true)),
            },
        );

        let result = extract_bool(&payload, "key");
        assert!(result);
    }

    #[test]
    fn test_extract_bool_from_string_value_true() {
        let mut payload = HashMap::new();
        payload.insert(
            "key".to_string(),
            Value {
                kind: Some(Kind::StringValue("true".to_string())),
            },
        );

        let result = extract_bool(&payload, "key");
        assert!(result);
    }

    #[test]
    fn test_extract_bool_from_string_value_false() {
        let mut payload = HashMap::new();
        payload.insert(
            "key".to_string(),
            Value {
                kind: Some(Kind::StringValue("false".to_string())),
            },
        );

        let result = extract_bool(&payload, "key");
        assert!(!result);
    }

    #[test]
    fn test_extract_bool_missing_key() {
        let payload: HashMap<String, Value> = HashMap::new();
        let result = extract_bool(&payload, "missing");
        assert!(!result);
    }

    #[test]
    fn test_extract_bool_invalid_string() {
        let mut payload = HashMap::new();
        payload.insert(
            "key".to_string(),
            Value {
                kind: Some(Kind::StringValue("notabool".to_string())),
            },
        );

        let result = extract_bool(&payload, "key");
        assert!(!result);
    }
}
