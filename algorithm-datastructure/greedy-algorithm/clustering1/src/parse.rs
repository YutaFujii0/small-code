use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

pub fn parse<T>(path: T) -> Result<Vec<(usize, usize, usize)>, Box<dyn std::error::Error>>
where T: AsRef<Path>
{
    let file = File::open(path)?;
    let mut result = vec![];
    let re = Regex::new(r"(?P<node1>\d+)\s(?P<node2>\d+)\s(?P<distance>\d+)").expect("invalid regex");
    for line in BufReader::new(file).lines() {
        let line = line?;
        for caps in re.captures_iter(&line) {
            let node1 = caps["node1"].parse::<usize>()?;
            let node2 = caps["node2"].parse::<usize>()?;
            let distance = caps["distance"].parse::<usize>()?;
            result.push((node1, node2, distance));
        }
    }
    Ok(result)
}
