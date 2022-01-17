mod parse;
mod clustering;
mod error;

use self::parse::*;
use self::clustering::*;

// const FILE_PATH: &str = "./dataset-sample.txt";
const FILE_PATH: &str = "./dataset.txt";
const K_CLUSTERS: usize = 4;

fn main() {
    println!("Clustering!");

    if let Ok(result) = parse(FILE_PATH) {
        let graph = preprocess(result);
        // println!("parsed! {:?}", graph);
        let _ = cluster(graph, K_CLUSTERS);
    }
}
