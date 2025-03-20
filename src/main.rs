use std::env;

use clap::Parser;

mod ai;
mod bd;
mod configuration;
mod console;
mod files;
mod ml;
mod workflow;

use configuration::config_loader::load_configurations;
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
    let (embeddings_config, llm_config, rag_ml_config) = load_configurations();

    destination_processor::index_destinations(&embeddings_config, &args).await;

    let mut process_result = sources_processor::process_sources(&embeddings_config, &args).await;

    print_rag_processing_result(&rag_ml_config, &process_result);

    let migration_plan = unknown_files_processor::create_folder_for_unknown_files(
        &llm_config,
        &rag_ml_config,
        &args,
        &mut process_result,
    )
    .await;

    print_migration_plan_table(&migration_plan);
    save_files_reorganisation_plan(migration_plan);

    plan_processor::migrate_files(&args).await;
}
