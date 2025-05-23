use clap::{Args as ClapArgs, Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize}; // For potential future config file use

#[derive(ValueEnum, Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AiProvider {
    #[default]
    Local, // Represents Ollama
    OpenAI,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Processes the files and generates a migration plan
    Process(ProcessArgs),

    /// Applies the last saved migration plan
    Apply(ApplyArgs),

    /// Rolls back the last migration using logs
    Rollback(RollbackArgs),
}

#[derive(ClapArgs, Debug, Clone)] // Added Clone
pub struct ProcessArgs {
    #[arg(
        long = "ai-provider",
        value_enum,
        default_value_t = AiProvider::Local,
        help = "AI provider to use ('local' for Ollama, or 'openai')"
    )]
    pub ai_provider: AiProvider,

    #[arg(
        long = "language-model", // Kept original long name
        short = 'L',
        help = "Ollama LLM model name (required if --ai-provider is local/ollama)"
    )]
    pub ollama_llm_model: Option<String>,

    #[arg(
        long = "embedding-model", // Kept original long name
        short = 'E',
        help = "Ollama Embedding model name (required if --ai-provider is local/ollama)"
    )]
    pub ollama_embedding_model: Option<String>,

    #[arg(
        long = "ollama-server-address", // More specific long name
        short = 'n', // Kept original short flag for ollama server address
        default_value_t = String::from("http://localhost:11434"),
        help = "Ollama server address (if --ai-provider is local/ollama)"
    )]
    pub ollama_server_address: String,

    #[arg(
        long = "openai-api-key",
        help = "OpenAI API Key (required if --ai-provider is openai). Can also be set via OPENAI_API_KEY environment variable."
    )]
    pub openai_api_key: Option<String>,

    #[arg(
        long = "openai-llm-model",
        default_value_t = String::from("gpt-4o-mini"),
        help = "OpenAI LLM model name for folder generation (if --ai-provider is openai)"
    )]
    pub openai_llm_model: String,

    #[arg(
        long = "openai-embedding-model",
        default_value_t = String::from("text-embedding-ada-002"), // A common default, user can change
        help = "OpenAI Embedding model name (if --ai-provider is openai)"
    )]
    pub openai_embedding_model: String,
    
    #[arg(
        long = "openai-api-base",
        default_value_t = String::from("https://api.openai.com/v1"),
        help = "OpenAI API base URL (if --ai-provider is openai). Can also be set via OPENAI_API_BASE environment variable."
    )]
    pub openai_api_base: String,

    #[arg(long = "openai-temperature", help = "Temperature for OpenAI LLM (default: 0.7)")]
    pub openai_temperature: Option<f32>,

    #[arg(long = "openai-max-tokens", help = "Max tokens for OpenAI LLM completion (default: 150)")]
    pub openai_max_tokens: Option<u32>,

    #[arg(long = "openai-embedding-dimensions", help = "Dimensions for OpenAI embeddings (e.g., for text-embedding-3 models)")]
    pub openai_embedding_dimensions: Option<u32>,


    #[arg(long = "source", short = 'S', help = "Source folder to reorder")]
    pub source: String,

    #[arg(
        long = "destination",
        short = 'D',
        default_value_t = String::from("home"),
        help = "Destination folder"
    )]
    pub destination: String,

    #[arg(
        long = "recursive",
        short = 'R',
        default_value_t = false,
        help = "Process subfolders"
    )]
    pub recursive: bool,

    #[arg(
        long = "force-apply",
        short = 'F',
        default_value_t = false,
        help = "Apply without review"
    )]
    pub force_apply: bool,

    #[arg(
        long = "continue-on-fs-errors",
        short = 'C',
        default_value_t = false,
        help = "Allow partial migration, when some of folders/files produce errors (because of access rights, file is locked, etc)"
    )]
    pub continue_on_fs_errors: bool,

    #[arg(
        long = "qdrant-address",
        short = 'q',
        default_value_t = String::from("http://localhost:6334"),
        help = "Qdrant server address"
    )]
    pub qdrant_server_address: String,
}

#[derive(ClapArgs, Debug)]
pub struct RollbackArgs {
    #[arg(
        long = "session-id",
        short = 'i',
        help = "Process command session id which migrations should be rolled back"
    )]
    pub session_id: String,
}

#[derive(ClapArgs, Debug)]
pub struct ApplyArgs {
    #[arg(
        long = "session-id",
        short = 'i',
        help = "Process command session id which migrations should be applied"
    )]
    pub session_id: String,
}
