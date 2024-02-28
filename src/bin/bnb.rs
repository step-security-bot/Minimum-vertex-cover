use std::env;

use vertex::{branch_and_bound, run_algorithm};
use vertex::graph_utils::load_clq_file;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        let graph = load_clq_file(&format!("src/resources/graphs/{}", args[1]))
            .expect("Error while loading graph");

        if args.len() == 3 && args[2] == "-c" {
            match run_algorithm(&args[1], &graph, &branch_and_bound, true) {
                Ok(res) => println!("Result : {}", res),
                Err(e) => println!("Error : {}", e),
            }
            return;
        }
        if args.len() == 3 {
            println!("Usage: cargo run [-r] --bin bnb <graph_name> [(on complement) -u]");
            return;
        }

        match run_algorithm(&args[1], &graph, &branch_and_bound, false) {
            Ok(res) => println!("Result : {}", res),
            Err(e) => println!("Error : {}", e),
        };
    } else {
        println!("Usage: cargo run [-r] --bin bnb <graph_name>");
    }
}