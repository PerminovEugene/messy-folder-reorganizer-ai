#[derive(Debug)]
pub struct CollectFilesMetaConfig {
    pub continue_on_fs_errors: bool,
    pub recursive: bool,
    pub process_folders: bool,
    pub process_files: bool,
}
