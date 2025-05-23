use std::path::PathBuf;

use crate::ai::embedding_context::add_context_to_folders_input;
use crate::ai::embeddings_request::get_ai_embeddings;
use crate::configuration::args::ProcessArgs;
use crate::configuration::config::{EmbeddingModelConfig, RagMlConfig};
use crate::configuration::ignore_list::parse_ignore_list;
use crate::console::messages::{
    print_creating_dest_embeddings, print_parsing_destination_folder, print_saving_dest_embeddings,
};
use crate::db::qdrant;
use crate::db::qdrant::fs_entry::meta::FS_ENTRY_COLLECTION_NAME;
use crate::errors::app_error::AppError;
use crate::fs::file_info::{self, build_fs_entry, FsEntry};
use crate::fs::parser::config::CollectFilesMetaConfig;
use crate::fs::parser::walker::collect_fs_entries_data;
use crate::fs::path::get_home_path;

pub async fn index_destinations(
    embedding_config: &EmbeddingModelConfig,
    rag_ml_config: &RagMlConfig,
    args: &ProcessArgs,
    session_id: &str,
) -> Result<(), AppError> {
    print_parsing_destination_folder();
    let mut dest_files_data: Vec<file_info::FsEntry> = Vec::new();

    let destination_base_folder = if args.destination != "home" {
        PathBuf::from(args.destination.clone())
    } else {
        get_home_path()
    };

    let collector_config = CollectFilesMetaConfig {
        continue_on_fs_errors: args.continue_on_fs_errors,
        recursive: true,
        process_folders: true,
        process_files: false,
    };

    let ignore_patters = parse_ignore_list(&rag_ml_config.destination_ignore)?;

    let root_relative_path: PathBuf = PathBuf::from("");
    collect_fs_entries_data(
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
        let dest_file_info = build_fs_entry(
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

    let dest_embeddings = get_ai_embeddings(
        &embeddings_input,
        args,
        embedding_config,
    )
    .await?;

    print_saving_dest_embeddings();

    save_destination_files_embeddings(args, dest_embeddings, dest_files_data, session_id).await?;

    Ok(())
}

async fn save_destination_files_embeddings(
    args: &ProcessArgs,
    vectors: Vec<Vec<f32>>,
    fs_entries: Vec<FsEntry>,
    session_id: &str,
) -> Result<(), AppError> {
    let client = qdrant::client::init(&args.qdrant_server_address).await?;
    let dimension_size = qdrant::utils::get_dimension_size_by_vectors(&vectors)?;
    qdrant::collection::safe_create_collection(&client, FS_ENTRY_COLLECTION_NAME, dimension_size)
        .await?;
    qdrant::fs_entry::insert::insert_fs_entries_by_file_infos(
        &client,
        &args.destination,
        vectors,
        &fs_entries,
        session_id,
    )
    .await?;

    Ok(())
}
