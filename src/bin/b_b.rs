use std::env;
use vertex;
use vertex::graph_utils::load_clq_file;
use vertex::branch_and_bound::solve;

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

            let res = solve(&graph);
            println!("Minimum vertex cover for the {:?} graph = {}", &args[1], res);
        }
        _ => {
            println!("Usage: cargo run [-r] --bin b_b <graph_name>");
        }
    }
}