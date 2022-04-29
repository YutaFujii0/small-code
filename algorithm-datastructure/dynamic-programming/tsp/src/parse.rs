use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn parse<T>(path: T) -> Result<Vec<(f32, f32)>, Box<dyn std::error::Error>>
where T: AsRef<Path>
{
    let mut res = vec![];
    let file = File::open(path)?;
    for line in BufReader::new(file).lines() {
        let line = line?;
        let item = line.split(" ")
            .map(|num| num.parse::<f32>().unwrap())
            .collect::<Vec<f32>>();
        res.push((item[0], item[1]));
    }
    Ok(res)
}
