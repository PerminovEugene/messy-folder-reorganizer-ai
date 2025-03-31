use std::path::PathBuf;

use crate::ai::embedding_context::add_context_to_folders_input;
use crate::ai::embeddings_request::get_embeddings;
use crate::bd::quadrant::add_vectors;
use crate::configuration::args::Args;
use crate::configuration::config::{EmbeddingModelConfig, RagMlConfig};
use crate::configuration::ignore_list::parse_ignore_list;
use crate::console::messages::{
    print_creating_dest_embeddings, print_parsing_destination_folder, print_saving_dest_embeddings,
};
use crate::errors::app_error::AppError;
use crate::files::file_collector::config::CollectFilesMetaConfig;
use crate::files::file_collector::walker::collect_files_metadata;
use crate::files::file_info::{self, convert_path_meta_to_file_info};
use crate::files::path::get_home_path;

pub async fn index_destinations(
    embedding_config: &EmbeddingModelConfig,
    rag_ml_config: &RagMlConfig,
    args: &Args,
) -> Result<(), AppError> {
    print_parsing_destination_folder();
    let mut dest_files_data: Vec<file_info::FileInfo> = Vec::new();

    let destination_base_folder = if args.destination != "home" {
        PathBuf::from(args.destination.clone())
    } else {
        get_home_path()
    };

    let collector_config = CollectFilesMetaConfig {
        skip_problematic_dir: args.skip_problematic_dir,
        recursive: true,
        process_folders: true,
        process_files: false,
    };

    let ignore_patters = parse_ignore_list(&rag_ml_config.destination_ignore)?;

    let root_relative_path: PathBuf = PathBuf::from("");
    collect_files_metadata(
        &destination_base_folder,
        &root_relative_path,
        &mut dest_files_data,
        &ignore_patters,
        &collector_config,
    )?;

    if args.destination != "home" {
        let destination_folder_name = destination_base_folder
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();
        let dest_file_info = convert_path_meta_to_file_info(
            destination_folder_name,
            &root_relative_path,
            destination_base_folder.metadata().unwrap(),
            true,
        );
        dest_files_data.push(dest_file_info);
    }

    let original_folder_names = dest_files_data
        .iter()
        .map(|d| d.file_name.clone())
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

    add_vectors(args, &dest_files_data, dest_embeddings)
        .await
        .unwrap();

    Ok(())
}
