use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use super::error::ParseError;

pub fn parse<T>(path: T) -> Result<Vec<(usize, f32, f32)>, Box<dyn std::error::Error>>
where T: AsRef<Path>
{
    let mut ret = vec![];
    let file = File::open(path)?;
    for line in BufReader::new(file).lines() {
        let line = line?;
        let rawdata = line.split(" ")
            .collect::<Vec<&str>>();
        ret.push((
            rawdata[0].parse::<usize>().or(Err(ParseError))?,
            rawdata[1].parse::<f32>().or(Err(ParseError))?,
            rawdata[2].parse::<f32>().or(Err(ParseError))?
        ))
    }
    Ok(ret)
}
