use petgraph::prelude::UnGraphMap;

use vertex::branch_and_bound::solve;
use vertex::graph_utils::{is_vertex_cover, load_clq_file};
use vertex::run_algorithm;

fn main() {
    let graph_list: Vec<&str> = vec!["brock200_2.clq", "brock200_4.clq", "brock400_2.clq",
                                     "brock400_4.clq", "brock800_4.clq"];

    for graph_id in graph_list {
        let graph = match load_clq_file(&format!("src/resources/graphs/{}", graph_id)) {
            Ok(x) => x,
            Err(e) => {
                println!("Error while loading graph : {}", e);
                return;
            }
        };
        println!("=========== {} ===========", graph_id);
        let res = run_algorithm(graph_id, &graph, &solve, false);
        println!("Result : {}", res);
    }
}

fn _naive(graph: &Box<UnGraphMap<u64, ()>>, features: &mut Vec<u64>, max_length: u64) -> () {
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