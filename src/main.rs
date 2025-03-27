use std::env;

use clap::Parser;

mod ai;
mod bd;
mod configuration;
mod console;
mod errors;
mod files;
mod ml;
mod workflow;

use configuration::config_loader::load_configurations;
use configuration::init::init;
use console::errors::print_app_error;
use console::messages::print_initial_message;
use console::table::print_migration_plan_table;
use console::table::print_rag_processing_result;
use errors::app_error::AppError;
use files::create_file::save_files_reorganisation_plan;
use files::file_info;
use workflow::destination_processor::index_destinations;
use workflow::plan_processor::migrate_files;
use workflow::sources_processor::process_sources;
use workflow::unknown_files_processor::create_folder_for_unknown_files;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {
    match run().await {
        Ok(_) => (),
        Err(e) => match e {
            AppError::OllamaConnection(_) => {
                print_app_error("Ollama Error", e);
                std::process::exit(1);
            }
            AppError::QdrantClient(_) => {
                print_app_error("Qdrant Error", e);
                std::process::exit(1);
            }
            _ => {
                print_app_error("Panic", e);
                panic!("Unhandled error. \n Please post error stack trace on github issues page https://github.com/PerminovEugene/messy-folder-reorganizer-ai/issues");
            }
        },
    }
}

async fn run() -> Result<(), AppError> {
    print_initial_message(VERSION);

    init();

    let args = configuration::args::Args::parse();
    let (embeddings_config, llm_config, rag_ml_config) = load_configurations();

    index_destinations(&embeddings_config, &rag_ml_config, &args).await?;
    let mut process_result = process_sources(&embeddings_config, &rag_ml_config, &args).await?;

    print_rag_processing_result(&rag_ml_config, &process_result);

    let migration_plan =
        create_folder_for_unknown_files(&llm_config, &rag_ml_config, &args, &mut process_result)
            .await;

    print_migration_plan_table(&migration_plan);
    save_files_reorganisation_plan(migration_plan);

    migrate_files(&args).await;

    Ok(())
}
