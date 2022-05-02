use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn parse<T>(path: T) -> Result<Vec<(i32, i32)>, Box<dyn std::error::Error>>
where T: AsRef<Path>
{
    let mut ret = vec![];
    let file = File::open(path)?;
    for (index, line) in BufReader::new(file).lines().enumerate() {
        if index == 0 {
            continue;
        }
        let line = line?;
        let raw = line.split(" ")
            .map(|num| num.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        ret.push((raw[0], raw[1]));
    }

    Ok(ret)
}