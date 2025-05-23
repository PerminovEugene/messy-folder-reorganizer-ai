use qdrant_client::QdrantError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Ollama is not available. Please check that it is launched.\nDetails: {0}")]
    OllamaConnection(String),
    #[error("Ollama response has unexpected format. Probably version mismatch.\nPlease contact the author and provide your Ollama version and used model names.\nDetails: {0}")]
    OllamaResponseParse(String),
    #[error("Ollama LLM model name not provided when using local AI provider.")]
    OllamaLlmModelMissing,
    #[error("Ollama Embedding model name not provided when using local AI provider.")]
    OllamaEmbeddingModelMissing,

    #[error("OpenAI API request failed: {0}")]
    OpenAIAPIError(String),
    #[error("OpenAI API authentication failed. Ensure OPENAI_API_KEY is set correctly and valid: {0}")]
    OpenAIAuthError(String),
    #[error("OpenAI API response parsing error: {0}")]
    OpenAIResponseParseError(String),
    #[error("OpenAI Service Error: Type: {error_type}, Message: {message}, Code: {code:?}")]
    OpenAIServiceError {
        message: String,
        error_type: String,
        code: Option<String>,
    },

    #[error("Qdrant client error: {0}")]
    QdrantClient(#[from] QdrantError),
    #[error("Custom Qdrant error: {0}")]
    QdrantCustom(String),

    #[error("JSON stringify error: {0}")]
    JSONStringify(String),

    #[error("File system error: {0}")]
    FileError(String),

    #[error("Configuration error: {0}")]
    Configuration(String),
    #[error("An unexpected error occurred: {0}")]
    GenericError(String),
}
