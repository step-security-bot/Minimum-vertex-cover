use graph::*;

/// Check if a given vertex cover is a vertex cover of a given graph.
///
/// Parse all edges of the graph and check if at least one of the vertices of each edge is in the vertex cover.
/// If not, return false.
///
/// # Example
/// ```rust
/// use graph::{Graph, GraphConstructible, GraphNauty};
/// use vertex::graph_utils::is_vertex_cover;
///
/// let mut graph_nauty = GraphNauty::new(3);
/// graph_nauty.add_edge(0, 1);
/// graph_nauty.add_edge(1, 2);
/// graph_nauty.add_edge(2, 0);
/// let mut vertex_cover: Vec<u64> = Vec::new();
/// vertex_cover.push(0);
/// assert!(!is_vertex_cover(&graph_nauty, &vertex_cover));
/// vertex_cover.push(1);
/// assert!(is_vertex_cover(&graph_nauty, &vertex_cover));
/// vertex_cover.push(2);
/// assert!(is_vertex_cover(&graph_nauty, &vertex_cover));
/// ```
pub fn is_vertex_cover(graph_nauty: &GraphNauty, vertex_cover: &Vec<u64>) -> bool {
    for edge in graph_nauty.edges() {
        if !vertex_cover.contains(&edge.0) && !vertex_cover.contains(&edge.1) {
            return false;
        }
    }
    true
}



#[cfg(test)]
mod graph_utils_tests {
    use super::*;
    use graph::{GraphNauty};

    #[test]
    fn test_is_vertex_cover() {
        let mut graph_nauty = GraphNauty::new(3);
        graph_nauty.add_edge(0, 1);
        graph_nauty.add_edge(1, 2);
        graph_nauty.add_edge(2, 0);
        let mut vertex_cover: Vec<u64> = Vec::new();
        vertex_cover.push(0);
        assert!(!is_vertex_cover(&graph_nauty, &vertex_cover));
        vertex_cover.push(1);
        assert!(is_vertex_cover(&graph_nauty, &vertex_cover));
        vertex_cover.push(2);
        assert!(is_vertex_cover(&graph_nauty, &vertex_cover));
    }
}