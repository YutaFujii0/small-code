use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn parse<T>(path: T) -> Result<(), Box<dyn std::error::Error>>
where
    T: AsRef<Path>,
{
    let file = File::open(path)?;
    for line in BufReader::new(file).lines() {
        let line = line?;
        let bit_str = line.as_str().replace(" ", "");
        let node = u32::from_str_radix(bit_str.as_str(), 2)?;

        println!("{:?}", node);
    }

    Ok(())
}
