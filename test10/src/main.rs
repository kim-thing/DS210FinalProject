mod graph;

use graph::DiseaseSymptomNetwork;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let filename = "/Users/kw/Downloads/kidney_disease.csv";
    let network = DiseaseSymptomNetwork::from_csv(filename)?;
    
    let avg_distances = network.average_distances();
    println!("Average distances: {:?}", avg_distances);
    
    Ok(())
}
