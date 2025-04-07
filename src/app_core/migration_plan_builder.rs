use crate::configuration::args::ProcessArgs;
use crate::configuration::config::{LLMModelConfig, RagMlConfig};
use crate::console::messages::{
    print_asking_llm_for_new_folder_names, print_clustering_unknown_vectors,
};
use crate::console::table::print_clusters_ai_proposed_names;
use crate::fs::migration::fs_entry_migration::FsEntryMigration;
use crate::ml::agglomerative_clustering::cluster_vectors_hierarchical;

use super::cluster_processor::process_clusters;
use super::sources_processor::FileProcessingResult;

pub async fn create_migration_plan(
    llm_config: &LLMModelConfig,
    rag_ml_config: &RagMlConfig,
    args: &ProcessArgs,
    processing_results: &mut [FileProcessingResult],
) -> Vec<FsEntryMigration> {
    let (high_confidence_results, low_confidence_results) =
        split_by_score_match(rag_ml_config, processing_results);

    let mut fs_entry_migrations: Vec<FsEntryMigration> = high_confidence_results
        .iter()
        .map(|x| FsEntryMigration {
            source_file_name: x.source_file_name.clone(),
            destination_relative_path: x.destination_relative_path.clone(),
            source_relative_path: x.source_relative_path.clone(),
            source_arg: args.source.clone(),
            destination_arg: args.destination.clone(),
        })
        .collect();

    if !low_confidence_results.is_empty() {
        print_clustering_unknown_vectors();

        let clusters = cluster_vectors_hierarchical(rag_ml_config, &low_confidence_results).await;

        print_asking_llm_for_new_folder_names();

        let cluster_id_to_path_hash =
            process_clusters(llm_config, args, &clusters, &low_confidence_results).await;

        print_clusters_ai_proposed_names(&cluster_id_to_path_hash);

        let migrations_based_on_unknown_vectors: Vec<FsEntryMigration> = clusters
            .iter()
            .flat_map(|cluster| {
                cluster.members.iter().map(|&member| {
                    let unknown = &low_confidence_results[member];
                    FsEntryMigration {
                        source_file_name: unknown.source_file_name.clone(),
                        source_relative_path: unknown.source_relative_path.clone(),
                        destination_relative_path: cluster_id_to_path_hash[&cluster.id].clone(),
                        source_arg: args.source.clone(),
                        destination_arg: args.destination.clone(),
                    }
                })
            })
            .collect();

        fs_entry_migrations.extend(migrations_based_on_unknown_vectors);
    }

    fs_entry_migrations.sort_by_key(|a| a.destination_relative_path.clone());
    fs_entry_migrations
}

fn split_by_score_match(
    rag_ml_config: &RagMlConfig,
    processing_results: &[FileProcessingResult],
) -> (Vec<FileProcessingResult>, Vec<FileProcessingResult>) {
    let threshold = rag_ml_config.valid_embedding_threshold.unwrap();

    processing_results
        .iter()
        .cloned()
        .partition(|cp| cp.score > threshold)
}
