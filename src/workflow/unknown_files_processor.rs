use std::collections::HashMap;

use crate::configuration::args::Args;
use crate::configuration::config::Config;
use crate::console::messages::{
    print_asking_llm_for_new_folder_names, print_clustering_unknown_vectors,
};
use crate::console::table::print_clusters_ai_proposed_names;
use crate::files::file_info::FilesReorganisationPlan;
use crate::ml::agglomerative_clustering::cluster_vectors_hierarchical;
use crate::{ai::ai_request::ask_ai_for_reordering_plan, ml::hierarchical_clustering::Cluster};
use futures::stream::{FuturesUnordered, StreamExt};
use serde::{Deserialize, Serialize};

use super::sources_processor::ProcessResult;

pub async fn create_folder_for_unknown_files(
    config: &Config,
    args: &Args,
    process_result: &mut [ProcessResult],
) -> Vec<FilesReorganisationPlan> {
    let (processed_vectors, unknown_vectors): (Vec<_>, Vec<_>) =
        process_result.iter().partition(|&cp| cp.score > 0.50);

    let mut migration_plan: Vec<FilesReorganisationPlan> = processed_vectors
        .iter()
        .map(|x| FilesReorganisationPlan {
            file_name: x.source_file_name.clone(),
            destination_inner_path: x.path.clone(),
            source: args.source.clone(),
            destination: args.destination.clone(),
        })
        .collect();

    if !unknown_vectors.is_empty() {
        print_clustering_unknown_vectors();
        let clusters = cluster_vectors_hierarchical(&unknown_vectors).await;

        print_asking_llm_for_new_folder_names();
        let folder_data = process_clusters(config, args, &clusters, &unknown_vectors).await;

        print_clusters_ai_proposed_names(&folder_data);

        let reorganisation_plans: Vec<FilesReorganisationPlan> = clusters
            .iter()
            .flat_map(|cluster| {
                cluster.members.iter().map(|&member| {
                    let unknown_vector_from_cluster = &unknown_vectors[member];
                    FilesReorganisationPlan {
                        file_name: unknown_vector_from_cluster.source_file_name.clone(),
                        destination_inner_path: folder_data[&cluster.id].clone(),
                        source: args.source.clone(),
                        destination: args.destination.clone(),
                    }
                })
            })
            .collect();
        migration_plan.extend(reorganisation_plans);
    }
    migration_plan
}

#[derive(Serialize, Deserialize, Debug)]

struct AiResponse {
    folder_name: String,
}

async fn process_clusters(
    config: &Config,
    args: &Args,
    clusters: &Vec<Cluster>,
    unknown_vectors: &[&ProcessResult],
) -> HashMap<usize, String> {
    let futures = FuturesUnordered::new();

    for cluster in clusters {
        let config = config.clone();
        let cluster_id = cluster.id;

        let files_data: Vec<&String> = cluster
            .members
            .iter()
            .take(10)
            .map(|&member| &unknown_vectors[member].source_file_name)
            .collect();

        let future = async move {
            let ai_response_raw = ask_ai_for_reordering_plan(
                files_data,
                args.llm_model.clone(),
                args.ai_server_address.clone(),
                config,
            )
            .await
            .unwrap();

            // println!("Ai response: {:?} {:?}", ai_response_raw, cluster_id);
            let ai_response: AiResponse =
                serde_json::from_str::<AiResponse>(&ai_response_raw).unwrap();

            (cluster_id, ai_response.folder_name)
        };

        futures.push(future);
    }

    let results: Vec<(usize, String)> = futures.collect::<Vec<_>>().await;
    results.into_iter().collect()
}
