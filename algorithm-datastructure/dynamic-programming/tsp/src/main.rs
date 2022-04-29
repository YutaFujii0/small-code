mod parse;

use crate::parse::*;

const FILE_PATH: &str = "dataset-sample.txt";

fn main() {
    println!("Traveling Salesman Problem!");

    if let Ok(res) = parse(FILE_PATH) {
        println!("{:?}", res);
    }
}
