use std::cmp::max;
use std::collections::HashMap;
use std::env;

use round::round;

use vertex::{Clock, MVCResult};
use vertex::graph_utils::add_time_to_yaml;
use vertex::mvcgraph::{load_clq_file, MVCGraph};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        let graph = load_clq_file(&format!("src/resources/graphs/{}", args[1]))
            .expect("Error while loading graph");

        test_val(&args[1], &graph);
    }
}


fn test_val(graph_id: &str, graph: &MVCGraph) -> () {
    let g = graph.get_complement();
    let density = (2 * g.size()) as f64 / (g.order() * (g.order() - 1)) as f64;
    println!("Finding max clique of the graph. Specificity of the complement : \nOrder = {} and size = {}. Density = {}",
             g.order(),
             g.size(),
             density);

    let limit = 3600;
    let mut clock = Clock::new(limit);

    let res = solve_mvc(&g, &mut clock);
    clock.stop_timer();

    assert!(g.is_vertex_cover(&res.1));

    let clique_val = g.order() - res.1.len() as u64;


    let res = MVCResult::new(graph_id.to_string(), clique_val, res.1, clock.get_time(), clock.is_time_up(), true);

    output_reaction(res, &clock);
}

fn output_reaction(res: MVCResult, clock: &Clock) {
    println!("================ Result ===================\n{}", res);
    println!("======== Details about performance ========");
    println!("Time spent in deg : {}%", round(clock.deg_lb.as_secs_f64() * 100.0
                                                  / clock.get_time().duration.as_secs_f64(), 4));
    println!("Time spent in clq : {}%", round(clock.clq_lb.as_secs_f64() * 100.0
                                                  / clock.get_time().duration.as_secs_f64(), 4));
    println!("Time spent in max deg : {}%", round(clock.max_deg.as_secs_f64() * 100.0
                                                      / clock.get_time().duration.as_secs_f64(), 4));
    println!("Time spent in copy : {}%", round(clock.copy.as_secs_f64() * 100.0
                                                   / clock.get_time().duration.as_secs_f64(), 4));
    println!("Time spent in clq complement : {}%", round(clock.clq_compl.as_secs_f64() * 100.0
                                                             / clock.get_time().duration.as_secs_f64(), 4));
    println!("Time spent in color set : {}%", round(clock.color_set.as_secs_f64() * 100.0
                                                        / clock.get_time().duration.as_secs_f64(), 4));

    let comment = "Custom graph (without multithreading)";
    add_time_to_yaml(&res.graph_id,
                     res.value,
                     res.time,
                     res.is_time_limit,
                     "clique",
                     comment);
}

fn solve_mvc(graph: &MVCGraph, clock: &mut Clock) -> (u64, Vec<u64>) {
    // Initialize the upper bound to the number of nodes in the graph
    // and the vertex cover found so far is empty
    let upper_bound_vc = &graph.get_nodes();
    let u = bnb_mvc(graph, graph, graph.order(),
                    upper_bound_vc, vec![], clock);

    assert!(graph.is_vertex_cover(&u.1));
    u
}


fn bnb_mvc(graph: &MVCGraph,
               g: &MVCGraph,
               upper_bound: u64,
               upper_bound_vc: &Vec<u64>,
               vertex_cover: Vec<u64>,
               clock: &mut Clock) -> (u64, Vec<u64>) {
    if clock.is_time_up() {
        return (upper_bound, upper_bound_vc.clone());
    }

    clock.enter_copy();
    let mut subgraph = g.clone();
    clock.exit_copy();

    if subgraph.size() == 0 {
        // If the subgraph is empty, all edges are covered => vertex cover
        return (vertex_cover.len() as u64, vertex_cover);
    }

    clock.enter_max_deg();
    let (v, _max_deg) = get_vertex_with_max_degree(&subgraph, None);
    clock.exit_max_deg();

    clock.enter_deg();
    let deg_lb = deg_lb(&subgraph);
    clock.exit_deg();

    clock.enter_clq();
    let clq_lb = clq_lb(&subgraph);
    clock.exit_clq();

    let lb = max(deg_lb, clq_lb);


    if vertex_cover.len() as u64 + lb  >= upper_bound {
        // We can't find a better solution in this branch, we stop and return the best known solution
        return (upper_bound, upper_bound_vc.clone());
    }

    let neighbors: Vec<u64> = subgraph.get_neighbors(v).unwrap().clone();

    // ====> First case <====
    // - G \ {v}
    // - C U v
    let mut vertex_cover_case1 = vertex_cover.clone();

    // Removes v + edges from v to neighbor
    subgraph.remove_node(v);
    vertex_cover_case1.push(v);
    let res_case1 = bnb_mvc(graph,
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
            bnb_mvc(graph,
                    &subgraph,
                    res_case1.0,
                    &res_case1.1,
                    vertex_cover_case2, clock)
        } else {
            bnb_mvc(graph,
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

fn get_vertex_with_max_degree(graph: &MVCGraph, marked_vertices: Option<&Vec<u64>>) -> (u64, usize) {
    let mut max_degree = 0;
    let mut max_degree_vertex = 0;
    for vertex in graph.get_nodes() {
        if marked_vertices.is_some() && marked_vertices.unwrap().contains(&vertex) {
            continue;
        }
        let degree = graph.degree(vertex).unwrap();
        if degree > max_degree {
            max_degree = degree;
            max_degree_vertex = vertex;
        }
    }
    (max_degree_vertex, max_degree as usize)
}

fn deg_lb(graph: &MVCGraph) -> u64 {

    let size = graph.size();
    let mut selected_vertexes = Vec::<u64>::new();
    let mut sum_degrees: u64 = 0;

    let mut subgraph = graph.clone();

    let mut working = true;
    while working {
        // Get the vertex with the highest degree in the subgraph.
        let (max_degree_vertex, _vertex_degree) = get_vertex_with_max_degree(&subgraph, None);
        selected_vertexes.push(max_degree_vertex);
        sum_degrees += graph.degree(max_degree_vertex).unwrap();
        subgraph.remove_node(max_degree_vertex);
        if sum_degrees >= size {
            working = false;
        }
    }

    let edges_left = subgraph.size();
    let next_vertex = get_vertex_with_max_degree(&subgraph, None).0;
    if edges_left == 0 {
        selected_vertexes.len() as u64
    } else {
        let estim = (edges_left / graph.degree(next_vertex).unwrap()) as f64;
        (selected_vertexes.len() as f64 + estim).floor() as u64
    }
}

fn clq_lb(graph: &MVCGraph) -> u64 {
    // 1) Get the complement of the graph
    // 2) Find a greedy coloring of the complement
    // 3) Each color is a independent set
    // 4) An independent set in the complement is a clique in the original graph
    // 5) Adds the numbers of nodes in each clique minus 1 (a clique is a complete graph)

    // 1) Get the complement of the graph
    let compl = graph.get_complement();

    // 2) Find a greedy coloring of the complement
    let color_set = welch_powell(&compl);

    // Adds the number of nodes in each color minus 1 = lower bound. If a value is 0, change it to 1
    color_set.iter().map(|&x| x as u64 - 1).sum::<u64>()
}

fn welch_powell(graph: &MVCGraph) -> Vec<usize> {
    // sort vertices by decreasing degree
    let sorted_vertices = {
        let mut vertices: Vec<_> = graph.get_nodes();
        vertices.sort_by_key(|&i| std::cmp::Reverse(graph.degree(i).unwrap()));
        vertices
    };

    // create an array of colors such as color[i] = j means that vertex i has color j
    let mut color: Vec<i32> = {
        let mut color = Vec::new();
        for _ in 0..sorted_vertices.len() {
            color.push(-1);
        }
        color
    };
    // Array such as res[i] = j means that color j colored i vertices
    let mut res: Vec<usize> = Vec::new();

    let mut vertex_to_index = HashMap::new();
    for (i, vertex) in sorted_vertices.iter().enumerate() {
        vertex_to_index.insert(vertex, i);
    }

    let mut current_color = 0;

    loop {
        let mut biggest_index = -1;
        // Find the biggest vertex that is not colored
        for i in 0..sorted_vertices.len() {
            // vertex - 1 because vertex are numbered from 1 to n
            if color[i] == -1 {
                biggest_index = i as i32;
                break;
            }
        }
        if biggest_index == -1 {
            // All vertices are colored
            break;
        }
        // Color the biggest vertex
        color[biggest_index as usize] = current_color;
        res.push(1);

        assert_eq!(color.len(), sorted_vertices.len());
        // Color vertices that are not neighbors of vertex colored with current_color
        for i in 0..sorted_vertices.len() {
            if color[i] == -1 {
                let mut can_color = true;
                for neighbor in graph.get_neighbors(sorted_vertices[i]).unwrap() {
                    let index = *vertex_to_index.get(&neighbor).unwrap();
                    if color[index] == current_color {
                        // If a neighbor is already colored with current_color, we don't color the vertex
                        can_color = false;
                        break;
                    }
                }
                if can_color {
                    color[i] = current_color;
                    res[current_color as usize] += 1;
                }
            }
        }
        current_color += 1;
    }

    res
}