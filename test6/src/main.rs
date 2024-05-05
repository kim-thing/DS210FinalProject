mod graph;
mod centrality;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Load the disease-symptom network
    let graph = graph::load_graph("/Users/kw/Downloads/kidney_disease.csv")?;

    // Calculate average distances
    let avg_distances = graph::average_distances(&graph);
    println!("Average Distances: {:?}", avg_distances);

    // Calculate select centrality measures
    let centrality_measures = centrality::calculate_centrality(&graph);
    println!("Centrality Measures: {:?}", centrality_measures);

    Ok(())
}
