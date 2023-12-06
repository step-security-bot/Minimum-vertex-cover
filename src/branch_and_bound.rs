use std::cmp::max;
use std::collections::HashMap;

use petgraph::prelude::UnGraphMap;

use crate::Clock;
use crate::graph_utils::{complement, copy_graph, get_vertex_with_max_degree};

pub fn solve(graph: &Box<UnGraphMap<u64, ()>>, clock: &mut Clock) -> (u64, Vec<u64>) {
    // Initialize the upper bound to the number of nodes in the graph
    // and the vertex cover found so far is empty
    let upper_bound_vc = &graph.nodes().collect();
    let u = b_and_b(graph, graph, graph.node_count() as u64,
                    upper_bound_vc, vec![], clock);
    let total_time = clock.get_time().duration.as_secs_f64();
    println!("Time spent in deg : {}%", clock.deg_lb.as_secs_f64() / total_time);
    println!("Time spent in clq : {}%", clock.clq_lb.as_secs_f64() / total_time);
    println!("Time spent in max deg : {}%", clock.max_deg.as_secs_f64() / total_time);
    println!("Time spent in copy : {}%", clock.copy.as_secs_f64() / total_time);
    u
}

fn b_and_b<'a>(graph: &UnGraphMap<u64, ()>,
               subgraph: &Box<UnGraphMap<u64, ()>>,
               upper_bound: u64,
               upper_bound_vc: &Vec<u64>,
               vertex_cover: Vec<u64>,
               clock: &mut Clock) -> (u64, Vec<u64>) {
    if clock.is_time_up() {
        return (upper_bound, upper_bound_vc.clone());
    }

    clock.enter_copy();
    let mut subgraph = copy_graph(subgraph);
    clock.exit_copy();

    if subgraph.edge_count() == 0 {
        // If the subgraph is empty, all edges are covered => vertex cover
        return (vertex_cover.len() as u64, vertex_cover);
    }

    clock.enter_deg();
    let deg_lb = deg_lb(&subgraph);
    clock.exit_deg();
    clock.enter_clq();
    // let clq_lb = clq_lb(subgraph);
    clock.exit_clq();
    let lb = max(deg_lb, 0);

    if vertex_cover.len() as u64 + lb >= upper_bound {
        // We can't find a better solution in this branch, we stop and return the best known solution
        return (upper_bound, upper_bound_vc.clone());
    }

    clock.enter_max_deg();
    let (v, _max_deg) = get_vertex_with_max_degree(&subgraph, None);

    clock.exit_max_deg();

    let neighbors: Vec<u64> = subgraph.neighbors(v).collect();

    // ====> First case <====
    // - G \ {v}
    // - C U v
    let mut vertex_cover_case1 = vertex_cover.clone();

    // Removes v + edges from v to neighbor
    subgraph.remove_node(v);
    vertex_cover_case1.push(v);
    let res_case1 = b_and_b(graph,
                            &subgraph,
                            upper_bound,
                            upper_bound_vc,
                            vertex_cover_case1, clock);

    // ====> Second case <====
    // - G \ N*(v)
    // - C U N(v)
    let mut vertex_cover_case2 = vertex_cover.clone();

    // Remove all neighbors of v + edges from neighbors to their neighbors
    for neighbor in neighbors {
        vertex_cover_case2.push(neighbor);
        subgraph.remove_node(neighbor);
    }

    let res_case2 = {
        if upper_bound >= res_case1.0 {
            b_and_b(graph,
                    &subgraph,
                    res_case1.0,
                    &res_case1.1,
                    vertex_cover_case2, clock)
        } else {
            b_and_b(graph,
                    &subgraph,
                    upper_bound,
                    upper_bound_vc,
                    vertex_cover_case2,
                    clock)
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

fn deg_lb(graph: &Box<UnGraphMap<u64, ()>>) -> u64 {
    let size = graph.edge_count();
    let mut selected_vertexes = Vec::<u64>::new();
    let mut sum_degrees: usize = 0;


    let mut working = true;
    while working {
        let (max_degree_vertex, vertex_degree) = get_vertex_with_max_degree(graph, Some(&selected_vertexes));
        selected_vertexes.push(max_degree_vertex);
        sum_degrees += vertex_degree;
        if sum_degrees >= size {
            working = false;
        }
    }

    let mut edges_left = 0;
    for (u, v, ()) in graph.all_edges() {
        if !selected_vertexes.contains(&u) && !selected_vertexes.contains(&v) {
            edges_left += 1;
        }
    }

    if edges_left == 0 {
        selected_vertexes.len() as u64
    } else {
        let estim = (edges_left / graph.neighbors(get_vertex_with_max_degree(&graph, Some(&selected_vertexes)).0).count()) as f64;
        selected_vertexes.len() as u64 + estim.floor() as u64
    }
}

#[allow(dead_code)]
fn sat_lb(_graph: &UnGraphMap<u64, ()>) -> u64 {
    todo!("Implement lower bound based on satisfiability")
}


// TODO : make this private
pub fn clq_lb(graph: &Box<UnGraphMap<u64, ()>>) -> u64 {
    // 1) Get the complement of the graph
    // 2) Find a greedy coloring of the complement
    // 3) Each color is a independent set
    // 4) An independent set in the complement is a clique in the original graph
    // 5) Adds the numbers of nodes in each clique minus 1 (a clique is a complete graph)

    // 1) Get the complement of the graph
    let compl = complement(graph);

    // 2) Find a greedy coloring of the complement
    let color_set = greedy_coloring(&compl);


    // Adds the number of nodes in each color minus 1 = lower bound. If a value is 0, change it to 1
    color_set.iter().map(|&x| x as u64 - 1).sum::<u64>()
}


// Color the graph such that every node has a different color than its neighbors.
// This algorithm returns a vector containing the number of vertex in each color.
fn greedy_coloring(graph: &Box<UnGraphMap<u64, ()>>) -> Vec<usize> {
    // 1. Create a color set. The vertex degree of each vertex is calculated and the vertex degrees are added to
    let mut color_set = Vec::new(); // color_set[i] = j means that color i has j vertexes
    let mut colors = HashMap::new();
    for i in graph.nodes() {
        colors.insert(i, 0);
    }

    let mut vertices_ordered_by_deg: Vec<_> = graph.nodes().collect();
    vertices_ordered_by_deg.sort_by_key(|&i| std::cmp::Reverse(graph.neighbors(i).count()));


    for vertex in vertices_ordered_by_deg {
        let mut color = 0;
        let mut color_found = false;
        while !color_found {
            if color_set.len() <= color {
                color_set.push(1);
                colors.insert(vertex, color);
                color_found = true;
            } else {
                let is_conflict = {
                    let mut is_conflict = false;
                    for neighbor in graph.neighbors(vertex) {
                        if *colors.get(&neighbor).unwrap() == color {
                            is_conflict = true;
                        }
                    }
                    is_conflict
                };
                if is_conflict {
                    color += 1;
                } else {
                    color_set[color] += 1;
                    colors.insert(vertex, color);
                    color_found = true;
                }
            }
        }
    }
    color_set
}


/// Solve the maximum clique problem by solving the minimum vertex cover problem.  
/// Algorithm : 
/// 1. Computes the complement of the graph
/// 2. Solves the minimum vertex cover problem on the complement
/// 3. The vertices that are not in the minimum vertex cover are in the maximum independent set
/// 4. An independent set in the complement is a clique in the original graph.
///
pub fn solve_clq(graph: &Box<UnGraphMap<u64, ()>>, clock: &mut Clock) -> (u64, Vec<u64>) {

    // We know that each clique correspond to an independent set in the complement of the graph
    let mut cmpl = complement(graph);

    // The complement of a maximum independent set is a minimum vertex cover
    let upper_bound_vc = &cmpl.nodes().collect();
    let mvc_res = b_and_b(graph, &mut cmpl,
                          graph.node_count() as u64, upper_bound_vc,
                          vec![], clock);


    // The complement of a minimum vertex cover is a maximum independent set
    let mut res_set = Vec::new();
    for node in cmpl.nodes() {
        if !mvc_res.1.contains(&node) {
            res_set.push(node);
        }
    }

    // |V| - |MVC| = |MIS|
    (graph.node_count() as u64 - mvc_res.0, res_set)
}

#[cfg(test)]
mod branch_and_bound_tests {
    use crate::graph_utils::load_clq_file;

    use super::*;

    #[test]
    fn test_greedy_coloring() {
        let mut graph = Box::new(UnGraphMap::<u64, ()>::new());
        for i in 0..5 {
            graph.add_node(i);
        }
        graph.add_edge(0, 1, ());
        graph.add_edge(0, 2, ());
        graph.add_edge(1, 3, ());
        graph.add_edge(2, 4, ());
        graph.add_edge(3, 4, ());

        let res = greedy_coloring(&graph);
        assert_eq!(res, vec![2, 2, 1])
    }

    #[test]
    fn test_greedy_coloring_paper_graph() {
        let graph = load_clq_file("src/resources/graphs/oui.clq").unwrap();
        let res = greedy_coloring(&graph);
        assert_eq!(res, vec![3, 3, 1])
    }

    #[test]
    fn test_problem_with_coloring() {
        let mut graph = Box::new(UnGraphMap::<u64, ()>::new());
        graph.add_node(4);
        graph.add_node(5);
        graph.add_node(6);
        graph.add_edge(4, 5, ());
        graph.add_edge(5, 6, ());

        let res = greedy_coloring(&graph);
        assert_eq!(res, vec![1, 2])
    }

    #[test]
    fn test_deg_lb() {
        let graph = match load_clq_file("src/resources/graphs/test_cycle_5.clq") {
            Ok(g) => g,
            Err(e) => panic!("{}", e)
        };
        let res = deg_lb(&graph);
        assert_eq!(res, 3);
    }

    #[test]
    fn test_b_and_b() {
        let mut graph = Box::new(UnGraphMap::<u64, ()>::new());
        for i in 0..4 {
            graph.add_node(i);
        }
        graph.add_edge(0, 1, ());
        graph.add_edge(1, 2, ());
        graph.add_edge(2, 0, ());
        graph.add_edge(2, 3, ());

        assert_eq!(solve(&graph, &mut Clock::new(3600)).0, 2);
    }

    #[test]
    fn test_with_queen_5() {
        let graph = load_clq_file("src/resources/graphs/queen5_5.clq").unwrap();
        let res = solve(&graph, &mut Clock::new(3600));
        assert_eq!(res.0, 20);
    }
}