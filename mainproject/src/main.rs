extern crate petgraph;
extern crate csv;

use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;
use std::error::Error;
use crate::power_law::fit_power_law_distribution;

mod power_law;
mod year;



fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "/Users/kw/Downloads/prevDis.csv";
    let (graph, city_min_connections, disease_min_connections) = build_graph_from_csv(file_path)?;

    let (average_distance, num_pairs, num_edges) = calculate_average_distance(&graph);

    println!("Average distance between pairs of vertices: {}", average_distance);
    println!("Number of pairs: {}", num_pairs);
    println!("Number of edges: {}", num_edges);

    for (city, min_connections) in &city_min_connections {
        println!("County: {:?} - Min connections to diseases: {}", graph[*city], *min_connections);
    }

    for (disease, min_connections) in &disease_min_connections {
        println!("Disease: {:?} - Min connections to counties: {}", graph[*disease], *min_connections);
    }

    // used for the fit_power_law_distribution function
    let data: Vec<usize> = graph
        .node_indices()
        .map(|node| graph.neighbors(node).count())
        .collect();
    
    let result = power_law::fit_power_law_distribution(&data);
    if result {
        println!("The distribution fits a power-law");
    } else {
        println!("The distribution does not fit a power-law");
    }





    let start_node = graph.node_indices().find(|&i| graph[i] == "Diphtheria").unwrap();
    let end_node = graph.node_indices().find(|&i| graph[i] == "Alameda").unwrap();
    if let Some(degree) = degrees_of_separation(&graph, start_node, end_node) {
        println!("Degrees of separation between Diphtheria and Alameda: {}", degree);
    } else {
        println!("No path found between Diphtheria and Alameda.");
    }

    Ok(())
}




fn build_graph_from_csv(file_path: &str) -> Result<(DiGraph<String, u32>, HashMap<NodeIndex, usize>, HashMap<NodeIndex, usize>), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(file_path)?;
    let mut disease_to_node: HashMap<String, NodeIndex> = HashMap::new();
    let mut city_to_node: HashMap<String, NodeIndex> = HashMap::new();
    let mut graph = DiGraph::<String, u32>::new();
    let mut city_min_connections: HashMap<NodeIndex, usize> = HashMap::new();
    let mut disease_min_connections: HashMap<NodeIndex, usize> = HashMap::new();

    for result in rdr.records() {
        //cleans data table
        let record = result?;
        let disease = record.get(0).ok_or("Missing disease")?.to_string();
        let city = record.get(1).ok_or("Missing county")?.to_string();
        let count: u32 = record.get(3).ok_or("Missing count")?.parse()?;
        
        let disease_node = *disease_to_node.entry(disease.clone()).or_insert_with(|| {
            let index = graph.add_node(disease.clone());
            disease_min_connections.insert(index, 0); // starts the disease_min_connections count
            index
        });

        let city_node = *city_to_node.entry(city.clone()).or_insert_with(|| {
            let index = graph.add_node(city.clone());
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

fn calculate_average_distance(graph: &DiGraph<String, u32>) -> (f64, usize, usize) {
    let mut sum_distances = 0;
    let mut num_pairs = 0;
    let mut num_edges = 0;

    //loops through nodes
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
    use petgraph::algo::dijkstra;
    //had to learn ton use dijkstra, orginal code wouldn't pass the test
    let distances = dijkstra(&graph, start, Some(end), |e| *e.weight());

    match distances.get(&end) {
        Some(distance) => Some(*distance),
        None => None,
    }
}


fn degrees_of_separation(graph: &DiGraph<String, u32>, start: NodeIndex, end: NodeIndex) -> Option<usize> {
    use petgraph::algo::dijkstra;
    // Use Dijkstra's algorithm to find the shortest path
    let distances = dijkstra(&graph, start, None, |e| *e.weight());

    match distances.get(&end) {
        Some(distance) => Some(*distance as usize),
        None => None,
    }
}






#[cfg(test)]
mod tests {
    use super::*;


    #[test]
fn test_shortest_path_length() {
    let mut graph = DiGraph::<String, u32>::new();
    let a = graph.add_node("A".to_string());
    let b = graph.add_node("B".to_string());
    let d = graph.add_node("D".to_string());

    graph.add_edge(a, b, 1);
    graph.add_edge(b, d, 1);

    assert_eq!(shortest_path_length(&graph, a, d).unwrap(), 2);
}



    #[test]
    fn test_specific_connections() {
        let mut graph = DiGraph::<String, u32>::new();
        let disease_mumps = graph.add_node("Mumps".to_string());
        let city_san_bernardino = graph.add_node("San Bernardino".to_string());
    
        graph.add_edge(disease_mumps, city_san_bernardino, 0);
    
        assert_eq!(graph.neighbors(disease_mumps).count(), 1);
        assert_eq!(graph.neighbors(city_san_bernardino).count(), 0);
    }


    






    #[test]
    fn test_degrees_of_separation() {
        let mut graph = DiGraph::<String, u32>::new();
        let disease_diphtheria = graph.add_node("Diphtheria".to_string());
        let city_alameda = graph.add_node("Alameda".to_string());
        let city_berkeley = graph.add_node("Berkeley".to_string());
    
        graph.add_edge(disease_diphtheria, city_alameda, 1);
        graph.add_edge(city_alameda, city_berkeley, 1);
    
        // Check degrees of separation between "Diphtheria" and "Alameda"
        assert_eq!(degrees_of_separation(&graph, disease_diphtheria, city_alameda), Some(1));
    
        // Check degrees of separation between "Diphtheria" and "Berkeley"
        assert_eq!(degrees_of_separation(&graph, disease_diphtheria, city_berkeley), Some(2));
    }
    


}

