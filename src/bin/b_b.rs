use std::env;

use vertex;
use vertex::branch_and_bound::solve;
use vertex::graph_utils::load_clq_file;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        let graph = match load_clq_file(&format!("src/resources/graphs/{}", args[1])) {
            Ok(x) => x,
            Err(e) => {
                println!("Error while loading graph : {}", e);
                return;
            }
        };
        // Run algorithm without updating value
        println!("/!\\ This algorithm compute the MVC value on the complement graph by default /!\\");
        let res = vertex::run_algorithm(&args[1], &graph, &solve, true);
        println!("Result : {}", res);
        // add_time_to_yaml(&res.graph_id, res.value, res.time, "BnB", "DegLB + ClqLB & BnB with only one graph copy");
    } else {
        println!("Usage: cargo run [-r] --bin b_b <graph_name>");
    }
}