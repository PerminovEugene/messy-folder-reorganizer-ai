use std::collections::{hash_map, HashMap};
use std::path::{Path, PathBuf};

use crate::configuration::args::ProcessArgs;
use crate::configuration::config::{EmbeddingModelConfig, LLMModelConfig, RagMlConfig};
use crate::console::messages::{
    print_asking_llm_for_new_folder_names, print_clustering_unknown_vectors,
};
use crate::console::table::print_clusters_ai_proposed_names;
use crate::db::qdrant::fs_entry::search::ClosestFsEntry;
use crate::errors::app_error::AppError;
use crate::fs::migration::fs_entry_migration::FsEntryMigration;
use crate::ml::agglomerative_clustering::cluster_vectors_hierarchical;

use super::cluster_processor::process_clusters;
use super::sources_processor::{find_closest_paths, FileProcessingResult};

pub async fn create_migration_plan(
    llm_config: &LLMModelConfig,
    rag_ml_config: &RagMlConfig,
    args: &ProcessArgs,
    processing_results: &mut [FileProcessingResult],
    session_id: &str,
    embedding_config: &EmbeddingModelConfig,
) -> Result<Vec<FsEntryMigration>, AppError> {
    let (high_confidence_results, low_confidence_results) =
        split_by_score_match(rag_ml_config, processing_results);

    let mut fs_entry_migrations: Vec<FsEntryMigration> =
        build_migrations(args, high_confidence_results);

    if !low_confidence_results.is_empty() {
        print_clustering_unknown_vectors();

        let clusters = cluster_vectors_hierarchical(rag_ml_config, &low_confidence_results).await;

        print_asking_llm_for_new_folder_names();

        let cluster_id_to_path_hash =
            process_clusters(llm_config, args, &clusters, &low_confidence_results).await;

        print_clusters_ai_proposed_names(&cluster_id_to_path_hash);

        let mut updated_hash_map = HashMap::new();

        /// for each generated folder name:
        ///  generate embedding, find closest existing folder match, add path to that folder and generated folder name
        for (cluster_id, cluster_folder_name) in &cluster_id_to_path_hash {
            let vec = vec![cluster_folder_name];
            let closest_paths_for_new_folders: Vec<ClosestFsEntry> =
                find_closest_paths(embedding_config, args, session_id, &vec).await?;
            let p = &closest_paths_for_new_folders[0].absolute_path;
            let abs_path_in_dest_to_new_folder = PathBuf::from(&p);
            let relative_path_to_new_folder = abs_path_in_dest_to_new_folder
                .strip_prefix(std::env::current_dir().unwrap())
                .map_err(|e| AppError::FileError(format!("Failed to strip prefix: {e}")))?
                .to_path_buf()
                .to_string_lossy()
                .to_string();
            updated_hash_map.insert(cluster_id, relative_path_to_new_folder);
        }

        let migrations_based_on_unknown_vectors: Vec<FsEntryMigration> = clusters
            .iter()
            .flat_map(|cluster| {
                cluster.members.iter().map(|&member| {
                    let unknown = &low_confidence_results[member];
                    FsEntryMigration {
                        source_file_name: unknown.source_file_name.clone(),
                        destination_file_name: unknown.source_file_name.clone(),
                        source_relative_path: unknown.source_relative_path.clone(),
                        destination_relative_path: updated_hash_map[&cluster.id].clone(),
                        source_arg: args.source.clone(),
                        destination_arg: args.destination.clone(),
                    }
                })
            })
            .collect();

        fs_entry_migrations.extend(migrations_based_on_unknown_vectors);
    }

    fs_entry_migrations.sort_by_key(|a| a.destination_relative_path.clone());
    Ok(fs_entry_migrations)
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

fn build_migrations(
    args: &ProcessArgs,
    processing_results: Vec<FileProcessingResult>,
) -> Vec<FsEntryMigration> {
    processing_results
        .iter()
        .map(|x| FsEntryMigration {
            source_file_name: x.source_file_name.clone(),
            destination_file_name: x.source_file_name.clone(),
            destination_relative_path: x.destination_relative_path.clone(),
            source_relative_path: x.source_relative_path.clone(),
            source_arg: args.source.clone(),
            destination_arg: args.destination.clone(),
        })
        .collect()
}
