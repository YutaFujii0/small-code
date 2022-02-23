mod dp;
mod parse;

use crate::dp::*;
use crate::parse::*;

// const FILE_PATH: &str = "dataset-sample.txt";
const FILE_PATH: &str = "dataset.txt";

fn main() {
    println!("Maximum Independent Set!");
    if let Ok(res) = parse(FILE_PATH) {
        let s = dp(res);
        println!("{:?}", s);
    }
}
