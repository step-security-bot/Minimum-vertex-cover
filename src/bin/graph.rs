use std::fs::File;
use std::io::{BufRead, BufReader};
use petgraph::prelude::UnGraphMap;
use vertex::branch_and_bound::solve;
use vertex::run_algorithm;

fn main() {

    let g = read_file();
    let res = run_algorithm("karate.graph", &g, &solve);
    println!("Result : {}", res);
}


pub fn read_file() -> UnGraphMap<u64, ()> {
    let file = File::open("src/resources/karate.graph").expect("");
    let reader = BufReader::new(file);

    let mut g = UnGraphMap::<u64, ()>::new();

    let mut i =0;
    for line in reader.lines() {
        i+=1;
        let line = line.expect("");
        let values: Vec<&str> = line.split_whitespace().collect();
        if i == 1 {
            let order = values[0].parse::<u64>().expect("");
            for j in 0..order {
                g.add_node(j);
            }
        } else {
            for value in values {
                let j = value.parse::<u64>().expect("");
                g.add_edge(i-2, j-1, ());
            }
        }
    }
    println!("Graph with {} vertex and {} edges", g.node_count(), g.edge_count());
    g
}
