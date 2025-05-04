// ===== src/graph.rs =====
use petgraph::graph::Graph;
use petgraph::Undirected;
use std::collections::HashMap;
use crate::similarity::euclidean_distance;
use crate::similarity::manhattan_distance;

/// Build a similarity graph where edges connect districts with distance below a threshold
pub fn build_euclidean_graph(
    vectors: &HashMap<String, Vec<f64>>,
    max_distance: f64,
) -> Graph<String, f64, Undirected> {
    let mut graph = Graph::<String, f64, Undirected>::new_undirected();
    let mut node_indices = HashMap::new();

    // Add nodes
    for district in vectors.keys() {
        let idx = graph.add_node(district.clone());
        node_indices.insert(district, idx);
    }

    let districts: Vec<_> = vectors.keys().cloned().collect();
    for i in 0..districts.len() {
        for j in i + 1..districts.len() {
            let a = &districts[i];
            let b = &districts[j];
            let distance = euclidean_distance(&vectors[a], &vectors[b]);

            if distance <= max_distance {
                graph.add_edge(node_indices[a], node_indices[b], distance);
            }
        }
    }

    graph
}

/// Build a similarity graph using Manhattan distance instead of Euclidean
pub fn build_manhattan_graph(
    vectors: &HashMap<String, Vec<f64>>,
    max_distance: f64,
) -> Graph<String, f64, Undirected> {
    let mut graph = Graph::<String, f64, Undirected>::new_undirected();
    let mut node_indices = HashMap::new();

    // Add nodes
    for district in vectors.keys() {
        let idx = graph.add_node(district.clone());
        node_indices.insert(district, idx);
    }

    let districts: Vec<_> = vectors.keys().cloned().collect();
    for i in 0..districts.len() {
        for j in i + 1..districts.len() {
            let a = &districts[i];
            let b = &districts[j];
            let distance = manhattan_distance(&vectors[a], &vectors[b]);

            if distance <= max_distance {
                graph.add_edge(node_indices[a], node_indices[b], distance);
            }
        }
    }

    graph
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_graph_creation_euclidean() {
        let mut vectors = HashMap::new();
        vectors.insert("A".to_string(), vec![1.0, 2.0]);
        vectors.insert("B".to_string(), vec![1.1, 2.1]);

        let graph = build_euclidean_graph(&vectors, 1.0);
        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edge_count(), 1);
    }

    #[test]
    fn test_graph_creation_manhattan() {
        let mut vectors = HashMap::new();
        vectors.insert("X".to_string(), vec![1.0, 1.0]);
        vectors.insert("Y".to_string(), vec![1.2, 1.2]);

        let graph = build_manhattan_graph(&vectors, 1.0);
        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edge_count(), 1);
    }
}
