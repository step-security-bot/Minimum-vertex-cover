use petgraph::graph::NodeIndex;
use petgraph::graphmap::GraphMap;
use petgraph::matrix_graph::MatrixGraph;
use petgraph::Undirected;

fn main() {
    graph_map()
}


pub fn matrix() {
    let mut graph = MatrixGraph::<u64, (), Undirected>::new_undirected();
    for i in 0..4 {
        graph.add_node(i);
    }
    graph.add_edge(NodeIndex::new(2), NodeIndex::new(4), ());

    graph.remove_node(NodeIndex::new(0));

    assert!(graph.has_edge(NodeIndex::new(2), NodeIndex::new(4)));

    for edges in graph.edges(NodeIndex::new(2)) {
        println!("edges = {:?}", edges);
    }
}

pub fn graph_map() {
    let mut graph = GraphMap::<u64, (), Undirected>::new();
    for i in 0..4 {
        graph.add_node(i);
    }
    graph.add_edge(2, 4, ());

    graph.remove_node(0);

    assert!(graph.contains_edge(2, 4));

    for neigh in graph.neighbors(2) {
        println!("neigh = {:?}", neigh);
    }

    graph.remove_node(4);

    for neigh in graph.neighbors(2) {
        println!("neigh = {:?}", neigh);
    }
}