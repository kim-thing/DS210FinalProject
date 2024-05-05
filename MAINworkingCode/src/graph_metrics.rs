//used for "usual distance between pairs of vertices in your graph" question 


use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;

pub fn average_distance_between_vertices(graph: &DiGraph<String, u32>) -> f64 {
    let mut sum_distances = 0;
    let mut num_pairs = 0;

    for disease_node in graph.node_indices() {
        for city_node in graph.node_indices() {
            if disease_node != city_node {
                let distance = shortest_path_length(graph, disease_node, city_node);
                if let Some(dist) = distance {
                    sum_distances += dist;
                    num_pairs += 1;
                }
            }
        }
    }

    if num_pairs > 0 {
        sum_distances as f64 / num_pairs as f64
    } else {
        0.0
    }
}

fn shortest_path_length(graph: &DiGraph<String, u32>, start: NodeIndex, end: NodeIndex) -> Option<u32> {
    let mut dfs = petgraph::visit::Dfs::new(graph, start);
    while let Some(nx) = dfs.next(graph) {
        if nx == end {
            return Some(dfs.discovered.len() as u32);
        }
    }
    None
}

pub fn calculate_min_connections(graph: &DiGraph<String, u32>) -> (HashMap<NodeIndex, usize>, HashMap<NodeIndex, usize>) {
    let mut city_min_connections: HashMap<NodeIndex, usize> = HashMap::new();
    let mut disease_min_connections: HashMap<NodeIndex, usize> = HashMap::new();

    for node in graph.node_indices() {
        city_min_connections.insert(node, usize::MAX);
        disease_min_connections.insert(node, usize::MAX);
    }

    for edge in graph.raw_edges() {
        *city_min_connections.entry(edge.target()).or_insert(usize::MAX) -= 1;
        *disease_min_connections.entry(edge.source()).or_insert(usize::MAX) -= 1;
    }

    (city_min_connections, disease_min_connections)
}
