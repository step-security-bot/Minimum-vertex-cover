use std::cmp::min;

use petgraph::prelude::UnGraphMap;

use crate::graph_utils::{copy_graph, get_vertex_with_max_degree};

pub fn solve(graph: &UnGraphMap<u64, ()>) -> u64 {
    // Initialize the upper bound to the number of nodes in the graph
    // and the vertex cover found so far is empty
    b_and_b(graph, graph.node_count() as u64, vec![])
}

fn b_and_b(graph: &UnGraphMap<u64, ()>,
           upper_bound: u64,
           vertex_cover: Vec<u64>) -> u64 {

    if vertex_cover.len() as u64 >= upper_bound {
        return upper_bound;
    }
    // If the vertex cover found so far + the minimum vertex that needs to be added are greater than the UB
    // if vertex_cover.len() as u64 + max(deg_lb(graph), sat_lb(graph)) >= upper_bound {
    //    return upper_bound;
    // }
    // If the graph is empty. (All edges have been covered)
    if graph.edge_count() == 0 {
        let _res = vertex_cover.len() as u64;
        return vertex_cover.len() as u64;
    }

    let v = get_vertex_with_max_degree(graph);

    let mut graph_case1 = copy_graph(graph);
    let mut graph_case2 = copy_graph(graph);

    // ====> First case <====
    // - G \ N*(v)
    // - C U N(v)
    let mut vertex_cover_case1 = vertex_cover.clone();

    for neighbor in graph.neighbors(v) {
        vertex_cover_case1.push(neighbor);
        graph_case1.remove_node(neighbor);
    }
    graph_case1.remove_node(v);
    let res_case1 = b_and_b(&graph_case1,
                            upper_bound,
                            vertex_cover_case1);

    // ====> Second case <====
    // - G \ {v}
    // - C U v
    let mut vertex_cover_case2 = vertex_cover.clone();
    graph_case2.remove_node(v);
    vertex_cover_case2.push(v);
    let res_case2 = b_and_b(&graph_case2,
                            min(upper_bound, res_case1),
                            vertex_cover_case2);
    
    return min(res_case1, res_case2);
}


#[allow(dead_code)]
fn deg_lb(_graph: &UnGraphMap<u64, ()>) -> u64 {
    todo!("Implement lower bound based on degree")
}

#[allow(dead_code)]
fn sat_lb(_graph: &UnGraphMap<u64, ()>) -> u64 {
    todo!("Implement lower bound based on satisfiability")
}

#[allow(dead_code)]
fn clq_lb(_graph: &UnGraphMap<u64, ()>) -> u64 {
    todo!("Implement lower bound based on clique AND see if it's useful")
}


#[cfg(test)]
mod branch_and_bound_tests {
    use crate::graph_utils::load_clq_file;

    use super::*;

    #[test]
    fn test_b_and_b() {
        let mut graph = UnGraphMap::<u64, ()>::new();
        for i in 0..4 {
            graph.add_node(i);
        }
        graph.add_edge(0, 1, ());
        graph.add_edge(1, 2, ());
        graph.add_edge(2, 0, ());
        graph.add_edge(2, 3, ());

        assert_eq!(solve(&graph), 2);
        //assert!(is_vertex_cover(&graph, &solve(&graph).1));
    }

    #[test]
    fn test_with_queen_5() {
        let graph = load_clq_file("src/resources/graphs/queen5_5.clq").unwrap();
        let res = solve(&graph);
        //assert!(is_vertex_cover(&graph, &res.1));
        assert_eq!(res, 20);
    }
}
