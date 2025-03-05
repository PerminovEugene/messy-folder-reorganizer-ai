use serde::{Deserialize, Serialize};
use std::{
    fs::Metadata,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Serialize, Deserialize, Debug)]

pub struct FileInfo {
    pub name: String,
    size: u64,
    created_at: String,
    modified_at: String,
}

fn get_system_time_string(time: SystemTime) -> String {
    time.duration_since(UNIX_EPOCH)
        .map(|dur| dur.as_secs().to_string())
        .unwrap_or_else(|_| "unknown".to_string())
}

pub fn convert_path_meta_to_file_info(path: &Path, file_meta: Metadata) -> FileInfo {
    FileInfo {
        name: path.to_string_lossy().to_string(),
        size: file_meta.len(),
        created_at: get_system_time_string(file_meta.created().unwrap_or(SystemTime::now())),
        modified_at: get_system_time_string(file_meta.modified().unwrap_or(SystemTime::now())),
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FilesReorganisationPlan {
    pub original: String,
    pub new_path: String,
}
