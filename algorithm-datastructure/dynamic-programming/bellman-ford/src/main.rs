mod bellman_ford;
mod errors;
mod graph;
mod parse;

use crate::bellman_ford::*;
use crate::graph::*;
use crate::parse::*;

const FILE_PATH: &str = "g3.txt";

fn main() {
    println!("Bellman Ford Algorithm!");
    if let Ok(ret) = parse(FILE_PATH) {
        // println!("{:?}", ret);
        let graph = Graph::new(ret);
        // println!("{:?}", graph.neighbors(1));
        // println!("{:?}", graph.inverse_neighbors(1));
        // println!("{:?}", bellman_ford(graph, 0));
        let mut local_mins = vec![];
        for source_index in 0..graph.nodes {
            let (result, no_negative_cycle) = bellman_ford(&graph, source_index);
            if no_negative_cycle {
                let min = result.into_iter().min().unwrap();
                local_mins.push(min);
            } else {
                println!("Negative cost cycle detected! Stop iterating...");
                break
            }
        }
        println!("shortest shortest path is {:?}", local_mins.iter().min().unwrap_or_else(|| &i32::MAX))
    }
}
