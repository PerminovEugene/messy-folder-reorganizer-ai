use serde::{Deserialize, Serialize};

use crate::ai::embeddings::{self, get_embeddings};
use crate::bd::quadrant::{add_vectors, find_closest_pathes};
use crate::configuration::args::Args;
use crate::configuration::config::Config;
use crate::files::create_file::{create_plan_file, create_source_file};
use crate::files::dirr_processing::collect_files_metadata;
use crate::files::file_info::{self, FilesReorganisationPlan};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProcessResult {
    pub path: String,
    pub score: f32,
    pub source_file_name: String,
    pub vector: Vec<f32>,
}

pub async fn process_sources(config: &Config, args: &Args) -> Vec<ProcessResult> {
    let mut files_data: Vec<file_info::FileInfo> = Vec::new();

    collect_files_metadata(
        &args.path,
        "",
        args.skip_problematic_dir,
        &mut files_data,
        &vec![],
        args.recursive,
        false,
        true,
    );
    create_source_file(&files_data);

    let file_names = files_data.iter().map(|d| &d.name).collect::<Vec<_>>();

    let embeddings = embeddings::get_embeddings(
        &file_names,
        args.model.clone(),
        args.ai_server_address.clone(),
        config.clone(),
    )
    .await
    .unwrap();

    let closest_pathes = find_closest_pathes(embeddings).await.unwrap();

    closest_pathes
        .into_iter()
        .zip(file_names.into_iter())
        .map(|(cp, file_name)| ProcessResult {
            path: cp.path,
            score: cp.score,
            source_file_name: file_name.clone(),
            vector: cp.vector,
        })
        // .zip(file_names.into_iter())
        .collect()
}
