/// Embedded assets (compile-time constants).
/// Adjust the relative paths so they point correctly to your files on disk.
/// For example, if this file is at `src/configuration/mod.rs`,
/// and your `assets` folder is at the project root, you might need `../../assets/...`.
pub const CONFIG_FILE_BYTES: &[u8] = include_bytes!("../../assets/config.toml");
pub const INITIAL_PROMPT_FILE_BYTES: &[u8] =
    // include_bytes!("../../assets/prompts/initial_sort_request.md");
    include_bytes!("../../assets/prompts/generate_folder_name.md");
