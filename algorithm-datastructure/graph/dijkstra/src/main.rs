mod parse;
mod graph;

use parse::*;
use graph::*;

const FILEPATH: &str = "./dataset.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let edges: Vec<(usize, usize, usize)>;
    if let Ok(lines) = read_lines(FILEPATH) {
        edges = parse(lines)?;
        let graph = Graph::new(edges);
        graph.dijstra();
    }

    Ok(())
}


// each time scan m edges
// if tail of e in X and head of e not in X
//  to do that hashmap<node_id, min length or none>
// calcurate greedy dijkstra's score
//   w = A[v] + lvw
// max w
// store that to hashmap<node_id, length>
// 
