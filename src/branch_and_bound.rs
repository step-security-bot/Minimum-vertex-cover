use std::cmp::min;

use petgraph::graph::NodeIndex;
use petgraph::matrix_graph::MatrixGraph;
use petgraph::Undirected;

use crate::graph_utils::{copy_graph, edges, get_vertex_with_max_degree};

pub fn solve(graph: &MatrixGraph<u64, (), Undirected>) -> u64 {
    // Initialize the upper bound to the number of nodes in the graph
    // and the vertex cover found so far is empty
    b_and_b(graph, graph.node_count() as u64, vec![])
}

fn b_and_b(graph: &MatrixGraph<u64, (), Undirected>,
           upper_bound: u64,
           vertex_cover: Vec<u64>) -> u64 {


    // If the vertex cover found so far + the minimum vertex that needs to be added are greater than the UB
    // if vertex_cover.len() as u64 + max(deg_lb(graph), sat_lb(graph)) >= upper_bound {
    //    return upper_bound;
    // }
    // If the graph is empty. (All edges have been covered)
    // TODO : need to check what to do with isolated nodes
    if graph.edge_count() == 0 || graph.node_count() == 0 {
        return vertex_cover.len() as u64;
    }

    // Select a vertex with the maximum degree to branch on
    let v = get_vertex_with_max_degree(graph);
    // Branch on 2 cases : v is in the vertex cover or v is not in the vertex cover
    let mut graph_v_not_in_cover = copy_graph(graph);
    let mut graph_v_in_cover = copy_graph(graph);

    // ====> First case <====
    let mut vertex_cover_v_in_cover = vertex_cover.clone();

    println!("v = {} and node count = {} and node cound for cover = {}", v, graph.node_count(), graph_v_in_cover.node_count());

    for (v, u) in edges(graph) {
        println!("v = {} and u = {}", v, u)
    }

    println!("===== vertex graph ====");

    for (v, u) in edges(&graph_v_in_cover) {
        println!("v = {} and u = {}", v, u)
    }

    for neighbor in graph.neighbors(NodeIndex::new(v)) {
        println!("neighbor = {:?}", neighbor);
        for (v, u) in edges(graph) {
            println!("v = {} and u = {}", v, u)
        }
        vertex_cover_v_in_cover.push(neighbor.index() as u64);
        graph_v_in_cover.remove_node(neighbor);
    }
    graph_v_in_cover.remove_node(NodeIndex::new(v));
    let res_v_in_cover = b_and_b(&graph_v_in_cover,
                                 upper_bound,
                                 vertex_cover_v_in_cover);

    // ====> Second case <====
    let vertex_cover_v_not_in_cover = vertex_cover.clone();

    graph_v_not_in_cover.remove_node(NodeIndex::new(v));
    let res_v_not_in_cover = b_and_b(&graph_v_not_in_cover,
                                     min(upper_bound, res_v_in_cover),
                                     vertex_cover_v_not_in_cover);
    
    return min(res_v_in_cover, res_v_not_in_cover);
}


#[allow(dead_code)]
fn deg_lb(_graph: &MatrixGraph<u64, (), Undirected>) -> u64 {
    todo!("Implement lower bound based on degree")
}

#[allow(dead_code)]
fn sat_lb(_graph: &MatrixGraph<u64, (), Undirected>) -> u64 {
    todo!("Implement lower bound based on satisfiability")
}

#[allow(dead_code)]
fn clq_lb(_graph: &MatrixGraph<u64, (), Undirected>) -> u64 {
    todo!("Implement lower bound based on clique AND see if it's usefull")
}
