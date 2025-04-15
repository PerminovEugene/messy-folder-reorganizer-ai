use std::collections::HashMap;
use std::path::{Path, PathBuf};

use qdrant_client::qdrant::{PointStruct, UpsertPointsBuilder, Value};
use qdrant_client::Payload;
use uuid::Uuid;

use crate::errors::app_error::AppError;
use crate::fs::file_info::FsEntry;

use super::meta::FS_ENTRY_COLLECTION_NAME;
use super::payload::FsEntryPayload;

pub async fn insert_fs_entries_by_file_infos(
    client: &qdrant_client::Qdrant,
    destination_path: &String,
    vectors: Vec<Vec<f32>>,
    file_infos: &[FsEntry],
    session_id: &str,
) -> Result<(), AppError> {
    let path_to_dest = std::env::current_dir().unwrap().join(destination_path);

    let points: Vec<PointStruct> = file_infos
        .iter()
        .zip(vectors.iter().cloned())
        .map(|(fs_entry, vector)| {
            let id = Uuid::new_v4().to_string();
            let payload = build_payload(fs_entry, &path_to_dest, session_id);
            PointStruct::new(id, vector, payload)
        })
        .collect();

    client
        .upsert_points(UpsertPointsBuilder::new(FS_ENTRY_COLLECTION_NAME, points))
        .await?;

    Ok(())
}

fn build_payload(file_info: &FsEntry, abs_path_to_dest: &Path, session_id: &str) -> Payload {
    let mut data = HashMap::new();
    data.insert(
        FsEntryPayload::FILE_NAME,
        Value::from(file_info.file_name.as_str()),
    );

    let abs_path: PathBuf = abs_path_to_dest.join(&file_info.relative_path);

    data.insert(FsEntryPayload::SESSION_ID, Value::from(session_id));

    data.insert(
        FsEntryPayload::ABSOLUTE_PATH,
        Value::from(abs_path.to_string_lossy().to_string()),
    );
    let segments = build_path_segments(abs_path);
    data.insert(FsEntryPayload::SEGMENTS, Value::from(segments));

    data.insert(FsEntryPayload::IS_ROOT, Value::from(file_info.is_root));
    Payload::from(data)
}

/// Splits absolute path /a/b/c/d to segments /a, /a/b, /a/b/c, /a/b/c/d
fn build_path_segments<P: AsRef<Path>>(abs_path: P) -> Vec<String> {
    let mut segments = Vec::new();
    let mut current = PathBuf::new();

    for component in abs_path.as_ref().components() {
        current.push(component);
        if let Some(path_str) = current.to_str() {
            segments.push(path_str.to_string());
        }
    }

    segments
}

#[cfg(test)]
mod tests {
    use super::*;
    use qdrant_client::qdrant::value::{self, Kind};

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
        let path_to_dest = PathBuf::from(String::from("/a/b/c/"));

        let payload = build_payload(&file_info, &path_to_dest, "id");
        let map: HashMap<String, Value> = payload.into();

        let file_name = map.get(FsEntryPayload::FILE_NAME).unwrap();
        assert_eq!(file_name.as_str().unwrap(), "hello.txt");

        let abs_path = map.get(FsEntryPayload::ABSOLUTE_PATH).unwrap();
        assert_eq!(abs_path.as_str().unwrap(), "/a/b/c/docs/");

        if let Some(Value {
            kind: Some(value::Kind::ListValue(list)),
        }) = map.get(FsEntryPayload::SEGMENTS)
        {
            let extracted: Vec<&str> = list
                .values
                .iter()
                .filter_map(|v| match &v.kind {
                    Some(value::Kind::StringValue(s)) => Some(s.as_str()),
                    _ => None,
                })
                .collect();

            let expected = vec!["/", "/a", "/a/b", "/a/b/c", "/a/b/c/docs"];
            assert_eq!(extracted, expected);
        } else {
            panic!("segments is not a list of strings");
        }
        let is_root = map.get(FsEntryPayload::IS_ROOT).unwrap();
        assert_eq!(is_root.kind, Some(Kind::BoolValue(false)));
    }
}
