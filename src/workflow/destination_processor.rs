use std::path::PathBuf;

use crate::ai::embedding_context::add_context_to_folders_input;
use crate::ai::embeddings_request::get_embeddings;
use crate::configuration::args::Args;
use crate::configuration::config::{EmbeddingModelConfig, RagMlConfig};
use crate::configuration::ignore_list::parse_ignore_list;
use crate::console::messages::{
    print_creating_dest_embeddings, print_parsing_destination_folder, print_saving_dest_embeddings,
};
use crate::db::qdrant;
use crate::db::qdrant::fs_entry::meta::FS_ENTRY_COLLECTION_NAME;
use crate::errors::app_error::AppError;
use crate::files::file_collector::config::CollectFilesMetaConfig;
use crate::files::file_collector::walker::collect_files_metadata;
use crate::files::file_info::{self, build_file_info, FileInfo};
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
        let dest_file_info = build_file_info(
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

    save_destination_files_embeddings(args, dest_embeddings, dest_files_data).await?;

    Ok(())
}

async fn save_destination_files_embeddings(
    args: &Args,
    vectors: Vec<Vec<f32>>,
    file_infos: Vec<FileInfo>,
) -> Result<(), AppError> {
    let client = qdrant::client::init(&args.qdrant_server_address).await?;
    let dimension_size = qdrant::utils::get_dimension_size_by_vectors(&vectors)?;
    qdrant::collection::reset(&client, FS_ENTRY_COLLECTION_NAME, dimension_size).await?;
    qdrant::fs_entry::insert::insert_fs_entries_by_file_infos(&client, vectors, &file_infos)
        .await?;

    Ok(())
}
