use std::env;

use clap::Parser;

use messy_folder_reorganizer_ai::commands::apply::apply_latest_migration_plan;
use messy_folder_reorganizer_ai::commands::process::run_process;
use messy_folder_reorganizer_ai::commands::rollback::start_rollback;
use messy_folder_reorganizer_ai::configuration::args::{Args, Commands};
use messy_folder_reorganizer_ai::configuration::init::init;
use messy_folder_reorganizer_ai::console::messages::print_initial_message;
use messy_folder_reorganizer_ai::errors::app_error::AppError;
use messy_folder_reorganizer_ai::errors::app_error_handler::handle;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {
    match run().await {
        Ok(_) => (),
        Err(e) => handle(e),
    }
}

async fn run() -> Result<(), AppError> {
    print_initial_message(VERSION);
    init();

    let args = Args::parse();

    match args.command {
        Commands::Process(process_args) => run_process(process_args).await,
        Commands::Apply(apply_args) => apply_latest_migration_plan(apply_args).await,
        Commands::Rollback(rollback_args) => start_rollback(rollback_args).await,
    }
}
