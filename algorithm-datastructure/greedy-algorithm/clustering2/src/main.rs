mod dfs;
mod parse;
use crate::dfs::*;
use crate::parse::*;

// const FILE_PATH: &str = "dataset-sample.txt";
const FILE_PATH: &str = "dataset.txt";

fn main() {
    println!("clustering!");
    if let Ok(nodes) = parse(FILE_PATH) {
        let count = connected_components(nodes);
        println!("cluster needed {:?}", count);
    }
}
