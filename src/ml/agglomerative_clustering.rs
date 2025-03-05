use linfa::prelude::*;
use linfa_kernel::{Kernel, KernelMethod};
use ndarray::{Array1, Array2, Ix1};

use linfa::traits::Transformer;
use linfa::Dataset;
use linfa_hierarchical::HierarchicalCluster;

use crate::workflow::sources_processor::ProcessResult;

pub async fn cluster_vectors_hierarchical(
    // config: DBSCANConfig,
    vectors: Vec<&ProcessResult>, //,Vec<Vec<f32>>,
) {
    dbg!(&vectors);
    // Convert embeddings into DatasetBase
    let dataset = embeddings_dataset(vectors);

    let kernel = Kernel::params()
        .method(KernelMethod::Gaussian(1.0))
        .transform(dataset.records().view());

    let kernel = HierarchicalCluster::default()
        // .num_clusters(n_clusters)
        .transform(kernel)
        .unwrap();

    // Print cluster assignments
    for (id, target) in kernel.targets().iter().zip(dataset.targets().into_iter()) {
        let name = match *target {
            0 => "setosa",
            1 => "versicolor",
            2 => "virginica",
            _ => unreachable!(),
        };

        print!("({} {}) ", id, name);
    }
}

/// Converts `Vec<Vec<f32>>` into a `Dataset<f64, usize, Ix1>`
pub fn embeddings_dataset(embeddings: Vec<&ProcessResult>) -> Dataset<f64, usize, Ix1> {
    let n_samples = embeddings.len();
    let dim = embeddings[0].vector.len();

    // Flatten Vec<Vec<f32>> into a single Vec<f64>
    let flattened: Vec<f64> = embeddings
        .into_iter()
        .flat_map(|ms| ms.vector.clone())
        .map(|x| x as f64)
        .collect();

    // Convert into an ndarray Array2<f64>
    let data: Array2<f64> = Array2::from_shape_vec((n_samples, dim), flattened)
        .expect("Failed to reshape embeddings into ndarray");

    // Create dummy targets (not used in clustering but required by Dataset)
    let targets: Array1<usize> = Array1::zeros(n_samples);

    // Create feature names: "feature_0", "feature_1", ..., "feature_N"
    let feature_names: Vec<String> = (0..dim).map(|i| format!("feature_{}", i)).collect();

    // ✅ Ensure the dataset is correctly constructed
    Dataset::new(data, targets)
        .map_targets(|x| *x as usize)
        .with_feature_names(feature_names)
}
