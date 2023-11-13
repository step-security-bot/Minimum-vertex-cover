use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

use petgraph::prelude::UnGraphMap;
use serde::{Deserialize, Serialize};

/// Check if a given vertex cover is a vertex cover of a given graph.
///
/// Parse all edges of the graph and check if at least one of the vertices of each edge is in the vertex cover.
/// If not, return false.
///
/// # Example
/// ```rust
/// use petgraph::matrix_graph::MatrixGraph;
/// use petgraph::prelude::UnGraphMap;
/// use petgraph::Undirected;
/// use petgraph::stable_graph::NodeIndex;
/// use vertex::graph_utils::is_vertex_cover;
///
/// let mut graph = UnGraphMap::<u64, ()>::new();
/// for i in 0..3 {
///    graph.add_node(i);
/// }
/// graph.add_edge(0, 1, ());
/// graph.add_edge(1, 2, ());
/// graph.add_edge(2, 0, ());
/// let mut vertex_cover: Vec<u64> = Vec::new();
/// vertex_cover.push(0);
/// assert!(!is_vertex_cover(&graph, &vertex_cover));
/// vertex_cover.push(1);
/// assert!(is_vertex_cover(&graph, &vertex_cover));
/// ```
pub fn is_vertex_cover(graph: &UnGraphMap<u64, ()>, vertex_cover: &Vec<u64>) -> bool {
    for (i, j) in edges(graph) {
        if !vertex_cover.contains(&(i)) && !vertex_cover.contains(&(j)) {
            return false;
        }
    }
    true
}

/// Load a graph from a DIMACS .col file.
///
/// The format of the file is the following:
/// * p <#vertex> <#edges> : the number of vertices and edges of the graph
/// * e <vertex1> <vertex2> : an edge between vertex1 and vertex2
/// * c <comment> : a comment
///
/// # Test file
/// ```text
/// c File: test.clq
/// p edge 5 6
/// e 1 2
/// e 1 3
/// e 1 4
/// e 3 4
/// e 5 1
/// e 5 2
/// ```
///
/// # Example
/// ```rust
/// use petgraph::stable_graph::NodeIndex;
/// use vertex::graph_utils::load_clq_file;
///
/// let graph = load_clq_file("src/resources/graphs/test.clq").unwrap();
/// assert_eq!(graph.node_count(), 5);
/// assert!(graph.contains_edge(0, 1));
/// assert!(graph.contains_edge(0, 2));
/// assert!(graph.contains_edge(0, 3));
/// assert!(graph.contains_edge(2, 3));
/// assert!(graph.contains_edge(4, 0));
/// assert!(graph.contains_edge(4, 1));
/// ```
pub fn load_clq_file(path: &str) -> Result<UnGraphMap<u64, ()>, Box<dyn Error>> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => return Err(format!("File {:?} not found \n {:?}", path, e).into()),
    };
    let reader = BufReader::new(file);

    let mut g = UnGraphMap::<u64, ()>::new();
    let mut exp_edges = 0;
    let mut edges = 0;

    for line in reader.lines() {
        let line = line?;
        let values: Vec<&str> = line.split_whitespace().collect();

        match values[0] {
            "c" => {
                continue;
            }
            "p" => {
                if values[1] != "edge" && values[1] != "col" {
                    // Idk why col but ok
                    return Err("Expecting edge/col format".into());
                }
                let order = values[2].parse::<u64>()?;
                exp_edges = values[3].parse::<u64>()?;
                for i in 0..order {
                    g.add_node(i);
                }
            }
            "e" => {
                if g.node_count() == 0 {
                    return Err("Expecting graph order".into());
                }
                let i = values[1].parse::<u64>()? - 1;
                let j = values[2].parse::<u64>()? - 1;

                g.add_edge(i,  j, ());
                edges += 1;
            }
            _ => {
                return Err(format!("Invalid file format for line {:?}", line).into());
            }
        }
    }
    if edges != exp_edges {
        return Err(format!("Expecting {} edges but read {} edges", exp_edges, edges).into());
    }
    if g.node_count() == 0 {
        return Err("Expecting graph order".into());
    }
    Ok(g)
}

pub struct EdgeIterator<'a> {
    pub graph: &'a UnGraphMap<u64, ()>,
    // We are going to iterate over the upper triangle of the adjacency matrix (i, j)
    pub i: u64,
    // current left vertex
    pub j: u64, // current right vertex
}

impl EdgeIterator<'_> {
    fn next_edge(&mut self) {
        self.j += 1;
        if self.j == self.graph.node_count() as u64 {
            self.i += 1;
            self.j = self.i + 1;
        }
    }
}

impl Iterator for EdgeIterator<'_> {
    type Item = (u64, u64);

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.graph.node_count() as u64;
        if n > 1 {
            self.next_edge();
            while self.i < n - 1 && !self.graph.contains_edge(
                self.i,
                self.j) {
                self.next_edge();
            }
            if self.i < n - 1 {
                return Some((self.i, self.j));
            }
        }
        None
    }
}

/// Returns an Iterator over the edges of the graph.
///
/// # Example
/// ```rust
/// use petgraph::prelude::UnGraphMap;
/// use vertex::graph_utils::edges;
///
/// let mut graph = UnGraphMap::<u64, ()>::new();
/// for i in 0..10 {
///     graph.add_node(i);
/// }
/// for i in 0..9 {
///     graph.add_edge(i, i+1, ());
/// }
///
/// let mut i = 0;
/// for (u, v) in edges(&graph) {
///     assert_eq!(u, i); // = i
///     assert_eq!(v, i + 1); // = j
///     i += 1;
/// }
/// assert_eq!(i, graph.edge_count() as u64);
/// ```
///
pub fn edges(graph: &UnGraphMap<u64, ()>) -> EdgeIterator {
    // TODO : regarder ça parce qu'il ne parcourt pas toutes les arêtes
    EdgeIterator {
        graph,
        i: 0,
        j: 0,
    }
}

/// Returns the vertex with the maximum degree in the graph and its degree.
///
/// # Example
/// ```rust
/// use petgraph::prelude::UnGraphMap;
///
/// use vertex::graph_utils::get_vertex_with_max_degree;
///
/// let mut graph = UnGraphMap::<u64, ()>::new();
/// for i in 0..10 {
///    graph.add_node(i);
/// }
/// for i in 0..9 {
///   graph.add_edge(i, i+1, ());
/// }
/// graph.add_edge(0, 9, ());
/// graph.add_edge(0, 8, ());
///
/// assert_eq!(get_vertex_with_max_degree(&graph).0, 0);
/// assert_eq!(get_vertex_with_max_degree(&graph).1, 3);
/// ```
pub fn get_vertex_with_max_degree(graph: &UnGraphMap<u64, ()>) -> (u64, usize) {
    let mut max_degree = 0;
    let mut max_degree_vertex = 0;
    for vertex in graph.nodes(){
        let degree = graph.neighbors(vertex).count();
        if degree > max_degree {
            max_degree = degree;
            max_degree_vertex = vertex;
        }
    }
    (max_degree_vertex, max_degree)
}

/// Since clone is not implemented for MatrixGraph, this function manually copies the graph.
/// It iterates over the nodes and edges of the graph and adds them to the copy.
///
/// # Example
/// ```rust
/// use petgraph::prelude::UnGraphMap;
///
/// use vertex::graph_utils::copy_graph;
///
/// let mut graph = UnGraphMap::<u64, ()>::new();
/// for i in 0..10 {
///   graph.add_node(i);
/// }
/// for i in 0..9 {
///  graph.add_edge(i, i+1, ());
/// }
///
/// let copy = copy_graph(&graph);
/// assert_eq!(copy.node_count(), 10);
/// assert_eq!(copy.edge_count(), 9);
/// ```
pub fn copy_graph(graph: &UnGraphMap<u64, ()>) -> UnGraphMap<u64, ()> {
    let mut copy = UnGraphMap::<u64, ()>::new();
    for i in graph.nodes() {
        copy.add_node(i);
    }
    for edge in graph.all_edges() {
        copy.add_edge(edge.0, edge.1, ());
    }
    copy
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct GraphInfo {
    id: String,
    format: String,
    order: usize,
    size: usize,
    mvc_val: u64,
}

/// Add the graph id with its format in the yaml file located at src/resources/graph_data.yml.
///
/// The default value for mvc_val is 0, it has to be updated manually.
/// If the graph id is already in the file, it is not added again.
///
/// # Panics
/// Panics if the file cannot be opened or the graph cannot be written to the file.
pub fn add_graph_to_yaml(id: &str, format: &str, graph: &UnGraphMap<u64, ()>, path: &str) {
    let file = File::open(path)
        .expect(format!("Unable to open file {:?}", path).as_str());
    let mut data: Vec<GraphInfo> = serde_yaml::from_reader(file).unwrap();

    if data.iter().any(|x| x.id == id) {
        // If the graph is already in the file, we don't add it again
        return;
    }

    let info = GraphInfo {
        id: id.to_string(),
        format: format.to_string(),
        order: graph.node_count(),
        size: graph.edge_count(),
        mvc_val: 0,
    };
    data.push(info);

    // Update the file
    let mut file = File::create(path)
        .expect(format!("Unable to create file {:?}", path).as_str());
    file.write_all(serde_yaml::to_string(&data).unwrap().as_bytes())
        .expect(format!("Unable to write file to {:?}", path).as_str());
}


/// Update the known value of the minimum vertex cover for a given graph id.
///
/// # Parameters
/// - id : the id of the graph (ex: test.clq)
/// - mvc_val : the new value of the minimum vertex cover
/// - path : the path to the yaml file containing the graph info (optionnal-> None or Some(path))
///
/// # Panics
/// Panics if :
/// - The file cannot be opened
/// - The graph id is not in the file
/// - The graph id cannot be updated (error while writing to the file)
///
/// # example
/// ```
/// use vertex::graph_utils::update_mvc_value;
///
/// update_mvc_value("test.clq", 3, Some("src/resources/graph_data.yml"));
/// // The value of the minimum vertex cover for the test.clq graph is now 3
///
/// update_mvc_value("test.clq", 2, None);
/// // The value of the minimum vertex cover for the test.clq graph is now 2
/// ```
pub fn update_mvc_value(id: &str, mvc_val: u64, path: Option<&str>) {
    let path = match path {
        Some(path) => path,
        None => "src/resources/graph_data.yml",
    };
    let file = File::open(path)
        .expect(format!("Unable to open file {:?}", path).as_str());

    let mut data: Vec<GraphInfo> = serde_yaml::from_reader(file).unwrap();

    let mut found = false;
    for info in data.iter_mut() {
        if info.id == id {
            info.mvc_val = mvc_val;
            found = true;
            break;
        }
    }
    if !found {
        panic!("Graph {:?} not found in {:?} to store the mvc : {:?}", id, path, mvc_val);
    }

    // Update the file
    let mut file = File::create(path)
        .expect(format!("Unable to create file {:?}", path).as_str());
    file.write_all(serde_yaml::to_string(&data).unwrap().as_bytes())
        .expect(format!("Unable to write file to {:?}", path).as_str());
}

/// Check if a given value is the optimal value for a given graph id.
/// The optimal value is the value stored in the yaml file. So, if the value in the yaml file is wrong, this function will return the wrong result.
///
/// # Parameters
/// - id : the id of the graph (ex: test.clq)
/// - val : the value to check
/// - path : the path to the yaml file containing the graph info (optionnal-> None or Some(path))
///
/// # Panics
/// Panics if :
/// - The file cannot be opened
/// - The graph id is not in the file
///
/// # example
/// ```
/// use vertex::graph_utils::is_optimal_value;
///
/// assert!(is_optimal_value("test.clq", 3, None));
/// assert!(!is_optimal_value("test.clq", 2, None));
/// ```
pub fn is_optimal_value(id: &str, val: u64, path: Option<&str>) -> bool {
    let path = match path {
        Some(path) => path,
        None => "src/resources/graph_data.yml",
    };
    let file = File::open(path)
        .expect(format!("Unable to open file {:?}", path).as_str());

    let data: Vec<GraphInfo> = serde_yaml::from_reader(file).unwrap();

    for info in data.iter() {
        if info.id == id {
            if info.mvc_val == val {
                return true;
            } else {
                return false;
            }
        }
    }
    panic!("Graph {:?} not found in {:?} to check if {:?} is optimal", id, path, val);
}

/// Get the optimal value for a given graph id.
/// The optimal value is the value stored in the yaml file. So, if the value in the yaml file is wrong,
/// this function will return the wrong result.
///
/// # Parameters
/// - id : the id of the graph (ex: test.clq)
/// - path : the path to the yaml file containing the graph info (optionnal-> None or Some(path))
///
/// # Panics
/// Panics if the file cannot be opened
///
/// # example
/// ```
/// use vertex::graph_utils::get_optimal_value;
///
/// assert_eq!(get_optimal_value("test.clq", None), Some(3));
/// assert_eq!(get_optimal_value("unknown_graph.clq", None), None);
/// ```
pub fn get_optimal_value(id: &str, path: Option<&str>) -> Option<u64> {
    let path = match path {
        Some(path) => path,
        None => "src/resources/graph_data.yml",
    };
    let file = File::open(path)
        .expect(format!("Unable to open file {:?}", path).as_str());

    let data: Vec<GraphInfo> = serde_yaml::from_reader(file).unwrap();

    for info in data.iter() {
        if info.id == id {
            return Some(info.mvc_val);
        }
    }
    return None;
}

#[cfg(test)]
mod graph_utils_tests {
    use super::*;

    #[test]
    fn test_edge_iterator() {
        let mut graph = UnGraphMap::<u64, ()>::new();
        for i in 0..10 {
            graph.add_node(i);
        }

        for i in 0..9 {
            graph.add_edge(i, i + 1, ());
        }

        let mut i = 0;
        for (u, v) in edges(&graph) {
            assert_eq!(u, i); // = i
            assert_eq!(v, i + 1); // = j
            i += 1;
        }
        assert_eq!(i, graph.edge_count() as u64);
    }

    #[test]
    fn test_edges_iterate_over_all_edges() {
        // TODO : Implémenter ceci avec l'exemple du test2.clq (c'est ça qui posait problème)
    }

    #[test]
    fn test_is_vertex_cover() {
        let mut graph = UnGraphMap::<u64, ()>::new();
        for i in 0..3 {
            graph.add_node(i);
        }
        graph.add_edge(0, 1, ());
        graph.add_edge(1, 2, ());
        graph.add_edge(2, 0, ());
        let mut vertex_cover: Vec<u64> = Vec::new();
        vertex_cover.push(0);
        assert!(!is_vertex_cover(&graph, &vertex_cover));
        vertex_cover.push(1);
        assert!(is_vertex_cover(&graph, &vertex_cover));
        vertex_cover.push(2);
        assert!(is_vertex_cover(&graph, &vertex_cover));
    }

    #[test]
    fn test_get_vertex_with_max_degree() {
        let mut graph = UnGraphMap::<u64, ()>::new();
        for i in 0..10 {
            graph.add_node(i);
        }
        for i in 0..9 {
            graph.add_edge(i, i+1, ());
        }
        graph.add_edge(0, 9, ());
        graph.add_edge(0, 8, ());
        graph.add_edge(0, 7, ());
        assert_eq!(get_vertex_with_max_degree(&graph).0, 0);
        assert_eq!(get_vertex_with_max_degree(&graph).1, 4);
    }

    #[test]
    fn test_get_vertex_with_max_degree2() {
        let mut graph = UnGraphMap::<u64, ()>::new();
        for i in 3..7 {
            graph.add_node(i);
        }
        graph.add_edge(3, 4, ());
        graph.add_edge(4, 5, ());
        graph.add_edge(5, 6, ());

        assert_eq!(get_vertex_with_max_degree(&graph).0, 4);
        assert_eq!(get_vertex_with_max_degree(&graph).1, 2);
    }

    #[test]
    fn test_copy_graph() {
        let mut graph = UnGraphMap::<u64, ()>::new();
        for i in 0..10 {
            graph.add_node(i);
        }
        for i in 0..9 {
            graph.add_edge(i, i+1, ());
        }

        let mut copy = copy_graph(&graph);
        graph.remove_edge(0, 1);
        assert_eq!(graph.edge_count(), 8);
        assert_eq!(copy.edge_count(), 9);

        graph.remove_node(0);
        assert_eq!(graph.node_count(), 9);
        assert_eq!(copy.node_count(), 10);

        copy.remove_node(0);
        copy.remove_node(1);
        assert_eq!(copy.node_count(), 8);
        assert_eq!(graph.node_count(), 9);

    }
    #[test]
    fn test_load_clq_file() {
        let graph = load_clq_file("src/resources/graphs/test.clq").unwrap();
        assert_eq!(graph.node_count(), 5);
        assert!(graph.contains_edge(0, 1));
        assert!(graph.contains_edge(0, 2));
        assert!(graph.contains_edge(0, 3));
        assert!(graph.contains_edge(2, 3));
        assert!(graph.contains_edge(4, 0));
        assert!(graph.contains_edge(4, 1));
    }
}