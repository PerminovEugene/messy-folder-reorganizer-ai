use qdrant_client::QdrantError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Ollama is not available. Please check that it is launched.\nDetails: {0}")]
    OllamaConnection(String),

    #[error("Ollama response has unexpected format. Probably version mismatch.\nPlease contact the author and provide your Ollama version and used model names.\nDetails: {0}")]
    OllamaResponseParse(String),

    #[error("Qdrant client error: {0}")]
    QdrantClient(#[from] QdrantError),

    #[error("JSON stringify error: {0}")]
    JSONStringify(String),
}
