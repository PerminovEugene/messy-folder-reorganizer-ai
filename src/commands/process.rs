use uuid::Uuid;
use crate::app_core::destination_processor::index_destinations;
use crate::app_core::migration_plan_builder::create_migration_plan;
use crate::app_core::migrations_processor::migrate_files;
use crate::app_core::sources_processor::process_sources;
use crate::configuration::config_loader::load_configurations;
use crate::console::messages::print_session_id;
use crate::console::table::{print_migration_plan_table, print_rag_processing_result};
use crate::errors::app_error::AppError;
use crate::fs::migration::storage::save_migrations_to_file;
use crate::configuration::args::{ProcessArgs, AiProvider}; // Ensure AiProvider is imported
use std::env; // For reading env var

pub async fn run_process(args: ProcessArgs) -> Result<(), AppError> {
    let (ollama_embeddings_config, ollama_llm_config, rag_ml_config) = load_configurations();

    match args.ai_provider {
        AiProvider::Local => {
            if args.ollama_llm_model.is_none() {
                return Err(AppError::OllamaLlmModelMissing);
            }
            if args.ollama_embedding_model.is_none() {
                return Err(AppError::OllamaEmbeddingModelMissing);
            }
        }
        AiProvider::OpenAI => {
            if args.openai_api_key.is_none() && env::var("OPENAI_API_KEY").is_err() {
                 return Err(AppError::Configuration(
                    "OpenAI API key not found. Please set the OPENAI_API_KEY environment variable or use the --openai-api-key argument.".to_string()
                ));
            }
        }
    }

    let session_id = Uuid::new_v4().to_string();
    print_session_id(&session_id);

    index_destinations(&ollama_embeddings_config, &rag_ml_config, &args, &session_id).await?;
    let mut process_result =
        process_sources(&ollama_embeddings_config, &rag_ml_config, &args, &session_id).await?;

    print_rag_processing_result(&rag_ml_config, &process_result);

    let migration_plan =
        create_migration_plan(&ollama_llm_config, &rag_ml_config, &args, &mut process_result).await;

    print_migration_plan_table(&migration_plan);
    save_migrations_to_file(migration_plan, &session_id)?;

    migrate_files(args.force_apply, args.continue_on_fs_errors, &session_id)?;

    Ok(())
}
