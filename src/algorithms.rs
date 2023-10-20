use graph::*;
use crate::graph_utils::is_vertex_cover;


/// Naïve algorithm that searches for the minimum vertex cover of a given graph.
///
/// The algorithm list all possible subsets of the vertices of the graph and check if each
/// subset is a vertex cover going from the smallest subset to the largest one.
///
/// # Example
/// ```rust
/// use graph::{Graph, GraphConstructible, GraphNauty};
/// use vertex::algorithms::naive_search;
///
/// let mut graph_nauty = GraphNauty::new(4);
/// graph_nauty.add_edge(0, 1);
/// graph_nauty.add_edge(1, 2);
/// graph_nauty.add_edge(2, 0);
/// graph_nauty.add_edge(2, 3);
///
/// let expected_vertex_cover = vec![0, 2];
/// assert_eq!(naive_search(&graph_nauty), Some(expected_vertex_cover));
/// ```
pub fn naive_search(graph: &GraphNauty) -> Option<Vec<u64>> {
    let possible_values: Vec<u64> = (0..graph.order()).collect();
    let subsets = powerset(&possible_values);
    println!("{:?}", subsets);

    for subset in subsets {
        if is_vertex_cover(graph, &subset) {
            println!("Found a vertex cover: {:?}", subset);
            return Some(subset);
        }
    }
    None
}


/// Generate all subsets of a given set.
fn powerset<T>(s: &[T]) -> Vec<Vec<T>> where T: Clone {
    (0..2usize.pow(s.len() as u32)).map(|i| {
        s.iter().enumerate().filter(|&(t, _)| (i >> t) % 2 == 1)
            .map(|(_, element)| element.clone())
            .collect()
    }).collect()
}

#[cfg(test)]
mod  algorithms_tests {
    use super::*;
    use graph::{GraphNauty};

    #[test]
    fn test_naive_algorithm() {
        let mut graph_nauty = GraphNauty::new(4);
        graph_nauty.add_edge(0, 1);
        graph_nauty.add_edge(1, 2);
        graph_nauty.add_edge(2, 0);
        graph_nauty.add_edge(2, 3);

        let expected_vertex_cover = vec![0, 2];
        assert_eq!(naive_search(&graph_nauty), Some(expected_vertex_cover));
    }

    #[test]
    fn test_powerset() {
        let initial_set = vec![1, 2, 3];
        let expected_powerset = vec![
            vec![],
            vec![1],
            vec![2],
            vec![1, 2],
            vec![3],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3]
        ];
        assert_eq!(powerset(&initial_set), expected_powerset);
    }
}