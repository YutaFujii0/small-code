mod error;
mod huffman;
mod parse;

use crate::parse::*;
use crate::huffman::*;

const FILE_PATH: &str = "dataset-sample.txt";

fn main() {
    println!("Huffman's coding algorithm!");
    if let Ok(res) = parse(FILE_PATH) {
        println!("hi, {:?}", res);
        huffman(res);
    }
}
