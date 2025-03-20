use serde::{Deserialize, Serialize};

use crate::ai::embeddings;
use crate::bd::quadrant::find_closest_pathes;
use crate::configuration::args::Args;
use crate::configuration::config::EmbeddingModelConfig;
use crate::console::messages::{
    print_generating_embeddings_for_sources, print_looking_for_suitable_destination,
    print_parsing_sources,
};
use crate::files::create_file::create_source_file;
use crate::files::dirr_processing::{collect_files_metadata, CollectFilesMetaConfig};
use crate::files::file_info;

#[derive(Serialize, Deserialize, Debug)]
pub struct ProcessResult {
    pub path: String,
    pub score: f32,
    pub source_file_name: String,
    pub vector: Vec<f32>,
}

pub async fn process_sources(config: &EmbeddingModelConfig, args: &Args) -> Vec<ProcessResult> {
    let mut files_data: Vec<file_info::FileInfo> = Vec::new();

    let collector_config = &CollectFilesMetaConfig {
        skip_problematic_dir: args.skip_problematic_dir,
        recursive: args.recursive,
        process_folders: false,
        process_files: true,
    };

    print_parsing_sources();

    collect_files_metadata(&args.source, "", &mut files_data, &vec![], collector_config);
    create_source_file(&files_data);

    let file_names = files_data.iter().map(|d| &d.name).collect::<Vec<_>>();

    let file_names = format_file_names(&file_names);

    print_generating_embeddings_for_sources();

    let embeddings = embeddings::get_embeddings(
        &file_names,
        args.embedding_model.clone(),
        args.ai_server_address.clone(),
        config.clone(),
    )
    .await
    .unwrap();

    print_looking_for_suitable_destination();
    let closest_pathes = find_closest_pathes(args, embeddings).await.unwrap();

    let mut result: Vec<ProcessResult> = closest_pathes
        .into_iter()
        .zip(file_names.into_iter())
        .map(|(cp, file_name)| ProcessResult {
            path: cp.path,
            score: cp.score,
            source_file_name: file_name.clone(),
            vector: cp.vector,
        })
        .collect();

    result.sort_by_key(|process_result| process_result.path.clone());

    result
}

fn format_file_name(file_name: &str) -> String {
    let parts: Vec<&str> = file_name.rsplitn(2, '.').collect();
    let format = parts.first().unwrap_or(&"").to_string();

    let name = parts.get(1).unwrap_or(&file_name).replace(['-', '_'], " ");

    format!("{}.{}", name, format)
}

fn format_file_names(file_names: &Vec<&String>) -> Vec<String> {
    file_names
        .iter()
        .map(|file_name| format_file_name(file_name))
        .collect()
}
