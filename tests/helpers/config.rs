use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct TestCase {
    pub source: Source,
    pub destination: Destination,
    pub expected: Expected,
}

#[derive(Debug, Deserialize)]
pub struct Source {
    pub folder: String,
    pub files: Vec<FilePath>,
}

#[derive(Debug, Deserialize)]
pub struct Destination {
    pub folder: String,
    pub structure: Vec<String>,
    pub existing_files: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Expected {
    pub structure: HashMap<String, HashMap<String, Vec<String>>>,
}

#[derive(Debug, Deserialize)]
pub struct FilePath {
    pub path: String,
}
