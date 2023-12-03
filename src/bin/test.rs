use std::cmp::max;

use petgraph::prelude::UnGraphMap;

use vertex::branch_and_bound::{clq_lb, deg_lb};
use vertex::graph_utils::{copy_graph, get_vertex_with_max_degree, is_vertex_cover};

fn main() {
    println!("Hello, world!");
}

pub fn solve_b(graph: &UnGraphMap<u64, ()>) -> (u64, Vec<u64>) {
    let mut copy = copy_graph(graph);
    // Initialize the upper bound to the number of nodes in the graph
    // and the vertex cover found so far is empty
    let upper_bound_vc = &copy.nodes().collect();
    let u = b_and_b(graph, &mut copy, graph.node_count() as u64, upper_bound_vc, vec![]);
    u
}


fn b_and_b<'a>(graph: &UnGraphMap<u64, ()>,
               subgraph: &UnGraphMap<u64, ()>,
               upper_bound: u64,
               upper_bound_vc: &Vec<u64>,
               vertex_cover: Vec<u64>) -> (u64, Vec<u64>) {


    if subgraph.edge_count() == 0 {
        return (vertex_cover.len() as u64, vertex_cover);
    }

    let deg_lb = deg_lb(subgraph);
    let clq_lb = clq_lb(subgraph);
    let lb = max(deg_lb, clq_lb);

    if vertex_cover.len() as u64 + lb >= upper_bound {
        return (upper_bound, upper_bound_vc.clone());
    }

    let (v, _max_deg) = get_vertex_with_max_degree(subgraph, None);

    let mut subgraph1 = copy_graph(subgraph);
    let mut subgraph2 = copy_graph(subgraph);

    // ====> First case <====
    // - G \ N*(v)
    // - C U N(v)
    let mut vertex_cover_case1 = vertex_cover.clone();

    // Remove all neighbors of v + edges from neighbors to their neighbors
    for neighbor in subgraph2.neighbors(v) {
        vertex_cover_case1.push(neighbor);
        subgraph1.remove_node(neighbor);
    }
    subgraph1.remove_node(v);

    let res_case1 = b_and_b(graph,
                            &subgraph1,
                            upper_bound,
                            upper_bound_vc,
                            vertex_cover_case1);

    // ====> Second case <====
    // - G \ {v}
    // - C U v
    let mut vertex_cover_case2 = vertex_cover.clone();

    // Removes v + edges from v to neighbor
    subgraph2.remove_node(v);
    vertex_cover_case2.push(v);
    let res_case2 = {
        if upper_bound >= res_case1.0 {
            b_and_b(graph,
                    &subgraph2,
                    res_case1.0,
                    &res_case1.1,
                    vertex_cover_case2)
        } else {
            b_and_b(graph,
                    &subgraph2,
                    upper_bound,
                    upper_bound_vc,
                    vertex_cover_case2)
        }
    };

    return {
        if res_case1.0 >= res_case2.0 {
            res_case2
        } else {
            res_case1
        }
    };
}


fn _naive(graph: &UnGraphMap<u64, ()>, features: &mut Vec<u64>, max_length: u64) -> () {
    if features.len() > max_length as usize {
        return;
    }
    if features.len() == max_length as usize {
        if is_vertex_cover(&graph, &features) {
            println!("{:?}", features);
        };
    }

    for node in graph.nodes() {
        if !features.contains(&node) {
            features.push(node);
            _naive(graph, features, max_length);
            features.pop();
        }
    }
}