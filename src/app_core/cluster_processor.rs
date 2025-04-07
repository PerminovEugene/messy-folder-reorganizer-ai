use futures::stream::{FuturesUnordered, StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::configuration::config::LLMModelConfig;
use crate::ml::hierarchical_clustering::Cluster;
use crate::{ai::llm_request::ask_ai_for_reordering_plan, configuration::args::ProcessArgs};

use super::sources_processor::FileProcessingResult;

#[derive(Serialize, Deserialize, Debug)]
struct AiResponse {
    folder_name: String,
}

pub async fn process_clusters(
    config: &LLMModelConfig,
    args: &ProcessArgs,
    clusters: &Vec<Cluster>,
    unknown_vectors: &[FileProcessingResult],
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
