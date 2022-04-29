mod combinations;
mod parse;
mod tsp;

use crate::parse::*;
use crate::tsp::*;

const FILE_PATH: &str = "dataset-sample.txt";

fn main() {
    println!("Traveling Salesman Problem!");

    if let Ok(res) = parse(FILE_PATH) {
        println!("{:?}", res);
        tsp(res);
    }
}
