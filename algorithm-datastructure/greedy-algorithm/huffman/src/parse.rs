use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

// use super::error::ArgumentError;

pub fn parse<T>(path: T) -> Result<Vec<u32>, Box<dyn std::error::Error>>
where T: AsRef<Path>
{
    let file = File::open(path)?;
    let weights = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    Ok(weights)
}
