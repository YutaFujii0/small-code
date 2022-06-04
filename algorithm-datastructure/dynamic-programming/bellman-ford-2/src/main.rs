use std::fmt;
use std::env;
use std::error::Error;

mod bellmanford;
mod parse;

use parse::*;
use bellmanford::Solution;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Bellman Ford algorithm!");
    let path = env::args()
        .nth(1)
        .ok_or(NoArgumentError)?;
    

    if let Ok(res) = parse(path) {
        println!("{:?}", res);
        Solution::call(res);
    }
    Ok(())
}

#[derive(Debug)]
struct NoArgumentError;

impl fmt::Display for NoArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No argument given.")
    }
}

impl Error for NoArgumentError {}