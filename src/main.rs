use std::env;

use clap::Parser;

mod ai;
mod bd;
mod configuration;
mod console;
mod files;
mod ml;
mod workflow;

use configuration::init::init;
use console::messages::print_initial_message;
use console::table::print_migration_plan_table;
use console::table::print_rag_processing_result;
use files::create_file::save_files_reorganisation_plan;
use files::file_info;
use workflow::destination_processor;
use workflow::plan_processor;
use workflow::sources_processor;
use workflow::unknown_files_processor;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {
    print_initial_message(VERSION);

    init();
    let args = configuration::args::Args::parse();
    let config = configuration::read_config::read_config();

    destination_processor::index_destinations(&config, &args).await;

    let mut process_result = sources_processor::process_sources(&config, &args).await;

    print_rag_processing_result(&process_result);

    let migration_plan = unknown_files_processor::create_folder_for_unknown_files(
        &config,
        &args,
        &mut process_result,
    )
    .await;

    print_migration_plan_table(&migration_plan);
    save_files_reorganisation_plan(migration_plan);

    plan_processor::migrate_files(&args).await;
}
