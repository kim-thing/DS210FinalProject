extern crate petgraph;
extern crate csv;

use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;
use std::error::Error;
use petgraph::visit::Dfs;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "/Users/kw/Downloads/prevDis.csv";
    let (graph, city_min_connections, disease_min_connections) = build_graph_from_csv(file_path)?;

    let (average_distance, num_pairs, num_edges) = calculate_average_distance(&graph);

    println!("Average distance between pairs of vertices: {}", average_distance);
    println!("Number of pairs: {}", num_pairs);
    println!("Number of edges: {}", num_edges);

    for (city, min_connections) in &city_min_connections {
        println!("City: {:?} - Min connections to diseases: {}", graph[*city], *min_connections);
    }

    for (disease, min_connections) in &disease_min_connections {
        println!("Disease: {:?} - Min connections to cities: {}", graph[*disease], *min_connections);
    }

    Ok(())
}






// fn build_graph_from_csv(file_path: &str) -> Result<(DiGraph<String, u32>, HashMap<NodeIndex, usize>, HashMap<NodeIndex, usize>), Box<dyn Error>> {
//     let mut rdr = csv::Reader::from_path(file_path)?;
//     let mut disease_to_node: HashMap<String, NodeIndex> = HashMap::new();
//     let mut city_to_node: HashMap<String, NodeIndex> = HashMap::new();
//     let mut graph = DiGraph::<String, u32>::new();
//     let mut city_min_connections: HashMap<NodeIndex, usize> = HashMap::new();
//     let mut disease_min_connections: HashMap<NodeIndex, usize> = HashMap::new();

//     for result in rdr.records() {
//         let record = result?;
//         let disease = record.get(0).ok_or("Missing disease")?.to_string();
//         let city = record.get(1).ok_or("Missing city")?.to_string();

//         let disease_node = *disease_to_node.entry(disease.clone()).or_insert_with(|| {
//             let index = graph.add_node(disease.clone());
//             index
//         });

//         let city_node = *city_to_node.entry(city.clone()).or_insert_with(|| {
//             let index = graph.add_node(city.clone());
//             index
//         });

//         graph.add_edge(disease_node, city_node, 0);

//         *city_min_connections.entry(city_node).or_insert(usize::MAX) -= 1;
//         *disease_min_connections.entry(disease_node).or_insert(usize::MAX) -= 1;
//     }

//     Ok((graph, city_min_connections, disease_min_connections))
// }


fn build_graph_from_csv(file_path: &str) -> Result<(DiGraph<String, u32>, HashMap<NodeIndex, usize>, HashMap<NodeIndex, usize>), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(file_path)?;
    let mut disease_to_node: HashMap<String, NodeIndex> = HashMap::new();
    let mut city_to_node: HashMap<String, NodeIndex> = HashMap::new();
    let mut graph = DiGraph::<String, u32>::new();
    let mut city_min_connections: HashMap<NodeIndex, usize> = HashMap::new();
    let mut disease_min_connections: HashMap<NodeIndex, usize> = HashMap::new();

    for result in rdr.records() {
        let record = result?;
        let disease = record.get(0).ok_or("Missing disease")?.to_string();
        let city = record.get(1).ok_or("Missing city")?.to_string();

        let disease_node = *disease_to_node.entry(disease.clone()).or_insert_with(|| {
            let index = graph.add_node(disease.clone());
            disease_min_connections.insert(index, 0); // Initialize disease_min_connections count
            index
        });

        let city_node = *city_to_node.entry(city.clone()).or_insert_with(|| {
            let index = graph.add_node(city.clone());
            city_min_connections.insert(index, 0); // Initialize city_min_connections count
            index
        });

        graph.add_edge(disease_node, city_node, 0);

        *city_min_connections.entry(city_node).or_insert(0) += 1; // Increment city connection count
        *disease_min_connections.entry(disease_node).or_insert(0) += 1; // Increment disease connection count
    }

    Ok((graph, city_min_connections, disease_min_connections))
}



//test
// fn build_graph_from_csv(file_path: &str) -> Result<(DiGraph<String, u32>, HashMap<NodeIndex, usize>, HashMap<NodeIndex, usize>), Box<dyn Error>> {
//     let mut rdr = csv::Reader::from_path(file_path)?;
//     let mut disease_to_node: HashMap<String, NodeIndex> = HashMap::new();
//     let mut city_to_node: HashMap<String, NodeIndex> = HashMap::new();
//     let mut graph = DiGraph::<String, u32>::new();
//     let mut city_min_connections: HashMap<NodeIndex, usize> = HashMap::new();
//     let mut disease_min_connections: HashMap<NodeIndex, usize> = HashMap::new();

//     for result in rdr.records() {
//         let record = result?;
//         let disease = record.get(0).ok_or("Missing disease")?.to_string();
//         let city = record.get(1).ok_or("Missing city")?.to_string();

//         let disease_node = *disease_to_node.entry(disease.clone()).or_insert_with(|| {
//             let index = graph.add_node(disease.clone());
//             city_min_connections.insert(index, 0); // Initialize min connections for disease node
//             disease_min_connections.insert(index, 0); // Initialize min connections for disease node
//             index
//         });

//         let city_node = *city_to_node.entry(city.clone()).or_insert_with(|| {
//             let index = graph.add_node(city.clone());
//             city_min_connections.insert(index, 0); // Initialize min connections for city node
//             disease_min_connections.insert(index, 0); // Initialize min connections for city node
//             index
//         });

//         graph.add_edge(disease_node, city_node, 0);

//         *city_min_connections.entry(city_node).or_insert(0) += 1; // Increment min connections for city node
//         *disease_min_connections.entry(disease_node).or_insert(0) += 1; // Increment min connections for disease node
//     }

//     Ok((graph, city_min_connections, disease_min_connections))
// }

















fn calculate_average_distance(graph: &DiGraph<String, u32>) -> (f64, usize, usize) {
    let mut sum_distances = 0;
    let mut num_pairs = 0;
    let mut num_edges = 0;

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

    for _ in graph.raw_edges() {
        num_edges += 1;
    }

    let average_distance = sum_distances as f64 / num_pairs as f64;
    (average_distance, num_pairs, num_edges)
}

fn shortest_path_length(graph: &DiGraph<String, u32>, start: NodeIndex, end: NodeIndex) -> Option<u32> {
    let mut dfs = Dfs::new(graph, start);
    while let Some(nx) = dfs.next(graph) {
        if nx == end {
            return Some(dfs.discovered.len() as u32);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortest_path_length() {
        let mut graph = DiGraph::<String, u32>::new();
        let a = graph.add_node("A".to_string());
        let b = graph.add_node("B".to_string());
        let c = graph.add_node("C".to_string());
        let d = graph.add_node("D".to_string());

        graph.add_edge(a, b, 0);
        graph.add_edge(b, c, 0);
        graph.add_edge(c, d, 0);

        assert_eq!(shortest_path_length(&graph, a, d), Some(3));
    }
}
