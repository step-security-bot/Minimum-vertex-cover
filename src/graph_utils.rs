use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

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
/// use graph::{Graph, GraphConstructible, GraphNauty};
/// use vertex::graph_utils::load_clq_file;
///
/// let graph = load_clq_file("src/resources/graphs/test.clq").unwrap();
/// assert_eq!(graph.order(), 5);
/// assert!(graph.is_edge(0, 1));
/// assert!(graph.is_edge(0, 2));
/// assert!(graph.is_edge(0, 3));
/// assert!(graph.is_edge(2, 3));
/// assert!(graph.is_edge(4, 0));
/// assert!(graph.is_edge(4, 1));
/// ```
pub fn load_clq_file(path: &str) -> Result<GraphNauty, Box<dyn Error>> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => return Err(format!("File {:?} not found \n {:?}", path, e).into()),
    };
    let reader = BufReader::new(file);

    let mut g = GraphNauty::new(0);
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
                for _ in 0..order {
                    g.add_vertex();
                }
            }
            "e" => {
                if g.order() == 0 {
                    return Err("Expecting graph order".into());
                }
                g.add_edge(values[1].parse::<u64>()? - 1, values[2].parse::<u64>()? - 1);
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
    if g.order() == 0 {
        return Err("Expecting graph order".into());
    }
    Ok(g)
}

#[cfg(test)]
mod graph_utils_tests {
    use graph::GraphNauty;

    use super::*;

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

    #[test]
    fn test_load_clq_file() {
        let graph = load_clq_file("src/resources/graphs/test.clq").unwrap();
        assert_eq!(graph.order(), 5);
        assert!(graph.is_edge(0, 1));
        assert!(graph.is_edge(0, 2));
        assert!(graph.is_edge(0, 3));
        assert!(graph.is_edge(2, 3));
        assert!(graph.is_edge(4, 0));
        assert!(graph.is_edge(4, 1));
    }
}