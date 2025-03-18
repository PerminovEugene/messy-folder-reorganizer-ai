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
use files::create_file::save_files_reorganisation_plan;
use files::file_info;
use workflow::destination_processor;
use workflow::plan_processor;
use workflow::sources_processor;
use workflow::unknown_files_processor;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {
    println!("Messy-folder-reorganizer-ai - Version {}", VERSION);

    init();
    let args = configuration::args::Args::parse();
    let config = configuration::read_config::read_config();

    destination_processor::index_destinations(&config, &args).await;

    let mut process_result = sources_processor::process_sources(&config, &args).await;

    process_result.iter().for_each(|result| {
        println!("{:?} close to {:?}", result.source_file_name, result.path);
    });

    let migration_plan = unknown_files_processor::create_folder_for_unknown_files(
        &config,
        &args,
        &mut process_result,
    )
    .await;

    save_files_reorganisation_plan(migration_plan);

    plan_processor::migrate_files(&args).await;
}
