use std::fs::File;
use std::io::Read;
use std::time::Duration;

use serde_yaml::{Sequence, Value};

use vertex::ElapseTime;
use vertex::graph_utils::{add_time_to_yaml, GraphInfo};

fn main() {
    let time = ElapseTime {
        duration: Duration::new(0, 0),
        min: 0,
        sec: 0,
        ms: 0,
        micro: 0,
    };
    add_time_to_yaml("test.clq", 3, time, "BnB", "Yes i'm a comment");
}

#[allow(dead_code)]
fn create_yaml() {
    let data_path = "src/resources/graph_data.yml";
    let time_path = "src/resources/time_result.yml";

    let graph_file = File::open(data_path).expect("Could not open graph file");

    let data: Vec<GraphInfo> = serde_yaml::from_reader(graph_file).expect("Could not parse graph file");

    // Créer un mapping tel que key = grap_id et value = Vec<YamlTime>
    let mut file = File::open(time_path).expect("Could not open time file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Could not read time file");

    let time: Value = serde_yaml::from_str(&contents).expect("Could not parse time file");
    let map = time.as_mapping().expect("Could not parse time file");

    let mut map = map.clone();


    for info in data.iter() {
        // Pour chaque graph

        // Créer une entrée id - Vec<YamlTime>
        let vec: Sequence = Vec::new();

        map.insert(Value::String(info.id.clone()), Value::Sequence(vec));
    }

    let mut file = File::create(time_path).expect("Could not open time file");
    serde_yaml::to_writer(&mut file, &map).expect("Could not write time file");
}