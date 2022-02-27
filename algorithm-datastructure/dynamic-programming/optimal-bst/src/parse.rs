use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn parse<T>(path: T) -> Result<Vec<f32>, Box<dyn std::error::Error>>
where T: AsRef<Path>
{
    let file = File::open(path)?;
    let res = BufReader::new(file).lines()
        .map(|line| line.unwrap())
        .map(|line| line.parse::<f32>().unwrap())
        .collect::<Vec<f32>>();
    Ok(res)
}
