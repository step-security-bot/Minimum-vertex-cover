extern crate graph;

use std::fmt::Display;
use std::time::Duration;
use petgraph::prelude::UnGraphMap;

use crate::graph_utils::{get_optimal_value, is_optimal_value, is_vertex_cover};

pub mod graph_utils;
pub mod format;
pub mod branch_and_bound;


pub struct ElapseTime {
    pub duration: Duration,
    pub min: u128,
    pub sec: u128,
    pub ms: u128,
    pub micro: u128,
}

impl ElapseTime {
    pub fn new(duration: Duration) -> ElapseTime {
        let elapsed = duration.as_micros();
        let min = elapsed / 60_000_000;
        let sec = (elapsed / 1_000_000) % 60;
        let ms = (elapsed / 1_000) % 1_000;
        let micro = elapsed % 1_000;
        ElapseTime {
            duration,
            min,
            sec,
            ms,
            micro,
        }
    }
}

impl Display for ElapseTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}min {}s {}ms {}µs", self.min, self.sec, self.ms, self.micro)
    }
}

/// Naïve algorithm that searches for the minimum vertex cover of a given graph.
///
/// The algorithm list all possible subsets of the vertices of the graph and check if each
/// subset is a vertex cover going from the smallest subset to the largest one.
///
/// # Example
/// ```rust
/// use petgraph::prelude::UnGraphMap;
/// use vertex::naive_search;
///
/// let mut graph = UnGraphMap::<u64, ()>::new();
/// for i in 0..4 {
///    graph.add_node(i);
/// }
/// graph.add_edge(0, 1, ());
/// graph.add_edge(1, 2, ());
/// graph.add_edge(2, 0, ());
/// graph.add_edge(2, 3, ());
///
/// let expected_vertex_cover = 2; //[0, 2] or [1, 2]
/// assert_eq!(naive_search(&graph), expected_vertex_cover);
/// ```
pub fn naive_search(graph: &UnGraphMap<u64, ()>) -> u64 {
    let possible_values: Vec<u64> = (0..graph.node_count() as u64).collect();
    let subsets : Vec<Vec<u64>> = get_subsets(&possible_values);

    for subset in subsets {
        if is_vertex_cover(graph, &subset) {
            return subset.len() as u64;
        }
    }
    0
}

/// Run a given algorithm on a given graph and print the result. It is the default function when you want
/// to test your algorithm on a certain graph. It prints the result and tell you if it is optimal or not based
/// on the data in the yaml file.
/// The algorithm must take an UnGraphMap as input and return a u64.
///
/// # Example
/// ```rust
/// use petgraph::prelude::UnGraphMap;use vertex::graph_utils::load_clq_file;
/// use vertex::{naive_search, run_algorithm};
///
/// let mut graph = load_clq_file("src/resources/graphs/test.clq").unwrap();
/// run_algorithm("test.clq", &graph, &naive_search);
/// ```
pub fn run_algorithm(graph_id: &str,
                     graph: &UnGraphMap<u64, ()>,
                     f: &dyn Fn(&UnGraphMap<u64, ()>) -> u64) -> ElapseTime{
    use std::time::Instant;
    let now = Instant::now();

    let res = f(graph);

    let elapsed = ElapseTime::new(now.elapsed());
    println!("{}", elapsed);
    println!("Minimum vertex cover for the {:?} graph = {}", graph_id, res);
    let is_opt = is_optimal_value(graph_id, res, None);
    if is_opt {
        println!("The value is optimal (as long as the data is correct in the yaml file)");
    } else {
        let true_opt = get_optimal_value(graph_id, None).unwrap_or(0);
        if true_opt == 0 {
            println!("The correct value is unknown");
        } else {
            println!("The value is not optimal and the correct value is {}", true_opt);
        }
    }
    return elapsed;
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
    use super::*;

    #[test]
    fn test_naive_algorithm() {
        let mut graph = UnGraphMap::<u64, ()>::new();
        for i in 0..4 {
            graph.add_node(i);
        }
        graph.add_edge(0, 1, ());
        graph.add_edge(1, 2, ());
        graph.add_edge(2, 0, ());
        graph.add_edge(2, 3, ());

        let expected_vertex_cover = 2;
        assert_eq!(naive_search(&graph), expected_vertex_cover);
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