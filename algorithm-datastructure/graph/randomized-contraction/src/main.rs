use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead, Lines, BufReader};
use regex::Regex;
use std::rc::Rc;

const PATH: &str = "./dataset-sample.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Randomized Contraction!");
    let mut edges = vec![];
    let vertices: Vec<Rc<u8>> = (1..201).map(|i| Rc::new(i)).collect();
    let vertex_ptrs: Vec<Rc<Rc<u8>>> = (0..200usize)
        .map(|i| Rc::new(Rc::clone(&vertices[i])))
        .collect();
    let lines = parse(PATH)?;
    let re = Regex::new(r"(?P<vertex>\d+)").expect("Invalid Regex");
    for line in lines {
        if let Ok(line) = line {
            // println!("{:?}", line);
            let mut tmp_vals = vec![];
            for caps in re.captures_iter(&line) {
                let vertex = caps["vertex"].parse::<usize>()?;
                tmp_vals.push(vertex);
            }
            for i in 1..tmp_vals.len() {
                edges.push(
                    (Rc::clone(&vertex_ptrs[tmp_vals[0]-1]), Rc::clone(&vertex_ptrs[tmp_vals[i]-1]))
                );
            }
            println!("{:?}", edges);
        }
    }
    Ok(())
}

fn parse<T>(path: T) -> io::Result<Lines<BufReader<File>>>
where T: AsRef<Path>
{
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}
