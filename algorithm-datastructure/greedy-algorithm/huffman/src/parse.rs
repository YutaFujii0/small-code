use datastructures::HeapNode;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn parse<T, F>(path: T) -> Result<HeapNode<u32, F>, Box<dyn std::error::Error>>
where
    T: AsRef<Path>,
    F: Fn(&u32, &u32) -> bool,
{
    let file = File::open(path)?;
    let mut heap = HeapNode::<u32, F>::new(out_of_order());
    for line in BufReader::new(file).lines() {
        let line = line?;
        let weight = line.parse::<u32>()?;
        heap.insert(weight);
    }

    Ok(heap)
}

fn out_of_order() -> impl Fn(&u32, &u32) -> bool {
    |left, right| -> bool { right < left }
}
