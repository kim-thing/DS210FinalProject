use csv::ReaderBuilder;
use std::error::Error;
use std::path::Path;
use crate::graph::Graph;

pub fn read_csv(file_path: &str) -> Result<Graph, Box<dyn Error>> {
    let mut graph = Graph::new();

    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path(Path::new(file_path))?;

    for result in rdr.records() {
        let record = result?;
        let row: Vec<f64> = record
            .iter()
            .map(|field| field.parse::<f64>().unwrap_or(0.0))
            .collect();
        for (i, &value) in row.iter().enumerate() {
            if value != 0.0 {
                graph.add_edge(0, i, value);
            }
        }
    }

    Ok(graph)
}
