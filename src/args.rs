use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(
        long = "model",
        short = 'M',
        help = "Model name loaded in ollama to use"
    )]
    pub model: String,

    #[arg(
        long = "path",
        short = 'P',
        help = "Path to the folder with files to reorder"
    )]
    pub path: String,

    // optional arguments
    #[arg(
        long = "recursive",
        short = 'R',
        default_value_t = false,
        help = "Should inner folders be processed"
    )]
    pub recursive: bool,

    #[arg(
        long = "show-ai-thinking",
        short = 'A',
        default_value_t = false,
        help = "Will show AI thinking details"
    )]
    pub show_ai_thinking: bool,

    #[arg(
        long = "show-promt",
        short = 'S',
        default_value_t = false,
        help = "Will show promt for AI"
    )]
    pub show_promt: bool,

    #[arg(
        long = "force-apply",
        short = 'F',
        default_value_t = false,
        help = "Will apply the reordering plan without review from user side"
    )]
    pub force_apply: bool,
}
