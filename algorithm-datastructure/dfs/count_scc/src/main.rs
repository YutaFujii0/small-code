// answer top 5 strongly connected components
// psuedocode
// assign leader node for each vertex
// group by leader node and count then order them descendently

mod node;
mod dfs;
mod parser;

use dfs::Solution;
use parser::Parser;

// const EDGE_PATH: &str = "./dataset_simple.txt";
const EDGE_PATH: &str = "./dataset.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (nodes, nodes_rev) = Parser::new(EDGE_PATH).call()?;
    
    let finishing_times = Solution::dfs_loop_first(&nodes_rev)?;
    let leaders = Solution::dfs_loop_second(&nodes, &finishing_times)?;
    println!("leaders-nodes {:?}", leaders);

    Ok(())
}
