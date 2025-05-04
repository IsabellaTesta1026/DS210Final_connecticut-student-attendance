//Purpose: Brings all mods together to load in specified data and information.
//Loads attendance data, computes similarity between districts
//for each student group, and builds similarity graphs based on Euclidean and Manhattan distance.
//Displays statistics on most similar/dissimilar district pairs and visual graph summaries.

//bring in mods to use 
mod data;
mod vector;
mod similarity;
mod graph;

//import certain functions to decipher specific data
use data::load_data; //loads all data
use vector::build_grouped_feature_vectors; //loads group data
use similarity::{euclidean_distance, manhattan_distance}; //loads calculations
use graph::{build_euclidean_graph, build_manhattan_graph}; //loads graph calculations

fn main() {
    //step 1: Load the attendence rates from file into vectors
    let records = load_data("district_attendance.csv").expect("Failed to load data");

    //step 2: map said groups: student group -> district -> feature vector (Vec<f64>)
    let grouped_vectors = build_grouped_feature_vectors(&records);
    
    //step 3: collect student group names involved 
    let group_names: Vec<_> = grouped_vectors.keys().cloned().collect();
    //print above names in order to give full breakdown of presented info
    println!("Included student groups ({} total):", group_names.len());
    for g in &group_names {
        println!("- {}", g);
    }
    //step 4: analyze each student group separately
    for (group, vectors) in grouped_vectors {
        println!("\nGroup: {}\n", group);

        let mut distances = vec![];
        let districts: Vec<_> = vectors.keys().cloned().collect();
        
        //Step 5: compare each unique district pair IN EACH GROUP
        for i in 0..districts.len() {
            for j in (i + 1)..districts.len() {
                let a = &districts[i];
                let b = &districts[j];
                
                //Step 6: compute distances for each pair
                let eu_dist = euclidean_distance(&vectors[a], &vectors[b]);
                let man_dist = manhattan_distance(&vectors[a], &vectors[b]);
                //store the end results of calculations
                distances.push(((a.clone(), b.clone()), eu_dist, man_dist));
            }
        }
        
        //step 7: sort the results in ascending order
        distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        //step 8: print the top 5 most similar distances based of the lowest Euclidean distance
        println!("Top 5 most similar (lowest Euclidean distance) districts:\n");
        for (i, ((a, b), eu_dist, man_dist)) in distances.iter().take(5).enumerate() {
            println!("{}. {} <--> {}", i + 1, a, b);
            println!("Euclidean: {:.6}", eu_dist);
            println!("Manhattan: {:.6}", man_dist);
            println!("   {:<25} => {:?}", a, vectors[a]);
            println!("   {:<25} => {:?}\n", b, vectors[b]);
        }
        //step 9: print the top 5 most dissimilar distances based of the highest Euclidean distance
        println!("Top 5 most dissimilar (highest Euclidean distance) districts:\n");
        for (i, ((a, b), eu_dist, man_dist)) in distances.iter().rev().take(5).enumerate() {
            println!("{}. {} <--> {}", i + 1, a, b);
            println!("Euclidean: {:.6}", eu_dist);
            println!("Manhattan: {:.6}", man_dist);
            println!("   {:<25} => {:?}", a, vectors[a]);
            println!("   {:<25} => {:?}\n", b, vectors[b]);
        }

        //step 10: average score calculations per group
        let total_pairs = distances.len() as f64;
        let total_euclidean: f64 = distances.iter().map(|d| d.1).sum();
        let total_manhattan: f64 = distances.iter().map(|d| d.2).sum();
        let avg_euclidean = total_euclidean / total_pairs;
        let avg_manhattan = total_manhattan / total_pairs;

        println!("Average Euclidean distance for group '{}': {:.6}", group, avg_euclidean);
        println!("Average Manhattan distance for group '{}': {:.6}\n", group, avg_manhattan);

        //step 11: build graphs with connected edges 
        let graph_eu = build_euclidean_graph(&vectors, 0.05);
        let graph_man = build_manhattan_graph(&vectors, 0.05);
        
        //step 12: print the stats of each graph
        println!("Euclidean graph for group '{}' has {} nodes and {} edges", group, graph_eu.node_count(), graph_eu.edge_count());
        println!("Manhattan graph for group '{}' has {} nodes and {} edges\n", group, graph_man.node_count(), graph_man.edge_count());
    }
}
