use crate::configuration::args::Args;
use crate::configuration::config::Config;
use crate::ml::agglomerative_clustering::cluster_vectors_hierarchical;

use super::sources_processor::ProcessResult;

pub async fn create_folder_for_unknown_files(
    config: &Config,
    args: &Args,
    process_result: &mut Vec<ProcessResult>,
) {
    let (processed_vectors, unknown_vectors): (Vec<_>, Vec<_>) =
        process_result.iter().partition(|&cp| cp.score < 0.2);

    // create_folder_for_names_for_

    if !unknown_vectors.is_empty() {
        println!("Clustering unknown vectors");
        let clusters: () = cluster_vectors_hierarchical(unknown_vectors).await;
        println!("Clusters: {:?}", clusters);
    }

    // .collect();
    // .partition(|&cp| cp.score < 0.2);

    // let plan: Vec<FilesReorganisationPlan> = processed_vectors
    //     .iter()
    //     .zip(file_names.into_iter())
    //     .map(|(dest_path, source_file_name)| FilesReorganisationPlan {
    //         original: source_file_name.clone(),
    //         new_path: dest_path.source_file_name.clone(),
    //     })
    //     .collect();
    // let json_plan = serde_json::to_string_pretty(&plan).unwrap();

    // create_plan_file(json_plan);
}

// async fn cluster_vectors() {
//     let data = Array2::from_shape_vec(
//         (n_samples, n_features),
//         embeddings.into_iter().flatten().collect(),
//     )
//     .unwrap();

//     cluster_vectors_hierarchical(data);

//     println!("Cluster labels: {:?}", dbscan.predict(&data));
// }
