use petgraph::prelude::UnGraphMap;

use vertex::graph_utils::{complement, is_vertex_cover, load_clq_file};

fn main() {
    // We are going to create a recursive function to iterate over all possible subset of vertices in a graph.
    // For all subset, we are going to test if it is a vertex cover. If it is, we are printing it.

    let graph = load_clq_file("src/resources/graphs/C125.9.clq").unwrap();

    let graph = complement(&graph);

    naive(&graph, &mut Vec::new(), 6);
}


fn naive(graph: &UnGraphMap<u64, ()>, features: &mut Vec<u64>, max_length: u64) -> () {
    if features.len() > max_length as usize {
        return;
    }
    if features.len() == max_length as usize {
        if is_vertex_cover(&graph, &features) {
            println!("{:?}", features);
        };
    }

    for node in graph.nodes() {
        if !features.contains(&node) {
            features.push(node);
            naive(graph, features, max_length);
            features.pop();
        }
    }
}