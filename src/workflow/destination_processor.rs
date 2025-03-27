use std::env;
use std::path::PathBuf;

use crate::ai::embedding_context::add_context_to_folders_input;
use crate::ai::embeddings_request::get_embeddings;
use crate::bd::quadrant::add_vectors;
use crate::configuration::args::Args;
use crate::configuration::config::{EmbeddingModelConfig, RagMlConfig};
use crate::configuration::config_loader::parse_ignore_list;
use crate::console::messages::{
    print_creating_dest_embeddings, print_parsing_destination_folder, print_saving_dest_embeddings,
};
use crate::errors::app_error::AppError;
use crate::files::dirr_processing::{collect_files_metadata, CollectFilesMetaConfig};
use crate::files::file_info::{self, convert_path_meta_to_file_info};

pub async fn index_destinations(
    embedding_config: &EmbeddingModelConfig,
    rag_ml_config: &RagMlConfig,
    args: &Args,
) -> Result<(), AppError> {
    print_parsing_destination_folder();
    let mut dest_files_data: Vec<file_info::FileInfo> = Vec::new();

    let dest = if args.destination != "home" {
        args.destination.clone()
    } else {
        env::var("HOME").unwrap_or_else(|_| ".".to_string())
    };

    let collector_config = CollectFilesMetaConfig {
        skip_problematic_dir: args.skip_problematic_dir,
        recursive: true,
        process_folders: true,
        process_files: false,
    };

    let ignore_patters = parse_ignore_list(&rag_ml_config.destination_ignore);

    collect_files_metadata(
        &dest,
        "",
        &mut dest_files_data,
        &ignore_patters,
        &collector_config,
    );

    if args.destination != "home" {
        let destination_base_folder = PathBuf::from(args.destination.clone());
        let file_name = destination_base_folder.file_name().unwrap();

        let destination_base_folder_2 = PathBuf::from(file_name);

        let dest_file_info = convert_path_meta_to_file_info(
            &destination_base_folder_2,
            destination_base_folder.metadata().unwrap(),
        );
        dest_files_data.push(dest_file_info);
    }

    let original_folder_names = dest_files_data
        .iter()
        .map(|d| d.name.clone())
        .collect::<Vec<_>>();

    let embeddings_input = add_context_to_folders_input(&original_folder_names);

    print_creating_dest_embeddings();

    let dest_embeddings = get_embeddings(
        &embeddings_input,
        args.embedding_model.clone(),
        args.ai_server_address.clone(),
        embedding_config.clone(),
    )
    .await?;

    print_saving_dest_embeddings();

    add_vectors(args, &original_folder_names, dest_embeddings)
        .await
        .unwrap();

    Ok(())
}
