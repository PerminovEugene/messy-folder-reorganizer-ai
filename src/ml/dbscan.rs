use linfa::prelude::*;
use linfa::traits::{Fit, Predict, Transformer};
use linfa::{Dataset, DatasetBase};
use linfa_clustering::Dbscan;
use ndarray::Array2;
use ndarray::{Array1, ArrayView2};

// Example config struct
pub struct DBSCANConfig {
    pub min_points: usize,
    pub tolerance: f64,
}

pub async fn cluster_vectors_dbscan(config: DBSCANConfig, embeddings: Vec<Vec<f32>>) {
    let array = convert_to_array2(embeddings);

    // Convert embeddings into DatasetBase
    let dataset = embeddings_to_dataset(&embeddings);
    // let dataset: DatasetBase<_, _> = Dataset::new(array, ());

    let eps = 0.5; // Радиус соседства
    let min_points = 5; // Минимальное количество точек для формирования кластера

    // // Запуск DBSCAN
    // let model = Dbscan::params(eps)
    //     .min_points(min_points)
    //     .fit(&dataset)
    //     .expect("DBSCAN clustering failed");

    // 3. Perform DBSCAN using transform(...)
    //    transform(...) returns a LabelMembership object
    let membership = Dbscan::params(config.min_points)
        .tolerance(config.tolerance)
        .transform(dataset)
        .expect("DBSCAN transform failed");

    // 4. Get the number of clusters found
    let n_clusters = membership.n_clusters();

    // 5. Access the label for each point (Some(cluster_id) or None for noise)
    let labels = membership.labels();

    println!("Found {} clusters", n_clusters);
    println!("Cluster labels: {:?}", labels);
}

/// Convert Vec<Vec<f32>> to ndarray::Array2<f64>
fn convert_to_array2(vectors: Vec<Vec<f32>>) -> Array2<f64> {
    let n_samples = vectors.len();
    let n_features = vectors[0].len();

    // Flatten Vec<Vec<f32>> → Vec<f64>
    let flattened: Vec<f64> = vectors
        .into_iter()
        .flatten()
        .map(|x| x as f64) // f32 → f64
        .collect();

    // Construct Array2 from the flattened vector
    Array2::from_shape_vec((n_samples, n_features), flattened)
        .expect("Failed to create Array2 from vectors")
}

// use linfa::prelude::*;
// use linfa_clustering::Dbscan;
// use ndarray::{Array2, Array1};

// fn main() {
//     // Example: Creating embeddings manually (100 samples, 1024 dimensions each)
//     let embeddings: Vec<Vec<f32>> = (0..100)
//         .map(|_| (0..1024).map(|_| rand::random::<f32>()).collect())
//         .collect();

//     // Convert embeddings into DatasetBase
//     let dataset = embeddings_to_dataset(embeddings);

//     // Define DBSCAN parameters
//     let eps = 0.5; // Neighborhood radius
//     let min_points = 5; // Minimum points in cluster

//     // Run DBSCAN
//     let model = Dbscan::params(eps)
//         .min_points(min_points)
//         .fit(&dataset)
//         .expect("DBSCAN clustering failed");

//     // Print cluster assignments
//     for (i, label) in model.labels().iter().enumerate() {
//         println!("Sample {} -> Cluster {:?}", i, label);
//     }
// }

fn embeddings_to_dataset(embeddings: &Vec<Vec<f32>>) -> DatasetBase<Array2<f64>, Array1<usize>> {
    let n_samples = embeddings.len();
    let dim = embeddings[0].len();

    // Flatten Vec<Vec<f32>> into Vec<f64>
    let flattened: Vec<f64> = embeddings.iter().flatten().map(|&x| x as f64).collect();

    // Convert to ndarray Array2<f64>
    let records = Array2::from_shape_vec((n_samples, dim), flattened)
        .expect("Failed to reshape embeddings into ndarray");

    // Create dummy targets (not used in DBSCAN)
    let targets = Array1::zeros(n_samples);

    // ✅ Correct way to construct DatasetBase
    Dataset::from((records, targets))
}
