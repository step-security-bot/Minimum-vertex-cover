use std::fmt::Display;
use std::ops::Add;
use std::time::Duration;

use petgraph::prelude::UnGraphMap;
use serde::{Deserialize, Serialize};

use crate::branch_and_bound::b_and_b;
use crate::graph_utils::{copy_graph, get_optimal_value, is_optimal_value, is_vertex_cover};

pub mod graph_utils;
pub mod format;
mod branch_and_bound;

pub mod mvcgraph;

/// Naïve algorithm that searches for the minimum vertex cover of a given graph.
///
/// The algorithm list all possible subsets of the vertices of the graph and check if each
/// subset is a vertex cover going from the smallest subset to the largest one.
///
/// This algorithm can be used on any graph with order < 65.
///
/// # Example
/// ```rust
/// use petgraph::prelude::UnGraphMap;
/// use vertex::{Clock, naive_search};
///
/// let mut graph = Box::new(UnGraphMap::<u64, ()>::new());
/// for i in 0..4 {
///    graph.add_node(i);
/// }
/// graph.add_edge(0, 1, ());
/// graph.add_edge(1, 2, ());
/// graph.add_edge(2, 0, ());
/// graph.add_edge(2, 3, ());
///
/// let expected_vertex_cover = 2; //[0, 2] or [1, 2]
/// assert_eq!(naive_search(&graph, &mut Clock::new(3600)).0, expected_vertex_cover);
/// ```
pub fn naive_search(graph: &UnGraphMap<u64, ()>, clock: &mut Clock) -> (u64, Vec<u64>) {
    if graph.node_count() > 64 {
        panic!("This algorithm can only be used on graph with less than 65 vertices")
    }
    let possible_values: Vec<u64> = (0..graph.node_count() as u64).collect();
    let mut found = false;
    let mut res = 0;
    let mut res_subset: Vec<u64> = Vec::new();
    for subset in get_subsets(&possible_values) {
        if clock.is_time_up() {
            break;
        }
        if !found || res > subset.len() as u64 {
            if is_vertex_cover(graph, &subset) {
                res = subset.len() as u64;
                res_subset = subset;
                found = true;
            }
        }
    }
    (res, res_subset)
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
/// let res = run_algorithm("test.clq", &graph, &naive_search, false);
/// println!("{}", res);
/// ```
pub fn run_algorithm(graph_id: &str,
                     graph: &UnGraphMap<u64, ()>,
                     f: &dyn Fn(&UnGraphMap<u64, ()>, &mut Clock) -> (u64, Vec<u64>),
                     cmpl: bool) -> MVCResult {
    let g: UnGraphMap<u64, ()>;
    if cmpl {
        g = graph_utils::complement(graph);
        let density = (2 * g.edge_count()) as f64 / (g.node_count() * (g.node_count() - 1)) as f64;
        println!("Running algorithm the complement of the graph. Order = {} and size = {}. Density = {}",
                 g.node_count(),
                 g.edge_count(),
                 density);
    } else {
        g = copy_graph(graph);
        let density = (2 * g.edge_count()) as f64 / (g.node_count() * (g.node_count() - 1)) as f64;
        println!("Running algorithm on the graph. Order = {} and size = {}, density = {}",
                 graph.node_count(),
                 graph.edge_count(),
                 density);
    }

    let limit = 900;

    let mut clock: Clock = Clock::new(limit);

    let res = f(&g, &mut clock);

    let elapsed = clock.get_time();

    assert!(is_vertex_cover(&g, &res.1));
    assert_eq!(res.0, res.1.len() as u64);

    let res = MVCResult::new(graph_id.to_string(), res.0, res.1, elapsed, clock.is_time_up(), cmpl);
    return res;
}

/// Branch and bound algorithm that searches for the minimum vertex cover of a given graph.
///
/// * Branch : The algorithm branches on the vertex with max degree.
/// It creates 2 branches : one where the vertex is in the vertex cover and one where its neighbors are in the vertex cover.
/// * Bound : The algorithm has 2 lower bounds : clqLB and degLB. (see the paper linked in README for more details)
///
/// The clock is used to stop the algorithm if it reaches the time limit defined in the clock.
/// It is also used to measure the time taken by the algorithm (and some of its subroutines).
///
/// # Example
/// ```rust
/// use petgraph::prelude::UnGraphMap;
/// use vertex::{Clock, branch_and_bound};
/// use vertex::graph_utils::load_clq_file;
///
/// let graph = load_clq_file("src/resources/graphs/test.clq")
///             .expect("Error while loading graph");
/// let mut clock = Clock::new(3600); // 1 hour time limit
///
/// let res = branch_and_bound(&graph, &mut clock);
///
/// assert_eq!(res.0, 3);
/// assert_eq!(res.1, vec![0, 4, 2]);
/// ```
///
pub fn branch_and_bound(graph: &UnGraphMap<u64, ()>, clock: &mut Clock) -> (u64, Vec<u64>) {
    // Initialize the upper bound to the number of nodes in the graph
    // and the vertex cover found so far is empty
    let upper_bound_vc = &graph.nodes().collect();
    let u = b_and_b(graph, graph, graph.node_count() as u64,
                    upper_bound_vc, vec![], clock);

    assert!(is_vertex_cover(graph, &u.1));
    u
}

/// Iterator over all the subsets of a given set
/// Example :
/// * set = `[0, 1, 2]`
/// * subsets = `[[], [0], [1], [0, 1], [2], [0, 2], [1, 2], [0, 1, 2]]`
///
pub struct SubsetIterator<T> where T: Clone {
    pub set: Vec<T>,
    pub n: usize,
    pub n_times: usize,
    pub i: u64,
}

impl<T> Iterator for SubsetIterator<T> where T: Clone {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= (1 << self.n) as u64 {
            return None;
        }
        let mut subset: Vec<T> = Vec::new();
        for j in 0..self.n {
            if (self.i & (1 << j)) != 0 {
                subset.push(self.set[j].clone());
            }
        }
        self.i += 1;
        Some(subset)
    }
}

/// Returns an iterator over all the subsets of a given set. (The size of the set is capped at 63)
fn get_subsets<T>(s: &[T]) -> SubsetIterator<T> where T: Clone {
    let n = s.len();

    SubsetIterator {
        set: s.to_vec(),
        n,
        n_times: 1 << n,
        i: 1,
    }
}

/// Struct representing the time taken by an algorithm (in minutes, seconds, milliseconds and microseconds)
///
/// # Example
/// ```rust
/// use std::time::Duration;
/// use vertex::ElapseTime;
///
/// let duration = Duration::new(190, 1001000); // 3 mins 10 second 1 ms 1 microsecond
///
/// let elapsed = ElapseTime::new(duration);
///
/// assert_eq!(elapsed.min, 3);
/// assert_eq!(elapsed.sec, 10);
/// assert_eq!(elapsed.ms, 1);
/// assert_eq!(elapsed.micro, 1);
/// ```
#[derive(Debug, PartialEq, Deserialize, Serialize)]
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

/// Struct representing the result of an algorithm
pub struct MVCResult {
    /// The id of the graph. Example : "test.clq"
    pub graph_id: String,
    /// The value of the minimum vertex cover calculated by the algorithm
    pub value: u64,
    /// The set of vertices that form the minimum vertex cover
    pub set: Vec<u64>,
    /// Whether the value is optimal or not. (Found in the clique and graph data yaml files)
    pub is_optimal: Option<bool>,
    /// The time taken by the algorithm
    pub time: ElapseTime,
    /// Whether the algorithm was stopped because it reached the time limit
    pub is_time_limit: bool,
    /// Whether the algorithm was run on the complement of the graph
    pub is_compl: bool,
}

impl MVCResult {
    pub fn new(graph_id: String, value: u64, mvc: Vec<u64>, time: ElapseTime, is_time_limit: bool, is_compl: bool) -> MVCResult {
        let is_optimal = if is_compl {
            is_optimal_value(&graph_id, value, Some("src/resources/clique_data.yml"))
        } else {
            is_optimal_value(&graph_id, value, None)
        };
        MVCResult {
            graph_id,
            value,
            set: mvc,
            is_optimal,
            time,
            is_time_limit,
            is_compl,
        }
    }
}

impl Display for MVCResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let opt_message = {
            if !self.is_optimal.is_none() {
                if self.is_optimal.unwrap() {
                    "\t The value is optimal (as long as the data is correct in the yaml file)".to_string()
                } else {
                    let true_opt = if self.is_compl {
                        get_optimal_value(&self.graph_id, Some("src/resources/clique_data.yml")).unwrap_or(0)
                    } else {
                        get_optimal_value(&self.graph_id, None).unwrap_or(0)
                    };
                    format!("\t The value is not optimal and the correct value is {}", true_opt).to_string()
                }
            } else {
                "\t The graph is not in the yaml file".to_string()
            }
        };

        let time_limit_message = {
            if self.is_time_limit {
                "\n\t The algorithm was stopped because it reached the time limit".to_string()
            } else {
                "".to_string()
            }
        };

        write!(f, "Minimum vertex cover for the {:?} graph = {}\n{}\n\t Time taken by the algorithm : {} {}",
               self.graph_id,
               self.value,
               opt_message,
               self.time,
               time_limit_message)
    }
}

/// Struct representing a clock used to measure the time taken by an algorithm and stop it if it reaches the time limit.
///
/// This clock is based on the std::time::Instant struct.
/// The clock can also be used to measure the time taken by some subroutines of the algorithm.
///
/// # Example
/// ```rust
/// use std::time::Duration;
/// use vertex::Clock;
///
/// let mut clock = Clock::new(3600); // 1 hour time limit
///
/// let elapsed = clock.get_time();
///
/// if clock.is_time_up() {
///    println!("Time is up !");
/// }
///```
pub struct Clock {
    pub start: std::time::Instant,
    limit: u64,
    elapsed: Option<Duration>,
    // in seconds
    pub start_deg: std::time::Instant,
    pub deg_lb: Duration,
    pub start_clq: std::time::Instant,
    pub clq_lb: Duration,
    pub start_max_deg: std::time::Instant,
    pub max_deg: Duration,
    pub enter_copy: std::time::Instant,
    pub copy: Duration,
    pub enter_clq_compl: std::time::Instant,
    pub clq_compl: Duration,
    pub enter_color_set: std::time::Instant,
    pub color_set: Duration,
}

impl Clock {
    pub fn new(limit: u64) -> Clock {
        Clock {
            start: std::time::Instant::now(),
            limit,
            elapsed: None,
            start_deg: std::time::Instant::now(),
            deg_lb: Duration::new(0, 0),
            start_clq: std::time::Instant::now(),
            clq_lb: Duration::new(0, 0),
            start_max_deg: std::time::Instant::now(),
            max_deg: Duration::new(0, 0),
            enter_copy: std::time::Instant::now(),
            copy: Duration::new(0, 0),
            enter_clq_compl: std::time::Instant::now(),
            clq_compl: Duration::new(0, 0),
            enter_color_set: std::time::Instant::now(),
            color_set: Duration::new(0, 0),
        }
    }

    /// Returns the time off the clock since it was created.
    pub fn get_time(&self) -> ElapseTime {
        if self.elapsed.is_none() {
            let elapsed = self.start.elapsed();
            ElapseTime::new(elapsed)
        } else {
            ElapseTime::new(self.elapsed.unwrap())
        }
    }

    /// Stops the clock.
    pub fn stop_timer(&mut self) {
        if self.elapsed.is_none() {
            self.elapsed = Some(self.start.elapsed());
        }
    }

    /// Returns true if the time limit is reached.
    pub fn is_time_up(&self) -> bool {
        let elapsed = self.start.elapsed();
        elapsed.as_secs() >= self.limit
    }

    pub fn enter_deg(&mut self) {
        self.start_deg = std::time::Instant::now();
    }

    pub fn exit_deg(&mut self) {
        self.deg_lb = self.deg_lb.add(self.start_deg.elapsed());
    }

    pub fn enter_clq(&mut self) {
        self.start_clq = std::time::Instant::now();
    }

    pub fn exit_clq(&mut self) {
        self.clq_lb = self.clq_lb.add(self.start_clq.elapsed());
    }

    pub fn enter_max_deg(&mut self) {
        self.start_max_deg = std::time::Instant::now();
    }

    pub fn exit_max_deg(&mut self) {
        self.max_deg = self.max_deg.add(self.start_max_deg.elapsed());
    }

    pub fn enter_copy(&mut self) {
        self.enter_copy = std::time::Instant::now();
    }

    pub fn exit_copy(&mut self) {
        self.copy = self.copy.add(self.enter_copy.elapsed());
    }

    pub fn enter_clq_compl(&mut self) {
        self.enter_clq_compl = std::time::Instant::now();
    }

    pub fn exit_clq_compl(&mut self) {
        self.clq_compl = self.clq_compl.add(self.enter_clq_compl.elapsed());
    }

    pub fn enter_color_set(&mut self) {
        self.enter_color_set = std::time::Instant::now();
    }

    pub fn exit_color_set(&mut self) {
        self.color_set = self.color_set.add(self.enter_color_set.elapsed());
    }
}
#[cfg(test)]
mod algorithms_tests {
    use super::*;

    #[test]
    fn test_naive_algorithm() {
        let mut graph = Box::new(UnGraphMap::<u64, ()>::new());
        for i in 0..4 {
            graph.add_node(i);
        }
        graph.add_edge(0, 1, ());
        graph.add_edge(1, 2, ());
        graph.add_edge(2, 0, ());
        graph.add_edge(2, 3, ());

        let expected_vertex_cover = 2;
        assert_eq!(naive_search(&graph, &mut Clock::new(3600)).0, expected_vertex_cover);
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
            vec![1, 2, 3],
        ];
        let expected_subset: Box<Vec<Vec<u64>>> = Box::new(expected_subset);
        let out = get_subsets(&initial_set);
        for val in out {
            assert!(expected_subset.contains(&val));
        }
    }
}