//added nodes that account for year 

use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::algo::connected_components;
use std::collections::{HashMap, HashSet};
use std::error::Error;

pub fn build_graph_from_csv(file_path: &str) -> Result<(DiGraph<(String, u32), u32>, HashMap<NodeIndex, usize>, HashMap<NodeIndex, usize>), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(file_path)?;
    let mut disease_to_node: HashMap<String, NodeIndex> = HashMap::new();
    let mut city_to_node: HashMap<String, NodeIndex> = HashMap::new();
    let mut graph = DiGraph::<(String, u32), u32>::new();
    let mut city_min_connections: HashMap<NodeIndex, usize> = HashMap::new();
    let mut disease_min_connections: HashMap<NodeIndex, usize> = HashMap::new();

    for result in rdr.records() {
        let record = result?;
        let disease = record.get(0).ok_or("Missing disease")?.to_string();
        let city = record.get(1).ok_or("Missing county")?.to_string();
        let year: u32 = record.get(2).ok_or("Missing year")?.parse()?;
        let count: u32 = record.get(3).ok_or("Missing count")?.parse()?;
        
        let disease_node = *disease_to_node.entry(disease.clone()).or_insert_with(|| {
            let index = graph.add_node((disease.clone(), year));
            disease_min_connections.insert(index, 0); // starts the disease_min_connections count
            index
        });

        let city_node = *city_to_node.entry(city.clone()).or_insert_with(|| {
            let index = graph.add_node((city.clone(), year));
            city_min_connections.insert(index, 0); // starts the city_min_connections count
            index
        });

        graph.add_edge(disease_node, city_node, count);

        //increments for city and disease
        *city_min_connections.entry(city_node).or_insert(0) += 1; 
        *disease_min_connections.entry(disease_node).or_insert(0) += 1; 
    }

    Ok((graph, city_min_connections, disease_min_connections))
}

pub fn calculate_average_distance(graph: &DiGraph<(String, u32), u32>) -> f64 {
    let mut sum_distances = 0;
    let mut num_pairs = 0;

    for start_node in graph.node_indices() {
        for end_node in graph.node_indices() {
            if start_node != end_node {
                let distance = shortest_path_length(graph, start_node, end_node);
                if let Some(dist) = distance {
                    sum_distances += dist;
                    num_pairs += 1;
                }
            }
        }
    }

    sum_distances as f64 / num_pairs as f64
}

pub fn average_degree(graph: &DiGraph<(String, u32), u32>) -> f64 {
    let mut total_degree = 0;

    for node in graph.node_indices() {
        total_degree += graph.neighbors(node).count();
    }

    total_degree as f64 / graph.node_count() as f64
}

pub fn average_neighbor_degree(graph: &DiGraph<(String, u32), u32>, distance: usize) -> f64 {
    let mut total_neighbor_degree = 0;
    let mut num_nodes = 0;

    for node in graph.node_indices() {
        let neighbors_at_distance = bfs_neighbors_at_distance(&graph, node, distance);
        for neighbor in neighbors_at_distance {
            total_neighbor_degree += graph.neighbors(neighbor).count();
            num_nodes += 1;
        }
    }

    total_neighbor_degree as f64 / num_nodes as f64
}

pub fn friend_recommendation_similarity(graph: &DiGraph<(String, u32), u32>) -> (NodeIndex, NodeIndex) {
    let mut max_similarity = 0.0;
    let mut most_similar_nodes = (NodeIndex::end(), NodeIndex::end());

    for node1 in graph.node_indices() {
        for node2 in graph.node_indices() {
            if node1 != node2 {
                let sim = jaccard_similarity(&graph, node1, node2);
                if sim > max_similarity {
                    max_similarity = sim;
                    most_similar_nodes = (node1, node2);
                }
            }
        }
    }

    most_similar_nodes
}

fn bfs_neighbors_at_distance(graph: &DiGraph<(String, u32), u32>, node: NodeIndex, distance: usize) -> HashSet<NodeIndex> {
    let mut visited = HashSet::new();
    let mut queue = Vec::new();
    let mut current_distance = 0;

    visited.insert(node);
    queue.push((node, current_distance));

    while !queue.is_empty() {
        let (current_node, current_distance) = queue.remove(0);
        if current_distance < distance {
            for neighbor in graph.neighbors(current_node) {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push((neighbor, current_distance + 1));
                }
            }
        }
    }

    visited
}

fn shortest_path_length(graph: &DiGraph<(String, u32), u32>, start: NodeIndex, end: NodeIndex) -> Option<u32> {
    use petgraph::algo::dijkstra;
    let distances = dijkstra(&graph, start, Some(end), |e| *e.weight());
    distances.get(&end).cloned()
}

fn jaccard_similarity(graph: &DiGraph<(String, u32), u32>, node1: NodeIndex, node2: NodeIndex) -> f64 {
    let neighbors1: HashSet<NodeIndex> = graph.neighbors(node1).collect();
    let neighbors2: HashSet<NodeIndex> = graph.neighbors(node2).collect();

    let intersection_size = neighbors1.intersection(&neighbors2).count() as f64;
    let union_size = neighbors1.union(&neighbors2).count() as f64;

    if union_size == 0.0 {
        0.0
    } else {
        intersection_size / union_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_average_distance() {
        let mut graph = DiGraph::<(String, u32), u32>::new();
        let a = graph.add_node(("A".to_string(), 2020));
        let b = graph.add_node(("B".to_string(), 2020));
        let c = graph.add_node(("C".to_string(), 2020));

        graph.add_edge(a, b, 1);
        graph.add_edge(b, c, 1);

        assert_eq!(calculate_average_distance(&graph), 1.0);
    }

    #[test]
    fn test_average_degree() {
        let mut graph = DiGraph::<(String, u32), u32>::new();
        let a = graph.add_node(("A".to_string(), 2020));
        let b = graph.add_node(("B".to_string(), 2020));
        let c = graph.add_node(("C".to_string(), 2020));

        graph.add_edge(a, b, 1);
        graph.add_edge(a, c, 1);

        assert_eq!(average_degree(&graph), 2.0);
    }

    #[test]
    fn test_average_neighbor_degree() {
        let mut graph = DiGraph::<(String, u32), u32>::new();
        let a = graph.add_node(("A".to_string(), 2020));
        let b = graph.add_node(("B".to_string(), 2020));
        let c = graph.add_node(("C".to_string(), 2020));

        graph.add_edge(a, b, 1);
        graph.add_edge(a, c, 1);
        graph.add_edge(b, c, 1);

        assert_eq!(average_neighbor_degree(&graph, 1), 2.0);
        assert_eq!(average_neighbor_degree(&graph, 2), 1.0);
    }

    #[test]
    fn test_friend_recommendation_similarity() {
        let mut graph = DiGraph::<(String, u32), u32>::new();
        let a = graph.add_node(("A".to_string(), 2020));
        let b = graph.add_node(("B".to_string(), 2020));
        let c = graph.add_node(("C".to_string(), 2020));
        let d = graph.add_node(("D".to_string(), 2020));

        graph.add_edge(a, b, 1);
        graph.add_edge(a, c, 1);
        graph.add_edge(b, d, 1);
        graph.add_edge(c, d, 1);

        let (node1, node2) = friend_recommendation_similarity(&graph);
        assert_eq!(node1, c);
        assert_eq!(node2, b);
    }
}
