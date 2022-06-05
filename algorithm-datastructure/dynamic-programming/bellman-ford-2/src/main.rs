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
    

    let mut local_mins = vec![];
    if let Ok(res) = parse(path) {
        let n = &res.iter()
            .map(|item| item.0)
            .max()
            .ok_or_else(|| "Unable to find max index of node from argument.")?;

        for source_index in 0..*n {
            if let Ok((result, no_negative_cycle)) = Solution::call(&res, source_index) {
                if no_negative_cycle {
                    let min = result.into_iter().min().unwrap();
                    local_mins.push(min);
                } else {
                    println!("Negative cost cycle detected! Stop iterating...");
                    break
                }
            }
        }
        println!("shortest shortest path is {:?}", local_mins.iter().min().unwrap_or_else(|| &i64::MAX))
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