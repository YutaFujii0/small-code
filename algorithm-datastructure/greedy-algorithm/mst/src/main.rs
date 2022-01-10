mod parse;
mod mst;
use self::parse::*;
use self::mst::*;

// const FILE_PATH: &str = "dataset-sample.txt";
const FILE_PATH: &str = "dataset.txt";

fn main() {
    println!("Hello, world!");
    if let Ok(re) = parse(FILE_PATH) {
        let graph = graphize(re);
        let cost = mst(graph);
        if let Ok(cost) = cost {
            println!("{:?}", cost);
        }
    }
}
