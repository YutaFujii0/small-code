mod dp;
mod item;
mod parse;

use crate::dp::*;
use crate::parse::*;

const FILE_PATH: &str = "dataset.txt";
const KNAPSACK_SIZE: usize = 2000000;
// const FILE_PATH: &str = "dataset-sample.txt";
// const KNAPSACK_SIZE: usize = 10000;

fn main() {
    println!("Knapsack problem!");
    if let Ok(res) = parse(FILE_PATH) {
        // println!("{:?}", res);
        println!("Optimal value for this knapsack is: {:?}", dp(&res, KNAPSACK_SIZE));
    }
}
