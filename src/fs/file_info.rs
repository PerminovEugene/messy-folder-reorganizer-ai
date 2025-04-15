use serde::{Deserialize, Serialize};
use std::{
    fs::Metadata,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct FsEntry {
    pub file_name: String,
    /// relative for what
    pub relative_path: String,
    pub size: u64,
    pub created_at: String,
    pub modified_at: String,
    // related to destination argument
    pub is_root: bool,
}

fn get_system_time_string(time: SystemTime) -> String {
    time.duration_since(UNIX_EPOCH)
        .map(|dur| dur.as_secs().to_string())
        .unwrap_or_else(|_| "unknown".to_string())
}

pub fn build_fs_entry(
    name: String,
    relative_path: &Path,
    meta: Metadata,
    is_root: bool,
) -> FsEntry {
    FsEntry {
        file_name: name,
        relative_path: relative_path.to_string_lossy().to_string(),
        size: meta.len(),
        created_at: get_system_time_string(meta.created().unwrap_or(SystemTime::now())),
        modified_at: get_system_time_string(meta.modified().unwrap_or(SystemTime::now())),
        is_root,
    }
}
