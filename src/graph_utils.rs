use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use petgraph::matrix_graph::MatrixGraph;
use petgraph::stable_graph::NodeIndex;
use petgraph::Undirected;

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
/// c Source: Cyril Moreau
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