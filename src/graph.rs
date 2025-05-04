//sets up the graph by declaring the graph type and that it will be undirected (no directed path for edges)
use petgraph::graph::Graph;
use petgraph::Undirected;
//will help map districts to nodes
use std::collections::HashMap;
//The distance mods that declare similarity in order to help build graph
use crate::similarity::euclidean_distance;
use crate::similarity::manhattan_distance;

//Build a similarity graph where edges connect districts with distance below a threshold
//edges are added between districts with Euclidean distance
pub fn build_euclidean_graph(
    vectors: &HashMap<String, Vec<f64>>,
    max_distance: f64,
) -> Graph<String, f64, Undirected> {
    let mut graph = Graph::<String, f64, Undirected>::new_undirected();
    let mut node_indices = HashMap::new();

    // Add districts as nodes
    for district in vectors.keys() {
        let idx = graph.add_node(district.clone());
        node_indices.insert(district, idx);
    }
    //list of districts for compairson reasons
    let districts: Vec<_> = vectors.keys().cloned().collect();
    //compare each unique pair of district
    for i in 0..districts.len() {
        for j in i + 1..districts.len() {
            let a = &districts[i];
            let b = &districts[j];

            //calculate the Euclidean distance for each pair
            let distance = euclidean_distance(&vectors[a], &vectors[b]);
            //if below threshold, add edge
            if distance <= max_distance {
                graph.add_edge(node_indices[a], node_indices[b], distance);
            }
        }
    }
    //returns final graph
    graph
}

///Build a similarity graph using Manhattan distance 
pub fn build_manhattan_graph(
    //inputs the district into vectors
    vectors: &HashMap<String, Vec<f64>>,
    //distance threshold
    max_distance: f64,
) -> Graph<String, f64, Undirected> {
    let mut graph = Graph::<String, f64, Undirected>::new_undirected();
    let mut node_indices = HashMap::new();

    // Add nodes
    for district in vectors.keys() {
        let idx = graph.add_node(district.clone());
        node_indices.insert(district, idx);
    }
    //get list of districts
    let districts: Vec<_> = vectors.keys().cloned().collect();
    //compare all unqiue district pairs
    for i in 0..districts.len() {
        for j in i + 1..districts.len() {
            let a = &districts[i];
            let b = &districts[j];
            //compuation
            let distance = manhattan_distance(&vectors[a], &vectors[b]);
            //add edges as needed when below threshold
            if distance <= max_distance {
                graph.add_edge(node_indices[a], node_indices[b], distance);
            }
        }
    }

    graph
}
//Tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    //test for Euclidean graph creation
    fn test_graph_creation_euclidean() {
        let mut vectors = HashMap::new();
        vectors.insert("A".to_string(), vec![1.0, 2.0]);
        vectors.insert("B".to_string(), vec![1.1, 2.1]);

        let graph = build_euclidean_graph(&vectors, 1.0);
        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edge_count(), 1);
    }

    #[test]
    //test for Manhattan graph creation
    fn test_graph_creation_manhattan() {
        let mut vectors = HashMap::new();
        vectors.insert("X".to_string(), vec![1.0, 1.0]);
        vectors.insert("Y".to_string(), vec![1.2, 1.2]);

        let graph = build_manhattan_graph(&vectors, 1.0);
        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edge_count(), 1);
    }
}
