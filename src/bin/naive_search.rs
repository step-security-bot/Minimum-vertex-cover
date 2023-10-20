use vertex; // Don't forget to add XD

use vertex::graph_utils::load_col_file;
use vertex::algorithms::naive_search;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        3 => {
            let graph = match load_col_file(&format!("src/resources/particular_graphs/{}",args[2])) {
                Ok(x) => x,
                Err(e) => panic!("{}", e),
            };
            let res = naive_search(&graph).expect("Error in naive_search");
            println!("Minimum vertex cover for the {:?} graph = {} => {:?}",&args[2], res.len(), res);
        }
        _ => {
            println!("{} args found : {:?}", args.len(), args);
            vertex::hello_world();
        }
    }
}