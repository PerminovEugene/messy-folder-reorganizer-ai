use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FsEntryMigration {
    pub source_file_name: String,
    pub destination_file_name: String,
    pub destination_relative_path: String,
    pub source_relative_path: String,
    pub source_arg: String,
    pub destination_arg: String,
}
