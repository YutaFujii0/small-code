mod parse;
mod graph;

use parse::*;
use graph::*;

const FILEPATH: &str = "./dataset.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Shortest path length calculating!");

    let nodes: Vec<(usize, Vec<(usize, usize)>)>;
    if let Ok(lines) = read_lines(FILEPATH) {
        nodes = parse(lines)?;
        let graph = Graph::new(nodes);
        graph.dijstra();
    }

    Ok(())
}
