use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn parse<T>(path: T) -> Result<Vec<String>, Box<dyn std::error::Error>>
where
    T: AsRef<Path>,
{
    let file = File::open(path)?;
    let mut results = vec![];
    for line in BufReader::new(file).lines() {
        let line = line?;
        results.push(line.as_str().replace(" ", ""));
    }

    Ok(results)
}
