use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::ai::embedding_context::add_context_to_files_input;
use crate::ai::embeddings_request;
use crate::configuration::args::Args;
use crate::configuration::config::{EmbeddingModelConfig, RagMlConfig};
use crate::configuration::ignore_list::parse_ignore_list;
use crate::console::messages::{
    print_generating_embeddings_for_sources, print_looking_for_suitable_destination,
    print_parsing_sources,
};
use crate::db::qdrant;
use crate::errors::app_error::AppError;
use crate::fs::create_file::create_source_file;
use crate::fs::file_info;
use crate::fs::parser::config::CollectFilesMetaConfig;
use crate::fs::parser::walker::collect_files_metadata;

#[derive(Serialize, Deserialize, Debug)]
pub struct ProcessResult {
    pub destination_relative_path: String,
    pub source_relative_path: String,
    pub score: f32,
    pub source_file_name: String,
    pub vector: Vec<f32>,
}

pub async fn process_sources(
    embedding_config: &EmbeddingModelConfig,
    rag_ml_config: &RagMlConfig,
    args: &Args,
) -> Result<Vec<ProcessResult>, AppError> {
    let mut files_data: Vec<file_info::FsEntry> = Vec::new();

    let collector_config = &CollectFilesMetaConfig {
        skip_problematic_dir: args.skip_problematic_dir,
        recursive: args.recursive,
        process_folders: false,
        process_files: true,
    };

    print_parsing_sources();

    let ignore_patters = parse_ignore_list(&rag_ml_config.source_ignore)?;

    let source_base_folder = PathBuf::from(args.source.clone());

    let root_relative_path: PathBuf = PathBuf::from("");

    collect_files_metadata(
        &source_base_folder,
        &root_relative_path,
        &mut files_data,
        &ignore_patters,
        collector_config,
    )?;
    create_source_file(&files_data);

    print_generating_embeddings_for_sources();

    // embeddings
    let original_file_names = files_data.iter().map(|d| &d.file_name).collect::<Vec<_>>();
    let formatted_file_names = format_file_names(&original_file_names);
    let embeddings_input = add_context_to_files_input(&formatted_file_names);
    let embeddings = embeddings_request::get_embeddings(
        &embeddings_input,
        args.embedding_model.clone(),
        args.ai_server_address.clone(),
        embedding_config.clone(),
    )
    .await?;

    print_looking_for_suitable_destination();

    let client = qdrant::client::init(&args.qdrant_server_address).await?;
    let closest_paths =
        qdrant::fs_entry::search::find_closest_fs_entry(&client, embeddings).await?;

    let mut result: Vec<ProcessResult> = closest_paths
        .into_iter()
        .zip(files_data.into_iter())
        .map(|(search_result, file_info)| {
            let destination_relative_path = if search_result.is_root {
                String::from("")
            } else {
                PathBuf::from(search_result.relative_path)
                    .join(search_result.name)
                    .to_string_lossy()
                    .to_string()
            };
            ProcessResult {
                destination_relative_path,
                score: search_result.score,
                source_file_name: file_info.file_name,
                source_relative_path: file_info.relative_path,
                vector: search_result.query_vector,
            }
        })
        .collect();

    result.sort_by(|a, b| {
        // First compare by path
        let path_cmp = a
            .destination_relative_path
            .cmp(&b.destination_relative_path);

        if path_cmp == std::cmp::Ordering::Equal {
            // If paths are equal, compare by score (descending)
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        } else {
            path_cmp
        }
    });

    Ok(result)
}

fn format_file_name(file_name: &str) -> String {
    let parts: Vec<&str> = file_name.rsplitn(2, '.').collect();
    let format = parts.first().unwrap_or(&"./").to_string();

    let name = parts.get(1).unwrap_or(&file_name).replace(['-', '_'], " ");

    format!("{}.{}", name, format)
}

fn format_file_names(file_names: &Vec<&String>) -> Vec<String> {
    file_names
        .iter()
        .map(|file_name| format_file_name(file_name))
        .collect()
}
