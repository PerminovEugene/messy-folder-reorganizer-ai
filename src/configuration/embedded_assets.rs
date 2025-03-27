/// Embedded assets (compile-time constants).
/// Adjust the relative paths so they point correctly to your files on disk.
/// For example, if this file is at `src/configuration/mod.rs`,
/// and your `assets` folder is at the project root, you might need `../../assets/...`.
pub const EMBEDDINGS_MODEL_CONFIG_FILE_BYTES: &[u8] =
    include_bytes!("../../assets/embeddings_config.toml");
pub const LLM_MODEL_CONFIG_FILE_BYTES: &[u8] = include_bytes!("../../assets/llm_config.toml");
pub const RAG_ML_CONFIG_FILE_BYTES: &[u8] = include_bytes!("../../assets/rag_ml_config.toml");

pub const GENERATE_FOLDER_NAME_PROMPT_FILE_BYTES: &[u8] =
    include_bytes!("../../assets/prompts/generate_folder_name.md");
