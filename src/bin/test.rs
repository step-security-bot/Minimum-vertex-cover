use petgraph::graph::NodeIndex;
use petgraph::graphmap::GraphMap;
use petgraph::matrix_graph::MatrixGraph;
use petgraph::Undirected;
use vertex::graph_utils::load_clq_file;

fn main() {
    graph_map();

    let graph = load_clq_file("src/resources/graphs/test2.clq").unwrap();
    println!("Does graph has edge 5 -6 ? {}", graph.contains_edge(5, 6));
    for vertex in graph.nodes() {
        println!("vertex = {:?}", vertex);
    }
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

    graph.remove_node(NodeIndex::new(4));

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