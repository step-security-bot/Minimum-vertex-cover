use std::env;
use vertex;
use vertex::graph_utils::{get_optimal_value, is_optimal_value, load_clq_file};
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
            let is_opt = is_optimal_value(&args[1], res, None);
            if is_opt {
                println!("The value is optimal (as long as the data is correct in the yaml file)");
            } else {
                let true_opt = get_optimal_value(&args[1], None).unwrap_or(0);
                if true_opt == 0 {
                    println!("The correct value is unknown");
                } else {
                    println!("The value is not optimal and the correct value is {}", true_opt);
                }
            }
        }
        _ => {
            println!("Usage: cargo run [-r] --bin b_b <graph_name>");
        }
    }
}