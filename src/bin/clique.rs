use std::env;

use petgraph::prelude::UnGraphMap;
use round::round;

use vertex;
use vertex::{Clock, MVCResult};
use vertex::branch_and_bound::solve;
use vertex::graph_utils::{add_time_to_yaml, complement, is_vertex_cover, load_clq_file};

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

        find_max_clique(&args[1], &graph);
    }
}



fn find_max_clique(graph_id: &str, graph: &Box<UnGraphMap<u64, ()>>) -> () {
    let g = complement(graph);let density = (2 * g.edge_count()) as f64 / (g.node_count() * (g.node_count() - 1)) as f64;
    println!("Finding max clique of the graph. Specificity of the complement : \nOrder = {} and size = {}. Density = {}",
             g.node_count(),
             g.edge_count(),
             density);

    let limit = 3600;
    let mut clock = Clock::new(limit);

    let res = solve(&g, &mut clock);
    clock.stop_timer();

    assert!(is_vertex_cover(&g, &res.1));


    let res = MVCResult::new(graph_id.to_string(), res.0, res.1, clock.get_time(), clock.is_time_up(), true);

    output_reaction(res, &clock);

}



fn output_reaction(res: MVCResult, clock: &Clock) {
    println!("================ Result ===================\n{}", res);
    println!("======== Details about performance ========");
    println!("Time spent in deg : {}%", round(clock.deg_lb.as_secs_f64() * 100.0
        / clock.get_time().duration.as_secs_f64(), 4));
    println!("Time spent in clq : {}%", round(clock.clq_lb.as_secs_f64() * 100.0
            / clock.get_time().duration.as_secs_f64(), 4));
    println!("Time spent in max deg : {}%", round(clock.max_deg.as_secs_f64() * 100.0
                / clock.get_time().duration.as_secs_f64(), 4));
    println!("Time spent in copy : {}%", round(clock.copy.as_secs_f64() * 100.0
                    / clock.get_time().duration.as_secs_f64(), 4));
    println!("Time spent in clq complement : {}%", round(clock.clq_compl.as_secs_f64() * 100.0
                        / clock.get_time().duration.as_secs_f64(), 4));
    println!("Time spent in color set : {}%", round(clock.color_set.as_secs_f64() * 100.0
                            / clock.get_time().duration.as_secs_f64(), 4));

    /*
    let comment = "Reversed recursive calls (2 copy)";
    add_time_to_yaml(&res.graph_id,
                     res.value,
                     res.time,
                     res.is_time_limit,
                     "clique",
                     comment);
     */

}