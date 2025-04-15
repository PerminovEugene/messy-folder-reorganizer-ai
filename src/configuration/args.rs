use clap::{Args as ClapArgs, Parser, Subcommand};

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

#[derive(ClapArgs, Debug)]
pub struct ProcessArgs {
    #[arg(long = "language-model", short = 'L', help = "Ollama LLM model name")]
    pub llm_model: String,

    #[arg(long = "embedding-model", short = 'E', help = "Embedding model name")]
    pub embedding_model: String,

    #[arg(long = "source", short = 'S', help = "Source folder to reorder")]
    pub source: String,

    #[arg(long = "destination", short = 'D', default_value_t = String::from("home"), help = "Destination folder")]
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

    #[arg(long = "llm-address", short = 'n', default_value_t = String::from("http://localhost:11434"), help = "LLM server address")]
    pub ai_server_address: String,

    #[arg(long = "qdrant-address", short = 'q', default_value_t = String::from("http://localhost:6334"), help = "Qdrant server address")]
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
