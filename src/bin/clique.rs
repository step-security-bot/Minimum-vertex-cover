use std::env;

use vertex;
use vertex::branch_and_bound::solve_clq;
use vertex::Clock;
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

        let res = solve_clq(&graph, &mut Clock::new(3600));
        println!("Result : {}", res.0);
    }
}