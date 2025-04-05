use crate::app_core::destination_processor::index_destinations;
use crate::app_core::migration_plan_builder::create_migration_plan;
use crate::app_core::migrations_processor::migrate_files;
use crate::app_core::sources_processor::process_sources;
use crate::configuration::config_loader::load_configurations;
use crate::console::table::{print_migration_plan_table, print_rag_processing_result};
use crate::errors::app_error::AppError;
use crate::fs::migration::plan::save_migrations_to_file;

use crate::configuration::args::ProcessArgs;

pub async fn run_process(args: ProcessArgs) -> Result<(), AppError> {
    let (embeddings_config, llm_config, rag_ml_config) = load_configurations();

    index_destinations(&embeddings_config, &rag_ml_config, &args).await?;
    let mut process_result = process_sources(&embeddings_config, &rag_ml_config, &args).await?;

    print_rag_processing_result(&rag_ml_config, &process_result);

    let migration_plan =
        create_migration_plan(&llm_config, &rag_ml_config, &args, &mut process_result).await;

    print_migration_plan_table(&migration_plan);
    save_migrations_to_file(migration_plan);

    migrate_files(&args)?;

    Ok(())
}
