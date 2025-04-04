use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FsEntryMigration {
    pub file_name: String,
    pub destination_inner_path: String,
    pub source_inner_path: String,
    pub source: String,
    pub destination: String,
}
