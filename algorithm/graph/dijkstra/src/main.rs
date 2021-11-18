mod parse;

use parse::*;

const FILEPATH: &str = "./dataset.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    if let Ok(lines) = read_lines(FILEPATH) {
        parse(lines)?;
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
