use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead, Lines, BufReader};
use regex::Regex;
use std::rc::Rc;
use std::cell::RefCell;
use rand::Rng;

const PATH: &str = "./dataset-sample.txt";
const VERTICES_SIZE: usize = 4;
type Vertex = Rc<u8>;
type VertexPtr = Rc<RefCell<Rc<u8>>>;
type Edge = (VertexPtr, VertexPtr);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Randomized Contraction!");
    let mut edges: Vec<Edge> = vec![];
    let vertices: Vec<Vertex> = (1..VERTICES_SIZE+1).map(|i| Rc::new(i as u8)).collect();
    let vertex_ptrs: Vec<VertexPtr> = (0..VERTICES_SIZE)
        .map(|i| Rc::new(RefCell::new(Rc::clone(&vertices[i]))))
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
            // println!("{:?}", edges);
        }
    }
    let mut vertices_size = vertices.len();
    let mut rng = rand::thread_rng();
    while vertices_size > 2 {
        let (left, right) = edges.remove(rng.gen_range(0..edges.len()));
        if left != right {
            *right.borrow_mut() = Rc::clone(&*left.borrow());
            vertices_size -= 1;
        }
    }
    let min_cut = edges
        .iter()
        .filter(|&(left, right)| left != right)
        .collect::<Vec<&Edge>>()
        .len();
    println!("min cut: {:?}", min_cut / 2);
    Ok(())
}

fn parse<T>(path: T) -> io::Result<Lines<BufReader<File>>>
where T: AsRef<Path>
{
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}
