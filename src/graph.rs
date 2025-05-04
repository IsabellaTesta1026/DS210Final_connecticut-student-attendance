//Purpose: Constructs undirected similarity graphs between districts based on their attendance data.
//Supports both Euclidean and Manhattan distance thresholds for defining similarity.

//sets up the graph by declaring the graph type and that it will be undirected (no directed path for edges)
use petgraph::graph::Graph;
use petgraph::Undirected;
//will help map districts to nodes
use std::collections::HashMap;
//The distance mods that declare similarity in order to help build graph
use crate::similarity::euclidean_distance;
use crate::similarity::manhattan_distance;

//Function 1: build_euclidean_graph
//Constructs an undirected graph where nodes are districts and edges are placed between
//districts whose Euclidean distance is below the specified threshold.
//Inputs:
    //`vectors`: A map of district names to their attendance feature vectors
    //`max_distance`: Threshold under which an edge is formed
//Output:
    //A `petgraph::Graph` with nodes as district names and edges weighted by distance
//Logic:
    //Adds all districts as nodes
    //Computes pairwise Euclidean distance
    //Adds an edge between two nodes if the distance is below threshold
pub fn build_euclidean_graph(
    vectors: &HashMap<String, Vec<f64>>,
    max_distance: f64,
) -> Graph<String, f64, Undirected> {
    let mut graph = Graph::<String, f64, Undirected>::new_undirected();
    let mut node_indices = HashMap::new();

    // Add each district as a graph node
    for district in vectors.keys() {
        let idx = graph.add_node(district.clone());
        node_indices.insert(district, idx);
    }
    //list of districts for comparison reasons
    let districts: Vec<_> = vectors.keys().cloned().collect();
    //compare each unique pair of districts
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

//Function 2: build_manhattan_graph
//Constructs an undirected graph where nodes are districts and edges are added
//between those whose Manhattan distance is below a set threshold.
//Inputs:
    //`vectors`: A map of district names to their attendance feature vectors
    //`max_distance`: Distance cutoff for edge inclusion
//Output:
    //An undirected graph with edges weighted by Manhattan distance
//Logic:
    //Adds all districts as nodes
    //For each district pair, calculates Manhattan distance
    //Adds an edge if distance is below threshold
pub fn build_manhattan_graph(
    //inputs the district into vectors
    vectors: &HashMap<String, Vec<f64>>,
    //distance threshold
    max_distance: f64,
) -> Graph<String, f64, Undirected> {
    let mut graph = Graph::<String, f64, Undirected>::new_undirected();
    let mut node_indices = HashMap::new();

    // Each district is added as a graph node
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
