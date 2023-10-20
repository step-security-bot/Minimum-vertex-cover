use std::fs::{File, read_dir};
use std::io::Write;
use graph::{Graph, GraphNauty};
use serde::{Deserialize, Serialize};
use vertex::graph_utils::load_clq_file;

pub fn update_graph_info() {
    let paths = match read_dir("src/resources/graphs") {
        Ok(x) => x,
        Err(e) => {
            println!("Error while reading directory : {}", e);
            return;
        },
    };

    for path in paths {
        let path = path.unwrap().path();
        let path_str = path.to_str().unwrap();

        if path_str.ends_with(".col") || path_str.ends_with(".clq") {
            let graph = match load_clq_file(path_str) {
                Ok(x) => x,
                Err(e) => {
                    println!("Error while loading graph at {:?} : {}",path_str, e);
                    return;
                },
            };
            println!("{}: {} vertices, {} edges", path_str, graph.order(), graph.size());
            add_graph_to_yaml(path_str.split("/").last().unwrap(), "clq", &graph);
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct GraphInfo {
    id: String,
    format: String,
    order: u64,
    size: u64,
    mvc_val: u64,
}

pub fn add_graph_to_yaml(id: &str, format: &str, graph: &GraphNauty) {
    let yaml_path = "src/resources/graph_data.yml";
    let file = File::open(yaml_path).expect(format!("Unable to open file {:?}", yaml_path).as_str());
    let mut data: Vec<GraphInfo> = serde_yaml::from_reader(file).unwrap();

    if data.iter().any(|x| x.id == id) {
        // If the graph is already in the file, we don't add it again
        return;
    }

    let info = GraphInfo {
        id: id.to_string(),
        format: format.to_string(),
        order: graph.order(),
        size: graph.size(),
        mvc_val: 0,
    };
    data.push(info);

    // Update the file
    let mut file = File::create(yaml_path).expect(format!("Unable to create file {:?}", yaml_path).as_str());
    file.write_all(serde_yaml::to_string(&data).unwrap().as_bytes()).expect(format!("Unable to write file to {:?}", yaml_path).as_str());
}


fn main() {
    update_graph_info();
}