use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;
use std::error::Error;

pub fn read_lines<P>(path: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>
{
    let file = File::open(path)?;

    Ok(io::BufReader::new(file).lines())
}

pub fn parse<T>(lines: io::Lines<T>) -> Result<(), Box<dyn Error>>
where T: BufRead
{
    let re = Regex::new(r"(?m)(?P<to>\d+),(?P<len>\d+)").expect("Invalid Regex");
    for line in lines {
        if let Ok(li) = line {
            let tail = Regex::new(r"(?m)\d+")
                .unwrap()
                .find(&li)
                .unwrap()
                .as_str()
                .parse::<usize>()?;
            for caps in re.captures_iter(&li) {
                let node = caps["to"].parse::<usize>()?;
                let length = caps["len"].parse::<usize>()?;
                println!("node {:?}-{:?}, len {:?}", tail, node, length);
            }
        }
    }
    Ok(())
}

#[allow(dead_code)]
struct Edge {
    nodes: (usize, usize),
    length: usize,
}
