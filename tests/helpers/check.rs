use std::fs;
use std::path::Path;

use super::config::TestCase;

pub fn check_expected_structure(test: &TestCase, path_to_case: &Path) -> Result<(), String> {
    for (base_folder, subfolders) in &test.expected.structure {
        let base_path = path_to_case.join(base_folder);

        for (subfolder, expected_files) in subfolders {
            let folder_path = base_path.join(subfolder);
            if !folder_path.exists() {
                return Err(format!("Missing folder: {}", folder_path.display()));
            }

            let mut actual_files: Vec<String> = fs::read_dir(&folder_path)
                .map_err(|e| format!("Failed to read dir {}: {}", folder_path.display(), e))?
                .filter_map(|entry| entry.ok())
                .filter(|entry| entry.file_type().map(|ft| ft.is_file()).unwrap_or(false))
                .map(|entry| entry.file_name().to_string_lossy().to_string())
                .collect();

            actual_files.sort();
            let mut expected_files_sorted = expected_files.clone();
            expected_files_sorted.sort();

            if actual_files != expected_files_sorted {
                return Err(format!(
                    "Mismatch in folder '{}':\n  Expected: {:?}\n  Found:    {:?}",
                    folder_path.display(),
                    expected_files_sorted,
                    actual_files
                ));
            }
        }
    }

    Ok(())
}
