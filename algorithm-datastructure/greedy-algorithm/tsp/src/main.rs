mod parse;
mod error;
mod tsp;

use crate::parse::*;
use crate::tsp::*;

const FILE_PATH: &str = "dataset-sample.txt";

fn main() {
    println!("Traveling Salesman Problem with greedy algorithm!");
    if let Ok(ret) = parse(FILE_PATH) {
        // println!("{:?}", ret);
        println!("Shortest trip by greeding algorithm is {:?}", tsp(ret));
    }
}
