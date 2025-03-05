use serde::{Deserialize, Serialize};

use crate::ai::embeddings::{self, get_embeddings};
use crate::bd::quadrant::{add_vectors, find_closest_vectors};
use crate::configuration::args::Args;
use crate::configuration::config::Config;
use crate::files::create_file::{create_plan_file, create_source_file};
use crate::files::dirr_processing::collect_files_metadata;
use crate::files::file_info::{self, FilesReorganisationPlan};

pub async fn process_sources(config: &Config, args: &Args) {
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

    let closest_pathes = find_closest_vectors(embeddings).await.unwrap();

    let plan: Vec<FilesReorganisationPlan> = closest_pathes
        .iter()
        .zip(file_names.into_iter())
        .map(|(dest_path, source_file_name)| FilesReorganisationPlan {
            original: source_file_name.clone(),
            new_path: dest_path.clone(),
        })
        .collect();
    let json_plan = serde_json::to_string_pretty(&plan).unwrap();

    create_plan_file(json_plan);
}
