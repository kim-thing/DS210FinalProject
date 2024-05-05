extern crate csv;
extern crate petgraph;

use csv::ReaderBuilder;
use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Symptom(String);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Disease(String);

#[derive(Debug)]
struct DiseaseSymptomGraph {
    graph: DiGraph<Disease, ()>,
    symptom_to_diseases: HashMap<Symptom, HashSet<Disease>>,
}

impl DiseaseSymptomGraph {
    fn new() -> Self {
        DiseaseSymptomGraph {
            graph: DiGraph::new(),
            symptom_to_diseases: HashMap::new(),
        }
    }

    fn add_disease(&mut self, disease: Disease, symptoms: Vec<Symptom>) -> NodeIndex {
        let idx = self.graph.add_node(disease.clone());

        for symptom in symptoms {
            if let Some(existing_diseases) = self.symptom_to_diseases.get_mut(&symptom) {
                for &disease in existing_diseases.iter() {
                    self.graph.add_edge(disease, idx, ());
                }
            }
            self.symptom_to_diseases
                .entry(symptom.clone())
                .or_insert_with(HashSet::new)
                .insert(disease.clone());
        }

        idx
    }

    fn find_diseases(&self, symptom: &Symptom) -> Option<&HashSet<Disease>> {
        self.symptom_to_diseases.get(symptom)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "/Users/kw/Downloads/kidney_disease.csv";

    let mut graph = DiseaseSymptomGraph::new();

    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    let headers = rdr.headers()?.clone();

    for result in rdr.records() {
        let record = result?;
        let disease_name = record.get(24).unwrap_or(&"").to_string();
        if !disease_name.is_empty() {
            let symptoms = record
                .iter()
                .enumerate()
                .filter_map(|(idx, val)| {
                    if idx != 24 && !val.is_empty() {
                        let symptom_name = headers.get(idx).unwrap().to_string();
                        Some(Symptom(symptom_name))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            let disease = Disease(disease_name);
            graph.add_disease(disease, symptoms);
        }
    }

    // Example usage: Finding diseases associated with a symptom
    let symptom_to_check = Symptom("bp".to_string());
    if let Some(diseases) = graph.find_diseases(&symptom_to_check) {
        let disease_names: Vec<_> = diseases.iter().map(|d| &d.0).collect();
        println!(
            "Diseases associated with symptom {:?}: {:?}",
            symptom_to_check, disease_names
        );
    } else {
        println!(
            "Symptom {:?} is not associated with any disease",
            symptom_to_check
        );
    }

    Ok(())
}
