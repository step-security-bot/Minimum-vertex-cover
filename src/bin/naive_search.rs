use std::env;

use vertex;
use vertex::graph_utils::load_clq_file;
use vertex::naive_search;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            let graph = match load_clq_file(&format!("src/resources/graphs/{}", args[1])) {
                Ok(x) => x,
                Err(e) => {
                    println!("Error while loading graph : {}", e);
                    return;
                },
            };
            vertex::run_algorithm(&args[1], &graph, &naive_search);
        }
        _ => {
            println!("Usage: cargo run [-r] --bin naive_search <graph_name>");
        }
    }
}