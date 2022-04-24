mod parse;
use crate::parse::*;

const FILE_PATH: &str = "g1.txt";

fn main() {
    println!("Hello, world!");
    if let Ok(ret) = parse(FILE_PATH) {
        println!("{:?}", ret);
    }
}
