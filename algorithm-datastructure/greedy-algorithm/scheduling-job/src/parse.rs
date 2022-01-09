use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
use std::collections::VecDeque;

pub fn parse<T>(path: T) -> Result<VecDeque::<(u64, u64)>, Box<dyn std::error::Error>>
where T: AsRef<Path>
{
    let mut results: VecDeque<(u64, u64)> = VecDeque::new();
    let file = File::open(path)?;
    let re = Regex::new(r"(?P<weight>\d+)\s(?P<length>\d+)").expect("invalid regex");
    for line in BufReader::new(file).lines() {
        if let Ok(line) = line {
            for cap in re.captures_iter(&line) {
                let weight = cap["weight"].parse::<u64>()?;
                let length = cap["length"].parse::<u64>()?;
                results.push_back((weight, length));
            }
        }
    }
    Ok(results)
}
