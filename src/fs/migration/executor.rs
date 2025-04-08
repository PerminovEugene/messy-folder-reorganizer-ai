use crate::console::messages::{print_done_to_same_string, print_file_renamed, print_move_file};
use crate::errors::app_error::AppError;

use path_clean::PathClean;
use std::fs;
use std::path::{Path, PathBuf};

use super::fs_entry_migration::FsEntryMigration;

pub fn process_migration(
    migration: &mut FsEntryMigration,
    root_dir: &PathBuf,
) -> Result<(), AppError> {
    let source_path = build_migration_source_path(migration, root_dir);
    if !source_path.exists() {
        return Err(AppError::FileError(format!(
            "Source file does not exist: {}",
            source_path.display()
        )));
    }

    let destination_path = build_migration_destination_path(migration, root_dir);
    print_move_file(source_path.display(), destination_path.display());

    if let Some(parent) = destination_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| AppError::FileError(format!("Failed to create directory: {}", e)))?;
    }

    let new_name = safe_rename(
        &source_path,
        &destination_path,
        &migration.destination_file_name,
    )?;
    migration.destination_file_name = new_name.clone();
    print_done_to_same_string();
    if new_name != destination_path.file_name().unwrap().to_string_lossy() {
        print_file_renamed(&migration.source_file_name, new_name);
    }

    Ok(())
}

pub fn safe_rename(from: &PathBuf, to: &Path, file_name: &str) -> Result<String, AppError> {
    let safe_path_buf = generate_new_safe_path_buf(to, file_name)?;

    match fs::rename(from, &safe_path_buf) {
        Ok(_) => {
            if let Some(name) = safe_path_buf.file_name() {
                Ok(name.to_string_lossy().to_string())
            } else {
                Err(AppError::FileError(String::from("Invalid name generated")))
            }
        }
        Err(e) => Err(AppError::FileError(format!("Failed to rename file: {}", e))),
    }
}

fn generate_new_safe_path_buf(to: &Path, file_name: &str) -> Result<PathBuf, AppError> {
    for counter in 0..=10 {
        let candidate_path = if counter == 0 {
            to.to_path_buf()
        } else {
            let (name, ext_opt) = split_file_name(file_name);
            let new_file_name = match ext_opt {
                Some(ext) => format!("{name} ({counter}).{ext}"),
                None => format!("{name} ({counter})"),
            };

            to.parent()
                .unwrap_or_else(|| Path::new(""))
                .join(&new_file_name)
        };

        if !candidate_path.exists() {
            return Ok(candidate_path);
        }
    }

    Err(AppError::FileError(format!(
        "Rename failed: file already exists after 10 attempts: {}",
        to.display()
    )))
}

fn split_file_name(full_name: &str) -> (String, Option<String>) {
    let path = Path::new(full_name);
    let file_stem = path
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_default();

    let extension = path.extension().map(|s| s.to_string_lossy().to_string());

    (file_stem, extension)
}

pub fn build_migration_source_path(migration: &FsEntryMigration, root_dir: &PathBuf) -> PathBuf {
    PathBuf::from(root_dir)
        .join(&migration.source_arg)
        .join(&migration.source_relative_path)
        .join(&migration.source_file_name)
        .clean()
}

pub fn build_migration_destination_path(
    migration: &FsEntryMigration,
    root_dir: &PathBuf,
) -> PathBuf {
    PathBuf::from(root_dir)
        .join(&migration.destination_arg)
        .join(&migration.destination_relative_path)
        .join(&migration.destination_file_name)
        .clean()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn test_split_file_name_with_extension() {
        let (name, ext) = split_file_name("document.txt");
        assert_eq!(name, "document");
        assert_eq!(ext, Some("txt".to_string()));
    }

    #[test]
    fn test_split_file_name_without_extension() {
        let (name, ext) = split_file_name("README");
        assert_eq!(name, "README");
        assert_eq!(ext, None);
    }

    #[test]
    fn test_generate_new_safe_path_buf_no_conflict() {
        let dir = tempdir().unwrap();
        let to = dir.path().join("file.txt");
        let result = generate_new_safe_path_buf(&to, "file.txt").unwrap();
        assert_eq!(result, to);
    }

    #[test]
    fn test_generate_new_safe_path_buf_with_conflict() {
        let dir = tempdir().unwrap();
        let to = dir.path().join("file.txt");

        // Simulate existing files: file.txt, file (1).txt, ..., file (9).txt
        for i in 0..10 {
            let file_name = if i == 0 {
                "file.txt".to_string()
            } else {
                format!("file ({}).txt", i)
            };
            let path = dir.path().join(file_name);
            File::create(path).unwrap();
        }

        // file (10).txt should be the next available
        let safe_path = generate_new_safe_path_buf(&to, "file.txt").unwrap();
        assert_eq!(
            safe_path.file_name().unwrap().to_string_lossy(),
            "file (10).txt"
        );
    }

    #[test]
    fn test_generate_new_safe_path_buf_too_many_conflicts() {
        let dir = tempdir().unwrap();
        let to = dir.path().join("file.txt");

        // Create file.txt to file (10).txt — total 11 files
        for i in 0..=10 {
            let file_name = if i == 0 {
                "file.txt".to_string()
            } else {
                format!("file ({}).txt", i)
            };
            let path = dir.path().join(file_name);
            File::create(path).unwrap();
        }

        let result = generate_new_safe_path_buf(&to, "file.txt");
        assert!(result.is_err());
    }
}
