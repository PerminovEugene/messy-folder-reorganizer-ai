use std::fs;
use std::io::Result;
use std::path::Path;

use super::config::TestCase;

fn reset_dir(path: &Path) -> Result<()> {
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)
}

pub fn create_test_folders(test: &TestCase, path: &Path) -> Result<()> {
    // --- Clean and recreate source folder ---
    let source = path.join(&test.source.folder);
    reset_dir(&source)?;

    for file in &test.source.files {
        let source_full_path = source.join(&file.path);
        if let Some(parent) = source_full_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::File::create(source_full_path)?;
    }

    let destination = path.join(&test.destination.folder);
    reset_dir(&destination)?;

    for dir in &test.destination.structure {
        let destination_full_path = destination.join(dir);
        fs::create_dir_all(destination_full_path)?;
    }

    for file in &test.destination.existing_files {
        let destination_full_path = destination.join(file);
        if let Some(parent) = destination_full_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::File::create(destination_full_path)?;
    }

    Ok(())
}
