use std::env;

use clap::Parser;

mod ai;
mod app_core;
mod commands;
mod configuration;
mod console;
mod db;
mod errors;
mod fs;
mod ml;

use commands::apply::apply_latest_migration_plan;
use commands::process::run_process;
use commands::rollback::start_rollback;
use configuration::args::{Args, Commands};
use configuration::init::init;
use console::messages::print_initial_message;
use errors::app_error::AppError;
use errors::app_error_handler::handle;

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
        Commands::Apply {} => apply_latest_migration_plan().await,
        Commands::Rollback {} => start_rollback().await,
    }
}
