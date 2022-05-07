mod error;
mod node;
mod parser;
mod scc;

use std::error::Error;

use parser::Parser;
use scc::*;

const FILE_PATH: &str = "dataset-sample.txt";

fn main() -> Result<(), Box<dyn Error>> {
    println!("2SAT Problem using strongly connected components!");
    if let Ok(ret) = Parser::run(FILE_PATH) {
        let (data_count, (nodes, nodes_rev)) = ret;
        let finishing_times = Solution::dfs_loop_first(&nodes_rev)?;
        let leaders = Solution::dfs_loop_second(&nodes, &finishing_times)?;

        for i in 1..(data_count+1) {
            println!("{} {:?}", i, leaders.get(&(i*2)).unwrap_or(&0) != leaders.get(&(i*2-1)).unwrap_or(&1));
        }
    }
    Ok(())
}
