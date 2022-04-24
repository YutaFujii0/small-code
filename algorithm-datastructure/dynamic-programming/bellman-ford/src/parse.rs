use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn parse<T>(path: T) -> Result<Vec<(usize, usize, i32)>, Box<dyn std::error::Error>>
where T: AsRef<Path>
{
    let mut ret = vec![];
    let file = File::open(path)?;
    for line in BufReader::new(file).lines() {
        let line = line?;
        let nodes = line.split(" ").collect::<Vec<&str>>();
        let from = nodes[0].parse::<usize>()?;
        let to = nodes[1].parse::<usize>()?;
        let cost = nodes[2].parse::<i32>()?;
        // let nodes = line
        //     .split(" ")
        //     .map(|num| num.parse::<i32>().unwrap())
        //     .collect::<Vec<i32>>();
        ret.push((from, to, cost));
    }
    Ok(ret)
}
