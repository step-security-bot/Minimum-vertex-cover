use vertex;
use vertex::graph_utils::load_clq_file;
use vertex::branch_and_bound::solve;

fn main() {
    let graph = match load_clq_file("src/resources/graphs/test.clq") {
        Ok(x) => x,
        Err(e) => {
            println!("Error while loading graph : {}", e);
            return;
        },
    };
    let res = solve(&graph);
    println!("Minimum vertex cover for the test.clq graph = {}", res);
}