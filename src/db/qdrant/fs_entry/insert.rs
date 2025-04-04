use std::collections::HashMap;

use qdrant_client::qdrant::{PointStruct, UpsertPointsBuilder, Value};
use qdrant_client::Payload;
use uuid::Uuid;

use crate::errors::app_error::AppError;
use crate::fs::file_info::FsEntry;

use super::meta::FS_ENTRY_COLLECTION_NAME;
use super::payload::FsEntryPayload;

pub async fn insert_fs_entries_by_file_infos(
    client: &qdrant_client::Qdrant,
    vectors: Vec<Vec<f32>>,
    file_infos: &[FsEntry],
) -> Result<(), AppError> {
    let points: Vec<PointStruct> = file_infos
        .iter()
        .zip(vectors.iter().cloned())
        .map(|(file_info, vector)| {
            let id = Uuid::new_v4().to_string();
            let payload = build_payload(file_info);
            PointStruct::new(id, vector, payload)
        })
        .collect();

    client
        .upsert_points(UpsertPointsBuilder::new(FS_ENTRY_COLLECTION_NAME, points))
        .await?;

    Ok(())
}

fn build_payload(file_info: &FsEntry) -> Payload {
    let mut data = HashMap::new();
    data.insert(
        FsEntryPayload::FILE_NAME,
        Value::from(file_info.file_name.as_str()),
    );
    data.insert(
        FsEntryPayload::RELATIVE_PATH,
        Value::from(file_info.relative_path.as_str()),
    );
    data.insert(FsEntryPayload::IS_ROOT, Value::from(file_info.is_root));
    Payload::from(data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use qdrant_client::qdrant::value::Kind;

    fn sample_file_info() -> FsEntry {
        FsEntry {
            size: 100,
            file_name: "hello.txt".to_string(),
            relative_path: "docs/".to_string(),
            created_at: "date".to_string(),
            modified_at: "date".to_string(),
            is_root: false,
        }
    }

    #[test]
    fn test_build_payload_fields() {
        let file_info = sample_file_info();
        let payload = build_payload(&file_info);
        let map: HashMap<String, Value> = payload.into();

        let file_name = map.get(FsEntryPayload::FILE_NAME).unwrap();
        assert_eq!(file_name.as_str().unwrap(), "hello.txt");

        let rel_path = map.get(FsEntryPayload::RELATIVE_PATH).unwrap();
        assert_eq!(rel_path.as_str().unwrap(), "docs/");

        let is_root = map.get(FsEntryPayload::IS_ROOT).unwrap();
        assert_eq!(is_root.kind, Some(Kind::BoolValue(false)));
    }
}
