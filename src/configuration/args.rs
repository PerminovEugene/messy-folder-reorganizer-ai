use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(
        long = "language-model",
        short = 'L',
        help = "Language model name loaded in ollama to use for folder names generation"
    )]
    pub llm_model: String,

    #[arg(
        long = "embedding-model",
        short = 'E',
        help = "Embedding model name loaded in ollama to use for embeddings generation"
    )]
    pub embedding_model: String,

    #[arg(
        long = "source",
        short = 'S',
        help = "Path to the folder with files to reorder"
    )]
    pub source: String,

    #[arg(
        long = "destination",
        short = 'D',
        default_value_t = String::from("home"),
        help = "Path to the folder which will be used as destination for reordering. If not specified, the home folder will be used"
    )]
    pub destination: String,

    // optional arguments
    #[arg(
        long = "recursive",
        short = 'R',
        default_value_t = false,
        help = "Should inner folders be processed"
    )]
    pub recursive: bool,

    #[arg(
        long = "force-apply",
        short = 'F',
        default_value_t = false,
        help = "Will apply the reordering plan without review from user side"
    )]
    pub force_apply: bool,

    #[arg(
        long = "skip-problematic-dir",
        short = 'd',
        default_value_t = false,
        help = "Will skip problematic dirrectories and files"
    )]
    pub skip_problematic_dir: bool,

    #[arg(
      long = "llm-address",
      short = 'n',
      default_value_t = String::from("http://localhost:11434/"),
      help = "Will replace default LLM server address (default address is http://localhost:11434/)"
    )]
    pub ai_server_address: String,

    #[arg(
      long = "qdrant-address",
      short = 'q',
      default_value_t = String::from("http://localhost:6334/"),
      help = "Will replace default qdrant server address (default address is http://localhost:6334/)"
  )]
    pub qdrant_server_address: String,
}
