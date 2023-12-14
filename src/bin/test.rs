use petgraph::prelude::UnGraphMap;
use vertex::graph_utils::load_clq_file;

fn main() {
    let mut graph: UnGraphMap<u64, ()> = UnGraphMap::new();
    graph.add_edge(1, 2, ());
    graph.add_edge(1, 3, ());
    graph.add_edge(1, 4, ());

    assert!(graph.contains_edge(1, 2) && graph.contains_edge(2, 1));
    for neighbor in graph.neighbors(1) {
        println!("{}", neighbor);
    }
}