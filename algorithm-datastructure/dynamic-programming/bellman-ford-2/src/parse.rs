use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use super::bellmanford::{VertexValue, Cost, RawData};

pub fn parse<T>(path: T) -> Result<RawData, Box<dyn std::error::Error>>
where T: AsRef<Path>
{
    let mut ret = vec![];
    let file = File::open(path)?;
    for line in BufReader::new(file).lines() {
        let line = line?;
        let nodes = line.split(" ").collect::<Vec<&str>>();
        let from = nodes[0].parse::<VertexValue>()?;
        let to = nodes[1].parse::<VertexValue>()?;
        let cost = nodes[2].parse::<Cost>()?;

        ret.push((from, to, cost));
    }
    Ok(ret)
}
