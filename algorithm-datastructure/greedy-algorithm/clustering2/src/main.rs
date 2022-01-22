mod parse;
use crate::parse::*;

const FILE_PATH: &str = "dataset-sample.txt";

fn main() {
    println!("clustering!");
    parse(FILE_PATH);
}
