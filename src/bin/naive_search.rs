use std::env;

use vertex;
use vertex::graph_utils::{add_time_to_yaml, load_clq_file};
use vertex::naive_search;

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
        if args.len() == 3 && args[2] == "-u" {
            // Update value
            let res = vertex::run_algorithm(&args[1], &graph, &naive_search);
            println!("Result : {}", res);
            add_time_to_yaml(&res.graph_id, res.value, res.time, "BnB", "");
            return;
        }
        if args.len() == 3 && args[2] != "-u"{
            println!("Usage: cargo run [-r] --bin naive_search <graph_name> [(do_update_val) -u]");
            return;
        }
        // Run algorithm without updating value
        let res = vertex::run_algorithm(&args[1], &graph, &naive_search);
        println!("Result : {}", res);
        add_time_to_yaml(&res.graph_id, res.value, res.time, "BnB", "");
    } else {
        println!("Usage: cargo run [-r] --bin naive_search <graph_name> [(do_update_val) -u]");
    }

}