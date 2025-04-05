use super::config::CollectFilesMetaConfig;
use crate::errors::app_error::AppError;

pub fn get_dir_entry_and_metadata(
    dir_entry_result: Result<std::fs::DirEntry, std::io::Error>,
    config: &CollectFilesMetaConfig,
) -> Result<Option<(std::fs::DirEntry, std::fs::Metadata)>, AppError> {
    let dir = match dir_entry_result {
        Ok(entry) => entry,
        Err(e) => {
            if config.continue_on_fs_errors {
                eprintln!("Error reading directory entry: {:?}", e);
                return Ok(None);
            } else {
                return Err(AppError::FileError(e.to_string()));
            }
        }
    };

    let metadata = match dir.metadata() {
        Ok(m) => m,
        Err(e) => {
            if config.continue_on_fs_errors {
                eprintln!("Error reading metadata for {:?}: {:?}", dir.path(), e);
                return Ok(None);
            } else {
                return Err(AppError::FileError(e.to_string()));
            }
        }
    };

    Ok(Some((dir, metadata)))
}
