mod parse;

use crate::parse::*;

const FILE_PATH: &str = "2sat1.txt";

fn main() {
    println!("2SAT Problem!");
    if let Ok(ret) = parse(FILE_PATH) {
        println!("{:?}", ret);
    }
}
