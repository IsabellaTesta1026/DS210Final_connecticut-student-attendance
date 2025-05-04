
mod data;
mod vector;
mod similarity;
mod graph;

use data::load_data;
use vector::build_grouped_feature_vectors;
use similarity::{euclidean_distance, manhattan_distance};
use graph::{build_euclidean_graph, build_manhattan_graph};

fn main() {
    let records = load_data("district_attendance.csv").expect("Failed to load data");
    let grouped_vectors = build_grouped_feature_vectors(&records);

    let group_names: Vec<_> = grouped_vectors.keys().cloned().collect();
    println!("Included student groups ({} total):", group_names.len());
    for g in &group_names {
        println!("- {}", g);
    }

    for (group, vectors) in grouped_vectors {
        println!("\nGroup: {}\n", group);

        let mut distances = vec![];
        let districts: Vec<_> = vectors.keys().cloned().collect();

        for i in 0..districts.len() {
            for j in (i + 1)..districts.len() {
                let a = &districts[i];
                let b = &districts[j];

                let eu_dist = euclidean_distance(&vectors[a], &vectors[b]);
                let man_dist = manhattan_distance(&vectors[a], &vectors[b]);

                distances.push(((a.clone(), b.clone()), eu_dist, man_dist));
            }
        }

        distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        println!("Top 5 most similar (lowest Euclidean distance) districts:\n");
        for (i, ((a, b), eu_dist, man_dist)) in distances.iter().take(5).enumerate() {
            println!("{}. {} <--> {}", i + 1, a, b);
            println!("Euclidean: {:.6}", eu_dist);
            println!("Manhattan: {:.6}", man_dist);
            println!("   {:<25} => {:?}", a, vectors[a]);
            println!("   {:<25} => {:?}\n", b, vectors[b]);
        }

        println!("Top 5 most dissimilar (highest Euclidean distance) districts:\n");
        for (i, ((a, b), eu_dist, man_dist)) in distances.iter().rev().take(5).enumerate() {
            println!("{}. {} <--> {}", i + 1, a, b);
            println!("Euclidean: {:.6}", eu_dist);
            println!("Manhattan: {:.6}", man_dist);
            println!("   {:<25} => {:?}", a, vectors[a]);
            println!("   {:<25} => {:?}\n", b, vectors[b]);
        }

        // Average score calculations per group
        let total_pairs = distances.len() as f64;
        let total_euclidean: f64 = distances.iter().map(|d| d.1).sum();
        let total_manhattan: f64 = distances.iter().map(|d| d.2).sum();
        let avg_euclidean = total_euclidean / total_pairs;
        let avg_manhattan = total_manhattan / total_pairs;

        println!("Average Euclidean distance for group '{}': {:.6}", group, avg_euclidean);
        println!("Average Manhattan distance for group '{}': {:.6}\n", group, avg_manhattan);

        // Build and summarize similarity graphs
        let graph_eu = build_euclidean_graph(&vectors, 0.05);
        let graph_man = build_manhattan_graph(&vectors, 0.05);

        println!("Euclidean graph for group '{}' has {} nodes and {} edges", group, graph_eu.node_count(), graph_eu.edge_count());
        println!("Manhattan graph for group '{}' has {} nodes and {} edges\n", group, graph_man.node_count(), graph_man.edge_count());
    }
}
