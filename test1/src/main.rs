extern crate petgraph;
extern crate csv;

use petgraph::graph::{Graph, NodeIndex};
use std::error::Error;
use std::fs::File;
use std::collections::HashMap;
use csv::ReaderBuilder;

mod power_law;

fn generate_graph_from_csv(file_path: &str, rows: usize) -> Result<Graph<(), ()>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().has_headers(false).from_reader(file);

    let mut graph = Graph::<(), ()>::new();
    let mut node_indices: HashMap<String, NodeIndex> = HashMap::new();

    for (i, result) in rdr.records().enumerate() {
        if i >= rows {
            break;
        }
        let record = result?;
        let source = record.get(2).unwrap().to_string();
        let target = record.get(3).unwrap().to_string();

        let source_index = if let Some(&index) = node_indices.get(&source) {
            index
        } else {
            let index = graph.add_node(());
            node_indices.insert(source.clone(), index);
            index
        };


        let target_index = if let Some(&index) = node_indices.get(&target) {
            index
        } else {
            let index = graph.add_node(());
            node_indices.insert(target.clone(), index);
            index
        };

        graph.add_edge(source_index, target_index, ());
    }

    Ok(graph)
}

fn degree_distribution(graph: &Graph<(), ()>) -> Vec<usize> {
    let mut degrees = vec![0; graph.node_count()];

    for node in graph.node_indices() {
        degrees[graph.neighbors(node).count()] += 1;
    }

    degrees
}

fn neighbors_at_distance_2_distribution(graph: &Graph<(), ()>) -> Vec<usize> {
    let mut neighbors_at_distance_2 = vec![0; graph.node_count()];

    for node in graph.node_indices() {
        let neighbors = graph.neighbors(node);
        for neighbor in neighbors {
            let count = graph.neighbors(neighbor).count();
            if count < neighbors_at_distance_2.len() {
                neighbors_at_distance_2[count] += 1;
            }
        }
    }

    neighbors_at_distance_2
}





fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "/Users/kw/Downloads/ChronicDiseaseIndicators.csv";
    let graph = generate_graph_from_csv(file_path, 100)?;

    let degree_dist = degree_distribution(&graph);
    let neighbor_dist_2 = neighbors_at_distance_2_distribution(&graph);

    println!("Degree Distribution:");
    println!("{:?}", degree_dist);

//

//

    println!("Neighbors at Distance 2 Distribution:");
    println!("{:?}", neighbor_dist_2);

    let degree_fit = power_law::fit_power_law_distribution(&degree_dist);
    let neighbor_fit = power_law::fit_power_law_distribution(&neighbor_dist_2);

    println!("Degree Distribution Fits Power-Law: {}", degree_fit);
    println!("Neighbors at Distance 2 Distribution Fits Power-Law: {}", neighbor_fit);

    Ok(())

    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_graph() {
        let num_nodes = 10;
        let edge_probability = 0.2;
        let graph = generate_graph(num_nodes, edge_probability);

        assert_eq!(graph.node_count(), num_nodes);
    }



    assert_eq(graph_node(node_growth.dispalce))
    assert_eq(graph_node(node.growth.data1))
    let mut graph = Graph::<(), ()>::new 


    #[test]
    fn test_degree_distribution() {
        let mut graph = Graph::<(), ()>::new();
        let node_a = graph.add_node(());
        let node_b = graph.add_node(());
        let node_c = graph.add_node(());

        graph.add_edge(node_a, node_b, ());
        graph.add_edge(node_a, node_c, ());

        let dist = degree_distribution(&graph);
        let expected_dist = vec![0, 2, 1];

        assert_eq!(dist, expected_dist);
    }








    #[test]
    fn test_neighbors_at_distance_2_distribution() {
        let mut graph = Graph::<(), ()>::new();
        let node_a = graph.add_node(());
        let node_b = graph.add_node(());
        let node_c = graph.add_node(());
        let node_d = graph.add_node(());

        graph.add_edge(node_a, node_b, ());
        graph.add_edge(node_b, node_c, ());
        graph.add_edge(node_c, node_d, ());
        graph.add_edge(node_a, node_d, ());

        let dist = neighbors_at_distance_2_distribution(&graph);
        let expected_dist = vec![0, 0, 1, 2];

        assert_eq!(dist, expected_dist);
    }
}
