use std::fs;
use std::io::Result;
use std::os::unix::fs::symlink;
use std::path::Path;

use super::config::{FolderEntry, TestCase};

fn reset_dir(path: &Path) -> Result<()> {
    if path.exists() {
        fs::remove_dir_all(path)?;
    }
    fs::create_dir_all(path)
}

pub fn create_test_folders(test: &TestCase, path_to_case: &Path) -> Result<()> {
    let (source_root_folder_name, entry) = test.source.iter().next().unwrap();
    let source_path = path_to_case.join(source_root_folder_name);
    reset_dir(&source_path)?;

    create_files_recursively(&source_path, entry)?;

    let (destination_root_folder_name, entry) = test.destination.iter().next().unwrap();

    let destination_path = path_to_case.join(destination_root_folder_name);

    reset_dir(&destination_path)?;

    create_files_recursively(&destination_path, entry)?;

    Ok(())
}

fn create_files_recursively(base_path: &Path, entry: &FolderEntry) -> std::io::Result<()> {
    match entry {
        FolderEntry::FileList(files) => {
            for file in files {
                let file_path = base_path.join(file);
                fs::File::create(file_path)?;
            }
        }
        FolderEntry::Folder(map) => {
            for (subfolder, subentry) in map {
                if subfolder == "files" || subfolder == "symlinks" {
                    let sub_path: &Path = base_path;
                    create_files_recursively(sub_path, subentry)?;
                } else {
                    let sub_path = base_path.join(subfolder);
                    fs::create_dir_all(&sub_path)?;
                    create_files_recursively(&sub_path, subentry)?;
                }
            }
        }
        FolderEntry::SymlinkTarget(map) => {
            for (link, target) in map {
                let link = base_path.join(link);
                let original = base_path.join(target);
                symlink(original, link)?;
            }
        }
    }
    Ok(())
}
