mod clustering;
mod error;
mod parse;

use std::env;
use std::result::Result;
use std::error::Error;
use self::clustering::*;
use self::parse::*;

const ERR_MSG: &str = "Usage: target/debug/clustering1 <dataset-path> <k_cluster_size>";

fn main() -> Result<(), Box<dyn Error>> {
    println!("Clustering!");

    let path = env::args().nth(1).ok_or_else(|| ERR_MSG)?;
    let k_clusters = env::args().nth(2)
        .ok_or_else(|| ERR_MSG)?
        .parse::<usize>()?;

    if let Ok(result) = parse(path) {
        let graph = preprocess(result);
        let _ = cluster(graph, k_clusters);
    }
    Ok(())
}
