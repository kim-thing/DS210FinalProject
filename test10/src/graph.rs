use csv::ReaderBuilder;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::algo::dijkstra;
use std::error::Error;
use std::fs::File;

pub struct DiseaseSymptomNetwork {
    graph: DiGraph<String, ()>,
}

impl DiseaseSymptomNetwork {
    pub fn from_csv(filename: &str) -> Result<Self, Box<dyn Error>> {
        let file = File::open(filename)?;
        let mut reader = ReaderBuilder::new().has_headers(true).from_reader(file);

        let mut graph = DiGraph::<String, ()>::new();

        for result in reader.records() {
            let record = result?;
            let disease = record.get(24).unwrap().to_string(); // Assuming classification is at index 24
            let symptoms: Vec<String> = record.iter().take(24).map(|s| s.to_string()).collect();

            for symptom in symptoms {
                let symptom_index = if let Some(node) = graph.node_indices().find(|i| &graph[*i] == &symptom) {
                    node
                } else {
                    graph.add_node(symptom.clone())
                };
                let disease_index = graph.add_node(disease.clone());

                graph.add_edge(symptom_index, disease_index, ());
            }
        }

        Ok(DiseaseSymptomNetwork { graph })
    }

    pub fn average_distances(&self) -> Vec<f64> {
        let mut avg_distances = Vec::new();

        for node in self.graph.node_indices() {
            let distances = dijkstra(&self.graph, node, None, |_| 1);
            let total_distance: f64 = distances.values().map(|&d| d as f64).sum();
            let avg_distance = total_distance / (distances.len() - 1) as f64; // Subtract 1 to exclude the node itself
            avg_distances.push(avg_distance);
        }

        avg_distances
    }
}

