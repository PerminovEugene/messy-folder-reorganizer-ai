use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Cluster {
    pub id: usize,
    pub members: HashSet<usize>,
}

pub fn hierarchical_clustering_auto(distance_matrix: &[Vec<f64>]) -> Vec<Cluster> {
    let n = distance_matrix.len();
    let mut clusters: Vec<Cluster> = (0..n)
        .map(|i| Cluster {
            id: i,
            members: [i].iter().cloned().collect(),
        })
        .collect();

    let mut distances = distance_matrix.to_vec();
    let mut active_clusters: HashSet<usize> = (0..n).collect();
    let mut last_merge_distances = Vec::new();

    while active_clusters.len() > 1 {
        let (c1, c2, min_dist) = find_closest_clusters(&distances, &active_clusters);

        last_merge_distances.push(min_dist);

        if min_dist > 0.5 {
            break;
        }

        merge_clusters(&mut clusters, &mut distances, &mut active_clusters, c1, c2);
    }

    clusters
        .into_iter()
        .filter(|c| active_clusters.contains(&c.id))
        .collect()
}

fn find_closest_clusters(
    distances: &[Vec<f64>],
    active_clusters: &HashSet<usize>,
) -> (usize, usize, f64) {
    let mut min_dist = f64::INFINITY;
    let mut best_pair = (0, 1);

    for &i in active_clusters {
        for &j in active_clusters {
            if i != j {
                let d = distances[i][j];
                if d < min_dist {
                    min_dist = d;
                    best_pair = (i, j);
                }
            }
        }
    }

    (best_pair.0, best_pair.1, min_dist)
}

fn merge_clusters(
    clusters: &mut [Cluster],
    distances: &mut [Vec<f64>],
    active_clusters: &mut HashSet<usize>,
    c1: usize,
    c2: usize,
) {
    let mut new_members = clusters[c1].members.clone();
    new_members.extend(&clusters[c2].members);

    let new_cluster = Cluster {
        id: c1,
        members: new_members,
    };

    clusters[c1] = new_cluster.clone();

    for &i in active_clusters.iter() {
        if i != c1 && i != c2 {
            let d1 = distances[c1][i];
            let d2 = distances[c2][i];
            distances[c1][i] = (d1 + d2) / 2.0;
            distances[i][c1] = distances[c1][i];
        }
    }

    active_clusters.remove(&c2);
}
