use serde::{Deserialize, Serialize};
use std::{
    fs::Metadata,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct FileInfo {
    pub file_name: String,
    pub relative_path: String,
    pub size: u64,
    pub created_at: String,
    pub modified_at: String,
    pub is_root: bool,
}

fn get_system_time_string(time: SystemTime) -> String {
    time.duration_since(UNIX_EPOCH)
        .map(|dur| dur.as_secs().to_string())
        .unwrap_or_else(|_| "unknown".to_string())
}

pub fn convert_path_meta_to_file_info(
    file_name: String,
    relative_path: &Path,
    file_meta: Metadata,
    is_root: bool,
) -> FileInfo {
    FileInfo {
        file_name,
        relative_path: relative_path.to_string_lossy().to_string(),
        size: file_meta.len(),
        created_at: get_system_time_string(file_meta.created().unwrap_or(SystemTime::now())),
        modified_at: get_system_time_string(file_meta.modified().unwrap_or(SystemTime::now())),
        is_root,
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FilesReorganisationPlan {
    pub file_name: String,
    pub destination_inner_path: String,
    pub source_inner_path: String,
    pub source: String,
    pub destination: String,
}
