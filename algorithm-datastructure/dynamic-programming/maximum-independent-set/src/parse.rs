use std::path::Path;
use std::io::{BufRead, BufReader};
use std::fs::File;

pub fn parse<T>(path: T) -> Result<Vec<u32>, Box<dyn std::error::Error>>
where T: AsRef<Path>
{
    let mut ans = vec![];
    let file = File::open(path)?;
    for line in BufReader::new(file).lines() {
        let line = line?;
        let num = line.parse::<u32>()?;
        ans.push(num);
    }
    Ok(ans)
}