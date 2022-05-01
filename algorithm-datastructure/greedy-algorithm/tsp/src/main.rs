mod parse;
mod error;

use crate::parse::*;

const FILE_PATH: &str = "dataset-sample.txt";

fn main() {
    println!("Traveling Salesman Problem with greedy algorithm!");
    if let Ok(ret) = parse(FILE_PATH) {
        println!("{:?}", ret);
    }
}
