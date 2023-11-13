use vertex::branch_and_bound::solve;
use vertex::graph_utils::{load_clq_file, update_mvc_value};

fn main() {
    let path = "src/resources/graphs/test.clq";
    let graph = load_clq_file(path).unwrap();

    let res = solve(&graph);

    update_mvc_value("test.clq", res, Some("src/resources/graph_data.yml"));

}
