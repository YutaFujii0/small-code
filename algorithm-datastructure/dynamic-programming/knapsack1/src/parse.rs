use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};

use super::item::Item;

pub fn parse<T>(path: T) -> Result<Vec<Item>, Box<dyn std::error::Error>>
where T: AsRef<Path>
{
    let file = File::open(path)?;
    let mut ans = vec![];
    for line in BufReader::new(file).lines() {
        let line = line?;
        let raw = line.split(' ').collect::<Vec<&str>>();
        let value = raw[0].parse::<u32>()?;
        let weight = raw[1].parse::<usize>()?;
        ans.push(Item { value, weight });
    }
    Ok(ans)
}