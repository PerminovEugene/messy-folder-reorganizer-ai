use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TestCase {
    pub source: HashMap<String, FolderEntry>,
    pub destination: HashMap<String, FolderEntry>,
    pub expected: HashMap<String, FolderEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FolderEntry {
    Folder(HashMap<String, FolderEntry>),
    FileList(Vec<String>),
    SymlinkTarget(HashMap<String, String>),
}
