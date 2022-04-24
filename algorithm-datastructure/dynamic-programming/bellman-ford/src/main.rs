mod bellman_ford;
mod errors;
mod graph;
mod parse;

use crate::bellman_ford::*;
use crate::graph::*;
use crate::parse::*;

const FILE_PATH: &str = "g1.txt";

fn main() {
    println!("Hello, world!");
    if let Ok(ret) = parse(FILE_PATH) {
        // println!("{:?}", ret);
        let graph = Graph::new(ret);
        println!("{:?}", graph.neighbors(1));
        println!("{:?}", graph.inverse_neighbors(1));
        bellman_ford(graph, 1);
    }
}
