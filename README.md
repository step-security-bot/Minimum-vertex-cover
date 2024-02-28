# Minimum-vertex-cover
Comparison of algorithms (exact and heuristic) for the minimum vertex cover problem

Documentation can be found [here](https://licornerose765.github.io/Minimum-vertex-cover/)

## Algorithms
### Exact algorithms
* Naive method : iterate over all possible subsets of vertices and check if it is a vertex cover.  
use : `cargo run -r --bin naive_search <file_name>`
* Branch and bound : Algorithm based on the paper presented by Wang, Luzhi, Shuli Hu, Mingyang Li, and Junping Zhou 
[Source](https://doi.org/10.3390/math7070603)  
use : `cargo run -r --bin bnb <file_name> [-c]`

### Heuristic algorithms


## Bins 
* `naive_method` : naive method  
use : `cargo run -r --bin naive_search <file_name>`
* `add_graph_to_yaml`: Update the graph information in the yaml file (get the graphs in the resources/graphs folder)  
use : `cargo run -r --bin add_graph_to_yaml`
* `bnb` : Find the MVC of the graph (or the complement if -c is added) using the branch and bound algorithm.  
use : `cargo run -r --bin bnb <file_name> [-c]`
* `clique` : Find the value of the maximum clique of the graph by find the MVC of the complement using the BnB algorithm.  
use : `cargo run -r --bin clique <file_name>`