#[cfg(test)]
mod tests {
    use super::*;
    use crate::{graph::load_graph, centrality::calculate_centrality};

    #[test]
    fn test_centrality_calculation() {
        let graph = load_graph("/Users/kw/Downloads/kidney_disease.csv").unwrap();
        let centrality_measures = calculate_centrality(&graph);
        // Add your test assertions here
    }
}
