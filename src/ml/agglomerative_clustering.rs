use crate::{
    app_core::sources_processor::FileProcessingResult, configuration::config::RagMlConfig,
    console::table::print_clustering_table,
    ml::hierarchical_clustering::hierarchical_clustering_auto,
};

use super::hierarchical_clustering::Cluster;

pub async fn cluster_vectors_hierarchical(
    config: &RagMlConfig,
    vectors: &[FileProcessingResult],
) -> Vec<Cluster> {
    let pathes = vectors
        .iter()
        .map(|x| x.source_file_name.clone())
        .collect::<Vec<_>>();

    // build source matrix
    let source_matrix: Vec<Vec<f32>> = vectors.iter().map(|ms| ms.vector.clone()).collect();

    // normalize vectors in matrix
    let normalized_vectors = normalize_matrix(source_matrix);

    // convert to distance matrix
    let distance_matrix = cosine_distance_matrix(&normalized_vectors);

    // clustering
    let clusters = hierarchical_clustering_auto(&distance_matrix, config);

    print_clustering_table(&clusters, &pathes);

    clusters
}

fn normalize_matrix(vectors: Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    vectors.into_iter().map(normalize_vector).collect()
}

fn normalize_vector(v: Vec<f32>) -> Vec<f32> {
    let norm = (v.iter().map(|&x| x * x).sum::<f32>()).sqrt();
    if norm == 0.0 {
        return vec![0.0; v.len()]; // avoid division by zero
    }
    v.iter().map(|&x| x / norm).collect()
}

fn cosine_distance_matrix(vectors: &[Vec<f32>]) -> Vec<Vec<f64>> {
    let n = vectors.len();
    let mut distance_matrix = vec![vec![0.0; n]; n]; // Initialize with zeros

    for i in 0..n {
        for j in 0..n {
            if i != j {
                let similarity = cosine_similarity(&vectors[i], &vectors[j]);
                distance_matrix[i][j] = 1.0 - similarity as f64;
            }
        }
    }

    distance_matrix
}

fn cosine_similarity(vec1: &[f32], vec2: &[f32]) -> f32 {
    let dot_product: f32 = vec1.iter().zip(vec2.iter()).map(|(a, b)| a * b).sum();
    let norm1: f32 = vec1.iter().map(|v| v * v).sum::<f32>().sqrt();
    let norm2: f32 = vec2.iter().map(|v| v * v).sum::<f32>().sqrt();

    if norm1 == 0.0 || norm2 == 0.0 {
        0.0
    } else {
        dot_product / (norm1 * norm2)
    }
}
