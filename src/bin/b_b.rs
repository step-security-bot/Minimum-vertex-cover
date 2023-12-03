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
        if args.len() == 3 && args[2] == "-c" {
            let res = vertex::run_algorithm(&args[1], &graph, &solve, true);
            println!("Result : {}", res);
            return;
        }
        if args.len() == 3 && args[2] != "-u" {
            println!("Usage: cargo run [-r] --bin b_b <graph_name> [(on complement) -u]");
            return;
        }

        let res = vertex::run_algorithm(&args[1], &graph, &solve, false);
        println!("Result : {}", res);
    } else {
        println!("Usage: cargo run [-r] --bin b_b <graph_name>");
    }
}