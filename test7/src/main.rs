mod graph;
mod utils;

use graph::Graph;
use utils::csv_reader::read_csv;

fn main() {
    // Read CSV file into a graph
    let graph = read_csv("/Users/kw/Downloads/kidney_disease.csv").unwrap();

    // Calculate average distances
    let avg_distances = graph.average_distances();
    println!("Average distances: {:?}", avg_distances);

    // Calculate centrality measures
    let centrality_measures = graph.centrality_measures();
    println!("Centrality measures: {:?}", centrality_measures);
}
