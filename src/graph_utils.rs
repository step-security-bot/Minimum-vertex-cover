use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

use petgraph::matrix_graph::MatrixGraph;
use petgraph::stable_graph::NodeIndex;
use petgraph::Undirected;
use serde::{Deserialize, Serialize};

/// Check if a given vertex cover is a vertex cover of a given graph.
///
/// Parse all edges of the graph and check if at least one of the vertices of each edge is in the vertex cover.
/// If not, return false.
///
/// # Example
/// ```rust
/// use petgraph::matrix_graph::MatrixGraph;
/// use petgraph::Undirected;
/// use petgraph::stable_graph::NodeIndex;
/// use vertex::graph_utils::is_vertex_cover;
///
/// let mut graph = MatrixGraph::<u64, (), Undirected>::new_undirected();
/// for i in 0..3 {
///    graph.add_node(i);
/// }
/// graph.add_edge(NodeIndex::new(0), NodeIndex::new(1), ());
/// graph.add_edge(NodeIndex::new(1), NodeIndex::new(2), ());
/// graph.add_edge(NodeIndex::new(2), NodeIndex::new(0), ());
/// let mut vertex_cover: Vec<u64> = Vec::new();
/// vertex_cover.push(0);
/// assert!(!is_vertex_cover(&graph, &vertex_cover));
/// vertex_cover.push(1);
/// assert!(is_vertex_cover(&graph, &vertex_cover));
/// ```
pub fn is_vertex_cover(graph: &MatrixGraph<u64, (), Undirected>, vertex_cover: &Vec<u64>) -> bool {
    for (i, j) in edges(graph) {
        if !vertex_cover.contains(&(i as u64)) && !vertex_cover.contains(&(j as u64)) {
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
/// use petgraph::matrix_graph::MatrixGraph;
/// use petgraph::stable_graph::NodeIndex;
/// use vertex::graph_utils::load_clq_file;
///
/// let graph = load_clq_file("src/resources/graphs/test.clq").unwrap();
/// assert_eq!(graph.node_count(), 5);
/// assert!(graph.has_edge(NodeIndex::new(0), NodeIndex::new(1)));
/// assert!(graph.has_edge(NodeIndex::new(0), NodeIndex::new(2)));
/// assert!(graph.has_edge(NodeIndex::new(0), NodeIndex::new(3)));
/// assert!(graph.has_edge(NodeIndex::new(2), NodeIndex::new(3)));
/// assert!(graph.has_edge(NodeIndex::new(4), NodeIndex::new(0)));
/// assert!(graph.has_edge(NodeIndex::new(4), NodeIndex::new(1)));
/// ```
pub fn load_clq_file(path: &str) -> Result<MatrixGraph<u64, (), Undirected>, Box<dyn Error>> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => return Err(format!("File {:?} not found \n {:?}", path, e).into()),
    };
    let reader = BufReader::new(file);

    let mut g = MatrixGraph::<u64, (), Undirected>::new_undirected();
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
                let i = values[1].parse::<usize>()? - 1;
                let j = values[2].parse::<usize>()? - 1;

                g.add_edge(NodeIndex::new(i), NodeIndex::new(j), ());
                edges += 1;
            }
            _ => {
                return Err(format!("Invalid file format for line {:?}", line).into());
            }
        }
    }
    if edges != exp_edges {
        return Err(format!("Expecting {} edges but readed {} edges", exp_edges, edges).into());
    }
    if g.node_count() == 0 {
        return Err("Expecting graph order".into());
    }
    Ok(g)
}

pub struct EdgeIterator<'a> {
    pub graph: &'a MatrixGraph<u64, (), Undirected>,
    // We are going to iterate over the upper triangle of the adjacency matrix (i, j)
    pub i: usize,
    // current left vertex
    pub j: usize, // current right vertex
}

impl EdgeIterator<'_> {
    fn next_edge(&mut self) {
        self.j += 1;
        if self.j == self.graph.node_count() {
            self.i += 1;
            self.j = self.i + 1;
        }
    }
}

impl Iterator for EdgeIterator<'_> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.graph.node_count();
        if n > 1 {
            self.next_edge();
            while self.i < n - 1 && !self.graph.has_edge(
                NodeIndex::new(self.i),
                NodeIndex::new(self.j)) {
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
/// use petgraph::matrix_graph::MatrixGraph;
/// use petgraph::Undirected;
/// use petgraph::stable_graph::NodeIndex;
/// use vertex::graph_utils::edges;
///
/// let mut graph = MatrixGraph::<u64, (), Undirected>::new_undirected();
/// for i in 0..10 {
///     graph.add_node(i);
/// }
/// for i in 0..9 {
///     graph.add_edge(NodeIndex::new(i), NodeIndex::new(i + 1), ())
/// }
///
/// let mut i = 0;
/// for (u, v) in edges(&graph) {
///     assert_eq!(u, i); // = i
///     assert_eq!(v, i + 1); // = j
///     i += 1;
/// }
/// assert_eq!(i, graph.edge_count());
/// ```
///
pub fn edges(graph: &MatrixGraph<u64, (), Undirected>) -> EdgeIterator {
    EdgeIterator {
        graph,
        i: 0,
        j: 0,
    }
}

/// Returns the vertex with the maximum degree in the graph.
///
/// # Example
/// ```rust
/// use petgraph::matrix_graph::MatrixGraph;
/// use petgraph::Undirected;
/// use petgraph::stable_graph::NodeIndex;
///
/// use vertex::graph_utils::get_vertex_with_max_degree;
///
/// let mut graph = MatrixGraph::<u64, (), Undirected>::new_undirected();
/// for i in 0..10 {
///    graph.add_node(i);
/// }
/// for i in 0..9 {
///   graph.add_edge(NodeIndex::new(i), NodeIndex::new(i + 1), ())
/// }
/// graph.add_edge(NodeIndex::new(0), NodeIndex::new(9), ());
/// graph.add_edge(NodeIndex::new(0), NodeIndex::new(8), ());
///
/// assert_eq!(get_vertex_with_max_degree(&graph), 0);
/// ```
pub fn get_vertex_with_max_degree(graph: &MatrixGraph<u64, (), Undirected>) -> usize {
    let mut max_degree = 0;
    let mut max_degree_vertex = 0;
    for vertex in 0..graph.node_count() {
        let degree = graph.neighbors(NodeIndex::new(vertex)).count();
        if degree > max_degree {
            max_degree = degree;
            max_degree_vertex = vertex;
        }
    }
    max_degree_vertex
}

/// Since clone is not implemented for MatrixGraph, this function manually copies the graph.
/// It iterates over the nodes and edges of the graph and adds them to the copy.
///
/// # Example
/// ```rust
/// use petgraph::matrix_graph::MatrixGraph;
/// use petgraph::Undirected;
/// use petgraph::stable_graph::NodeIndex;
///
/// use vertex::graph_utils::copy_graph;
///
/// let mut graph = MatrixGraph::<u64, (), Undirected>::new_undirected();
/// for i in 0..10 {
///   graph.add_node(i);
/// }
/// for i in 0..9 {
///  graph.add_edge(NodeIndex::new(i), NodeIndex::new(i + 1), ())
/// }
///
/// let copy = copy_graph(&graph);
/// assert_eq!(copy.node_count(), 10);
/// assert_eq!(copy.edge_count(), 9);
/// ```
pub fn copy_graph(graph: &MatrixGraph<u64, (), Undirected>) -> MatrixGraph<u64, (), Undirected> {
    let mut copy = MatrixGraph::<u64, (), Undirected>::new_undirected();
    for i in 0..graph.node_count() {
        copy.add_node(i as u64);
    }
    for (u, v) in edges(&graph) {
        copy.add_edge(NodeIndex::new(u), NodeIndex::new(v), ());
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
///
/// # Example
/// TODO : add example
pub fn add_graph_to_yaml(id: &str, format: &str, graph: &MatrixGraph<u64, (), Undirected>, path: &str) {
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
/// # Panics
/// Panics if :
/// - The file cannot be opened
/// - The graph id is not in the file
/// - The graph id cannot be updated (error while writing to the file)
///
/// # example
/// TODO : add example
pub fn update_mvc_value(id: &str, mvc_val: u64, path: &str) {
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

#[cfg(test)]
mod graph_utils_tests {
    use super::*;

    #[test]
    fn test_edge_iterator() {
        let mut graph = MatrixGraph::<u64, (), Undirected>::new_undirected();
        for i in 0..10 {
            graph.add_node(i);
        }

        for i in 0..9 {
            graph.add_edge(NodeIndex::new(i), NodeIndex::new(i + 1), ())
        }

        let mut i = 0;
        for (u, v) in edges(&graph) {
            assert_eq!(u, i); // = i
            assert_eq!(v, i + 1); // = j
            i += 1;
        }
        assert_eq!(i, graph.edge_count());
    }

    #[test]
    fn test_is_vertex_cover() {
        let mut graph = MatrixGraph::<u64, (), Undirected>::new_undirected();
        for i in 0..3 {
            graph.add_node(i);
        }
        graph.add_edge(NodeIndex::new(0), NodeIndex::new(1), ());
        graph.add_edge(NodeIndex::new(1), NodeIndex::new(2), ());
        graph.add_edge(NodeIndex::new(2), NodeIndex::new(0), ());
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
        let mut graph = MatrixGraph::<u64, (), Undirected>::new_undirected();
        for i in 0..10 {
            graph.add_node(i);
        }
        for i in 0..9 {
            graph.add_edge(NodeIndex::new(i), NodeIndex::new(i + 1), ())
        }
        graph.add_edge(NodeIndex::new(0), NodeIndex::new(9), ());
        graph.add_edge(NodeIndex::new(0), NodeIndex::new(8), ());
        graph.add_edge(NodeIndex::new(0), NodeIndex::new(7), ());
        assert_eq!(get_vertex_with_max_degree(&graph), 0);
    }

    #[test]
    fn test_copy_graph() {
        let mut graph = MatrixGraph::<u64, (), Undirected>::new_undirected();
        for i in 0..10 {
            graph.add_node(i);
        }
        for i in 0..9 {
            graph.add_edge(NodeIndex::new(i), NodeIndex::new(i + 1), ())
        }

        let mut copy = copy_graph(&graph);
        graph.remove_edge(NodeIndex::new(0), NodeIndex::new(1));
        assert_eq!(graph.edge_count(), 8);
        assert_eq!(copy.edge_count(), 9);

        graph.remove_node(NodeIndex::new(0));
        assert_eq!(graph.node_count(), 9);
        assert_eq!(copy.node_count(), 10);

        copy.remove_node(NodeIndex::new(0));
        copy.remove_node(NodeIndex::new(1));
        assert_eq!(copy.node_count(), 8);
        assert_eq!(graph.node_count(), 9);

    }
    #[test]
    fn test_load_clq_file() {
        let graph = load_clq_file("src/resources/graphs/test.clq").unwrap();
        assert_eq!(graph.node_count(), 5);
        assert!(graph.has_edge(NodeIndex::new(0), NodeIndex::new(1)));
        assert!(graph.has_edge(NodeIndex::new(0), NodeIndex::new(2)));
        assert!(graph.has_edge(NodeIndex::new(0), NodeIndex::new(3)));
        assert!(graph.has_edge(NodeIndex::new(2), NodeIndex::new(3)));
        assert!(graph.has_edge(NodeIndex::new(4), NodeIndex::new(0)));
        assert!(graph.has_edge(NodeIndex::new(4), NodeIndex::new(1)));
    }
}