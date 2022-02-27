mod optimal_bst;
mod parse;

use crate::optimal_bst::*;
use crate::parse::*;

const FILE_PATH: &str = "dataset-sample.txt";

fn main() {
    println!("Optimal BST!");
    if let Ok(res) = parse(FILE_PATH) {
        println!("{:?}", res);
        println!("{:?}", optimal_bst(res));
    }
}
