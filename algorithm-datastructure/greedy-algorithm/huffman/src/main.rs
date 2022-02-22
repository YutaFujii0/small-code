mod conquer;
mod error;
mod huffman;
mod parse;

use crate::conquer::*;
use crate::parse::*;
use crate::huffman::*;

// const FILE_PATH: &str = "dataset-sample.txt";
const FILE_PATH: &str = "dataset.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Huffman's coding algorithm!");
    if let Ok(res) = parse(FILE_PATH) {
        let node = huffman(res)?;
        // println!("{:?}", node);
        println!("{:?}", min_max_depth(node, 0));
    }
    Ok(())
}
