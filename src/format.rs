use graph::{Graph, GraphConstructible, GraphNauty};
use petgraph::graphmap::UnGraphMap;

/// Takes a graph in the GraphNauty format and returns a graph in the petgraph format. (with adjacency matrix)
///
/// # Example
///
/// ```rust
/// use vertex::format::graph_nauty_to_petgraph;
/// use graph::{Graph, GraphConstructible, GraphNauty};
///
/// let mut graph_nauty = GraphNauty::new(4);
/// graph_nauty.add_edge(0, 1);
/// graph_nauty.add_edge(1, 2);
/// graph_nauty.add_edge(2, 0);
/// graph_nauty.add_edge(2, 3);
/// let petgraph = graph_nauty_to_petgraph(&graph_nauty);
/// assert_eq!(petgraph.node_count(), 4);
/// ```
pub fn graph_nauty_to_petgraph(graph: &GraphNauty) -> UnGraphMap<u64, ()> {
    let mut petgraph = UnGraphMap::<u64, ()>::new();
    for i in 0..graph.order() {
        petgraph.add_node(i);
    }
    for i in 0..graph.order() {
        for j in 0..graph.order() {
            if graph.is_edge(i, j) && !petgraph.contains_edge(i, j){
                petgraph.add_edge(i, j, ());
            }
        }
    }
    petgraph
}

/// Takes a graph in the petgraph format (with adjacency matrix) and returns a graph in the GraphNauty format.
///
/// # Example
///
/// ```rust
/// use graph::Graph;
/// use vertex::format::petgraph_to_graph_nauty;
/// use petgraph::prelude::UnGraphMap;
///
/// let mut petgraph = UnGraphMap::<u64, ()>::new();
/// for i in 0..4 {
///    petgraph.add_node(i);
/// }
/// petgraph.add_edge(0, 1, ());
/// petgraph.add_edge(1, 2, ());
/// petgraph.add_edge(2, 0, ());
/// petgraph.add_edge(2, 3, ());
/// let graph_nauty = petgraph_to_graph_nauty(&petgraph);
/// assert_eq!(graph_nauty.order(), 4);
/// ```
pub fn petgraph_to_graph_nauty(graph: &UnGraphMap<u64, ()>) -> GraphNauty {
    let mut graph_nauty = GraphNauty::new(graph.node_count() as u64);
    for node in 0..graph.node_count() {
        for neighbor in graph.neighbors(node as u64) {
            graph_nauty.add_edge(node as u64, neighbor);
        }
    }
    graph_nauty
}


#[cfg(test)]
mod format_test {
    use super::*;

    #[test]
    fn test_graph_nauty_to_petgraph() {
        let mut graph_nauty = GraphNauty::new(4);
        graph_nauty.add_edge(0, 1);
        graph_nauty.add_edge(1, 2);
        graph_nauty.add_edge(2, 0);
        graph_nauty.add_edge(2, 3);
        let petgraph = graph_nauty_to_petgraph(&graph_nauty);
        assert_eq!(petgraph.node_count(), 4);
        assert_eq!(petgraph.edge_count(), 4);
        assert!(petgraph.contains_edge(0, 1));
        assert!(petgraph.contains_edge(1, 2));
        assert!(petgraph.contains_edge(2, 0));
        assert!(petgraph.contains_edge(2, 3));
    }

    #[test]
    fn test_petgraph_to_graph_nauty() {
        let mut petgraph = UnGraphMap::<u64, ()>::new();
        for i in 0..4 {
            petgraph.add_node(i);
        }
        petgraph.add_edge(0, 1, ());
        petgraph.add_edge(1, 2, ());
        petgraph.add_edge(2, 0, ());
        petgraph.add_edge(2, 3, ());

        let graph_nauty = petgraph_to_graph_nauty(&petgraph);
        assert_eq!(graph_nauty.order(), 4);
        assert_eq!(graph_nauty.size(), 4);
        assert!(graph_nauty.is_edge(0, 1));
        assert!(graph_nauty.is_edge(1, 2));
        assert!(graph_nauty.is_edge(2, 0));
        assert!(graph_nauty.is_edge(2, 3));
    }
}