use graph::{Graph, GraphConstructible, GraphNauty};
use petgraph::graph::NodeIndex;
use petgraph::matrix_graph::MatrixGraph;
use petgraph::Undirected;

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
pub fn graph_nauty_to_petgraph(graph: &GraphNauty) -> MatrixGraph<u64, (), Undirected> {
    let mut petgraph = MatrixGraph::<u64, (), Undirected>::new_undirected();
    for i in 0..graph.order() {
        petgraph.add_node(i);
    }
    for i in 0..graph.order() {
        for j in 0..graph.order() {
            if graph.is_edge(i, j) && !petgraph.has_edge(NodeIndex::new(i as usize), NodeIndex::new(j as usize)){
                petgraph.add_edge(NodeIndex::new(i as usize), NodeIndex::new(j as usize), ());
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
/// use petgraph::matrix_graph::MatrixGraph;
/// use petgraph::Undirected;
/// use petgraph::graph::NodeIndex;
///
/// let mut petgraph = MatrixGraph::<u64, (), Undirected>::new_undirected();
/// for i in 0..4 {
///    petgraph.add_node(i);
/// }
/// petgraph.add_edge(NodeIndex::new(0), NodeIndex::new(1), ());
/// petgraph.add_edge(NodeIndex::new(1), NodeIndex::new(2), ());
/// petgraph.add_edge(NodeIndex::new(2), NodeIndex::new(0), ());
/// petgraph.add_edge(NodeIndex::new(2), NodeIndex::new(3), ());
/// let graph_nauty = petgraph_to_graph_nauty(&petgraph);
/// assert_eq!(graph_nauty.order(), 4);
/// ```
pub fn petgraph_to_graph_nauty(graph: &MatrixGraph<u64, (), Undirected>) -> GraphNauty {
    let mut graph_nauty = GraphNauty::new(graph.node_count() as u64);
    for node in 0..graph.node_count() {
        for neighbor in graph.neighbors(NodeIndex::new(node)) {
            graph_nauty.add_edge(node as u64, neighbor.index() as u64);
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
        assert!(petgraph.has_edge(NodeIndex::new(0), NodeIndex::new(1)));
        assert!(petgraph.has_edge(NodeIndex::new(1), NodeIndex::new(2)));
        assert!(petgraph.has_edge(NodeIndex::new(2), NodeIndex::new(0)));
        assert!(petgraph.has_edge(NodeIndex::new(2), NodeIndex::new(3)));
    }

    #[test]
    fn test_petgraph_to_graph_nauty() {
        let mut petgraph = MatrixGraph::<u64, (), Undirected>::new_undirected();
        for i in 0..4 {
            petgraph.add_node(i);
        }
        petgraph.add_edge(NodeIndex::new(0), NodeIndex::new(1), ());
        petgraph.add_edge(NodeIndex::new(1), NodeIndex::new(2), ());
        petgraph.add_edge(NodeIndex::new(2), NodeIndex::new(0), ());
        petgraph.add_edge(NodeIndex::new(2), NodeIndex::new(3), ());
        let graph_nauty = petgraph_to_graph_nauty(&petgraph);
        assert_eq!(graph_nauty.order(), 4);
        assert_eq!(graph_nauty.size(), 4);
        assert!(graph_nauty.is_edge(0, 1));
        assert!(graph_nauty.is_edge(1, 2));
        assert!(graph_nauty.is_edge(2, 0));
        assert!(graph_nauty.is_edge(2, 3));
    }
}