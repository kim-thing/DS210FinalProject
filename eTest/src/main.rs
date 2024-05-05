extern crate petgraph;
extern crate csv;

use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use petgraph::visit::Dfs;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "/Users/kw/Downloads/prevDis.csv";

    let mut rdr = csv::Reader::from_path(file_path)?;
    let mut disease_to_node: HashMap<String, NodeIndex> = HashMap::new();
    let mut county_to_node: HashMap<String, NodeIndex> = HashMap::new();
    let mut graph = DiGraph::<String, u32>::new();

    for result in rdr.records() {
        let record = result?;
        let disease = record.get(0).ok_or("Missing disease")?.to_string();
        let county = record.get(1).ok_or("Missing county")?.to_string();

        let disease_node = *disease_to_node.entry(disease.clone()).or_insert_with(|| {
            let index = graph.add_node(disease.clone());
            index
        });

        let county_node = *county_to_node.entry(county.clone()).or_insert_with(|| {
            let index = graph.add_node(county.clone());
            index
        });

        graph.add_edge(disease_node, county_node, 0);
    }

    let mut dot_file = File::create("graph.dot")?;
    write!(dot_file, "digraph {{\n")?;
    for edge in graph.raw_edges() {
        let source = graph[edge.source()].clone();
        let target = graph[edge.target()].clone();
        write!(dot_file, "    \"{}\" -> \"{}\";\n", source, target)?;
    }
    write!(dot_file, "}}\n")?;

    let mut sum_distances = 0;
    let mut num_distances = 0;

    for disease_node in graph.node_indices() {
        for county_node in graph.node_indices() {
            if disease_node != county_node {
                let distance = shortest_path_length(&graph, disease_node, county_node);
                if let Some(dist) = distance {
                    sum_distances += dist;
                    num_distances += 1;
                }
            }
        }
    }

    let average_distance = sum_distances as f64 / num_distances as f64;
    println!("Average distance: {}", average_distance);

    Ok(())
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
