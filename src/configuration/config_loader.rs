use std::fs;

use regex::Regex;

use super::{
    config::{EmbeddingModelConfig, LLMModelConfig, RagMlConfig},
    consts::{
        EMBEDDINGS_GENERATION_CONFIGURATION_FILE, LLM_CONFIGURATION_FILE, RAG_ML_CONFIGURATION_FILE,
    },
    init::get_config_file_path,
};

pub fn read_config<T: serde::de::DeserializeOwned>(config_file_name: &str) -> T {
    let config_path = get_config_file_path(config_file_name);

    let toml_content = fs::read_to_string(config_path).unwrap_or_else(|_| "".to_string());

    let config: T = toml::from_str(&toml_content).unwrap();

    config
}

pub fn load_configurations() -> (EmbeddingModelConfig, LLMModelConfig, RagMlConfig) {
    let embeddings_config: EmbeddingModelConfig =
        read_config(EMBEDDINGS_GENERATION_CONFIGURATION_FILE);
    let llm_config: LLMModelConfig = read_config(LLM_CONFIGURATION_FILE);
    let rag_ml_config: RagMlConfig = read_config(RAG_ML_CONFIGURATION_FILE);

    (embeddings_config, llm_config, rag_ml_config)
}

pub fn parse_ignore_list(ignore_list: &Option<Vec<String>>) -> Vec<regex::Regex> {
    ignore_list
        .as_ref()
        .unwrap_or(&vec![])
        .iter()
        .map(|pattern| Regex::new(pattern).unwrap())
        .collect::<Vec<_>>()
}
