mod parse;
mod clustering;
use self::parse::*;
use self::clustering::*;

const FILE_PATH: &str = "./dataset-sample.txt";
const K_CLUSTERS: usize = 4;
// const FILE_PATH: &str = "./dataset.txt";

fn main() {
    println!("Clustering!");

    if let Ok(re) = parse(FILE_PATH) {
        println!("parsed! {:?}", re);
        cluster(re, K_CLUSTERS);
    }
}
