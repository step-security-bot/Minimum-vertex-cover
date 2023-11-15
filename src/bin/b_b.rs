use std::env;

use vertex;
use vertex::branch_and_bound::solve;
use vertex::graph_utils::load_clq_file;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            let graph = match load_clq_file(&format!("src/resources/graphs/{}", args[1])) {
                Ok(x) => x,
                Err(e) => {
                    println!("Error while loading graph {} \n {}", args[1] ,e);
                    return;
                },
            };
            vertex::run_algorithm(&args[1], &graph, &solve);
        }
        _ => {
            println!("Usage: cargo run [-r] --bin b_b <graph_name>");
        }
    }
}