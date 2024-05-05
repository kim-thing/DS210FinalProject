use csv::Reader;
use petgraph::graph::{DiGraph, NodeIndex};

pub type Graph = DiGraph<String, f32>;

pub fn load_graph(file_path: &str) -> Result<Graph, Box<dyn std::error::Error>> {
    let mut graph = Graph::new();

    let mut reader = Reader::from_path(file_path)?;
    for result in reader.records() {
        let record = result?;
        let mut iter = record.iter().filter_map(|field| {
            if field.is_empty() {
                None
            } else {
                Some(field.to_string())
            }
        });

        let disease = iter.next().unwrap();
        let mut symptoms = iter.map(|s| s.parse::<f32>().unwrap());
        let node_index = graph.add_node(disease);
        while let Some(symptom) = symptoms.next() {
            graph.add_edge(node_index, node_index, symptom);
        }
    }

    Ok(graph)
}

pub fn average_distances(graph: &Graph) -> f32 {
    // Implementation of average distance calculation
    unimplemented!()
}
