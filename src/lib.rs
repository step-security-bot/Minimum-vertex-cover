extern crate graph;

use petgraph::matrix_graph::MatrixGraph;
use petgraph::Undirected;

use crate::graph_utils::is_vertex_cover;

pub mod graph_utils;
pub mod format;
pub mod branch_and_bound;

/// Naïve algorithm that searches for the minimum vertex cover of a given graph.
///
/// The algorithm list all possible subsets of the vertices of the graph and check if each
/// subset is a vertex cover going from the smallest subset to the largest one.
///
/// # Example
/// ```rust
/// use petgraph::matrix_graph::MatrixGraph;
/// use petgraph::Undirected;
/// use petgraph::stable_graph::NodeIndex;
/// use vertex::naive_search;
///
/// let mut graph = MatrixGraph::<u64, (), Undirected>::new_undirected();
/// for i in 0..4 {
///    graph.add_node(i);
/// }
/// graph.add_edge(NodeIndex::new(0), NodeIndex::new(1), ());
/// graph.add_edge(NodeIndex::new(1), NodeIndex::new(2), ());
/// graph.add_edge(NodeIndex::new(2), NodeIndex::new(0), ());
/// graph.add_edge(NodeIndex::new(2), NodeIndex::new(3), ());
///
/// let expected_vertex_cover = vec![0, 2];
/// assert_eq!(naive_search(&graph), Some(expected_vertex_cover));
/// ```
pub fn naive_search(graph: &MatrixGraph<u64, (), Undirected>) -> Option<Vec<u64>> {
    let possible_values: Vec<u64> = (0..graph.node_count() as u64).collect();
    let subsets : Vec<Vec<u64>> = get_subsets(&possible_values);

    for subset in subsets {
        if is_vertex_cover(graph, &subset) {
            return Some(subset);
        }
    }
    None
}


/// Generate all subsets of a given set. This subset is ordered by size.
fn get_subsets<T>(s: &[T]) -> Vec<Vec<T>> where T: Clone {
    let mut tmp: Vec<Vec<T>> = (0..2usize.pow(s.len() as u32)).map(|i| {
        s.iter().enumerate().filter(|&(t, _)| (i >> t) % 2 == 1)
            .map(|(_, element)| element.clone())
            .collect()
    }).collect();
    tmp.sort_by(|a, b| a.len().cmp(&b.len()));
    tmp
}

#[cfg(test)]
mod  algorithms_tests {
    use petgraph::stable_graph::NodeIndex;

    use super::*;

    #[test]
    fn test_naive_algorithm() {
        let mut graph = MatrixGraph::<u64, (), Undirected>::new_undirected();
        for i in 0..4 {
            graph.add_node(i);
        }
        graph.add_edge(NodeIndex::new(0), NodeIndex::new(1), ());
        graph.add_edge(NodeIndex::new(1), NodeIndex::new(2), ());
        graph.add_edge(NodeIndex::new(2), NodeIndex::new(0), ());
        graph.add_edge(NodeIndex::new(2), NodeIndex::new(3), ());

        let expected_vertex_cover = vec![0, 2];
        assert_eq!(naive_search(&graph), Some(expected_vertex_cover));
    }

    #[test]
    fn test_get_subset() {
        let initial_set = vec![1, 2, 3];
        let expected_subset = vec![
            vec![],
            vec![1],
            vec![2],
            vec![3],
            vec![1, 2],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3]
        ];
        assert_eq!(get_subsets(&initial_set), expected_subset);
    }
}