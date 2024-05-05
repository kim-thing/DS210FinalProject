use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::algo::dijkstra;
use petgraph::prelude::*;

pub struct Graph {
    graph: DiGraph<(), f64>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            graph: DiGraph::new(),
        }
    }

    pub fn add_edge(&mut self, source: usize, target: usize, weight: f64) {
        let source_idx = NodeIndex::new(source);
        let target_idx = NodeIndex::new(target);
        self.graph.add_edge(source_idx, target_idx, weight);
    }

    pub fn average_distances(&self) -> f64 {
        let num_nodes = self.graph.node_count();
        let mut total_distance = 0.0;

        for start in self.graph.node_indices() {
            let distances = dijkstra(&self.graph, start, None, |_| 1.0 / 0.0);
            for (_, distance) in distances {
                if distance.is_finite() {
                    total_distance += distance;
                }
            }
        }

        total_distance / (num_nodes * (num_nodes - 1)) as f64
    }

    pub fn centrality_measures(&self) -> Vec<f64> {
        let mut centralities = vec![0.0; self.graph.node_count()];
        for node in self.graph.node_indices() {
            let distances = dijkstra(&self.graph, node, None, |_| 1.0);
            let mut total_distance = 0.0;
            for (_, distance) in distances {
                if distance.is_finite() {
                    total_distance += distance;
                }
            }
            centralities[node.index()] = total_distance;
        }
        centralities
    }
}
