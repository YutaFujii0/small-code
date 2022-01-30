mod parse;

use crate::parse::*;

const FILE_PATH: &str = "dataset-sample.txt";

fn main() {
    println!("Huffman's coding algorithm!");
    if let Ok(res) = parse(FILE_PATH) {
        println!("hi");
    }
}
