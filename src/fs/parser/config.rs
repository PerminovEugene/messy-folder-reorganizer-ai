#[derive(Debug)]
pub struct CollectFilesMetaConfig {
    pub skip_problematic_dir: bool,
    pub recursive: bool,
    pub process_folders: bool,
    pub process_files: bool,
}
