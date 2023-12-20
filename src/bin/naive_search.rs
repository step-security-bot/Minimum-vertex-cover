use std::env;

use vertex;
use vertex::graph_utils::load_clq_file;
use vertex::naive_search;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        let graph = load_clq_file(&format!("src/resources/graphs/{}", args[1]))
            .expect("Error while loading graph");

        // Run algorithm without updating value
        println!("/!\\ This algorithm compute the MVC value on the complement graph by default /!\\");
        let res = vertex::run_algorithm(&args[1], &graph, &naive_search, true);
        println!("Result : {}", res);
    } else {
        println!("Usage: cargo run [-r] --bin naive_search <graph_name>");
    }
}