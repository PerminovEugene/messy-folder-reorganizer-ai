use std::{collections::HashMap, fs, path::Path};

use crate::helpers::config::FolderEntry;

use super::config::TestCase;

pub fn assert_fs_structure(
    test: &TestCase,
    path_to_case: &Path,
    field_name: &str,
) -> Result<(), String> {
    fn check_folder(folder_path: &Path, entry: &FolderEntry) -> Result<(), String> {
        match entry {
            FolderEntry::FileList(expected_files) => {
                let mut actual_files: Vec<String> = fs::read_dir(folder_path)
                    .map_err(|e| format!("Failed to read dir {}: {}", folder_path.display(), e))?
                    .filter_map(|entry| entry.ok())
                    .filter(|entry| entry.file_type().map(|ft| ft.is_file()).unwrap_or(false))
                    .map(|entry| entry.file_name().to_string_lossy().to_string())
                    .collect();

                actual_files.sort();
                let mut expected_sorted = expected_files.clone();
                expected_sorted.sort();

                if actual_files != expected_sorted {
                    return Err(format!(
                        "Mismatch in folder '{}':\n  Expected: {:?}\n  Found:    {:?}",
                        folder_path.display(),
                        expected_sorted,
                        actual_files
                    ));
                }
            }
            FolderEntry::Folder(map) => {
                for (name, subentry) in map {
                    if name == "files" || name == "symlinks" {
                        check_folder(folder_path, subentry)?;
                    } else {
                        let subfolder_path = folder_path.join(name);
                        if !subfolder_path.exists() {
                            return Err(format!("Missing folder: {}", subfolder_path.display()));
                        }
                        check_folder(&subfolder_path, subentry)?;
                    }
                }
            }
            FolderEntry::SymlinkTarget(_) => {
                println!("Symlink is ignored")
            }
        }
        Ok(())
    }

    let structure_map: &HashMap<String, FolderEntry> = match field_name {
        "expected" => &test.expected,
        "source" => &test.source,
        "destination" => &test.destination,
        _ => return Err(format!("Unknown field name: {}", field_name)),
    };

    for (base_folder, structure) in structure_map {
        let base_path = path_to_case.join(base_folder);
        if !base_path.exists() {
            return Err(format!("Missing base folder: {}", base_path.display()));
        }

        check_folder(&base_path, structure)?;
    }

    Ok(())
}
