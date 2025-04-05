use std::collections::HashMap;

use crate::configuration::args::ProcessArgs;
use crate::configuration::config::{LLMModelConfig, RagMlConfig};
use crate::console::messages::{
    print_asking_llm_for_new_folder_names, print_clustering_unknown_vectors,
};
use crate::console::table::print_clusters_ai_proposed_names;
use crate::fs::migration::fs_entry_migration::FsEntryMigration;
use crate::ml::agglomerative_clustering::cluster_vectors_hierarchical;
use crate::{ai::llm_request::ask_ai_for_reordering_plan, ml::hierarchical_clustering::Cluster};
use futures::stream::{FuturesUnordered, StreamExt};
use serde::{Deserialize, Serialize};

use super::sources_processor::ProcessResult;

pub async fn create_migration_plan(
    llm_config: &LLMModelConfig,
    rag_ml_config: &RagMlConfig,
    args: &ProcessArgs,
    process_result: &mut [ProcessResult],
) -> Vec<FsEntryMigration> {
    let threshhold = rag_ml_config.valid_embedding_threshold.unwrap();

    let (processed_vectors, unknown_vectors): (Vec<_>, Vec<_>) =
        process_result.iter().partition(|&cp| cp.score > threshhold);

    let mut migration_plan: Vec<FsEntryMigration> = processed_vectors
        .iter()
        .map(|x| FsEntryMigration {
            file_name: x.source_file_name.clone(),
            destination_inner_path: x.destination_relative_path.clone(),
            source_inner_path: x.source_relative_path.clone(),
            source: args.source.clone(),
            destination: args.destination.clone(),
        })
        .collect();

    // TODO Move to separated func?
    if !unknown_vectors.is_empty() {
        print_clustering_unknown_vectors();
        let clusters = cluster_vectors_hierarchical(rag_ml_config, &unknown_vectors).await;

        print_asking_llm_for_new_folder_names();
        let cluster_id_to_path_hash =
            process_clusters(llm_config, args, &clusters, &unknown_vectors).await;

        print_clusters_ai_proposed_names(&cluster_id_to_path_hash);

        let reorganisation_plans: Vec<FsEntryMigration> = clusters
            .iter()
            .flat_map(|cluster| {
                cluster.members.iter().map(|&member| {
                    let unknown_vector_from_cluster = &unknown_vectors[member];
                    FsEntryMigration {
                        file_name: unknown_vector_from_cluster.source_file_name.clone(),
                        source_inner_path: unknown_vector_from_cluster.source_relative_path.clone(),
                        destination_inner_path: cluster_id_to_path_hash[&cluster.id].clone(),
                        source: args.source.clone(),
                        destination: args.destination.clone(),
                    }
                })
            })
            .collect();
        migration_plan.extend(reorganisation_plans);
    }

    migration_plan.sort_by_key(|a| a.destination_inner_path.clone()); // Sort first

    migration_plan
}

#[derive(Serialize, Deserialize, Debug)]

struct AiResponse {
    folder_name: String,
}

async fn process_clusters(
    config: &LLMModelConfig,
    args: &ProcessArgs,
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

            let ai_response: AiResponse =
                serde_json::from_str::<AiResponse>(&ai_response_raw).unwrap();

            (cluster_id, ai_response.folder_name)
        };

        futures.push(future);
    }

    let results: Vec<(usize, String)> = futures.collect::<Vec<_>>().await;
    results.into_iter().collect()
}
