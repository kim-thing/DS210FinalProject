#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::{load_graph, average_distances};

    #[test]
    fn test_load_graph() {
        let graph = load_graph("/Users/kw/Downloads/kidney_disease.csv").unwrap();
        assert_eq!(graph.node_count(), 8);
    }

    #[test]
    fn test_average_distances() {
        let graph = load_graph("/Users/kw/Downloads/kidney_disease.csv").unwrap();
        let avg_distances = average_distances(&graph);
        // Add your test assertions here
    }
}
