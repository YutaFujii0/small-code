use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

use datastructures::HeapNode;

const FILE_PATH: &str = "./dataset.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Median Maintenance!");

    let mut median_sum = 0;
    let mut smallers = HeapNode::<i32, _>::new(out_of_order_smallers);
    let mut largers = HeapNode::<i32, _>::new(out_of_order_largers);
    let lines = read_lines(FILE_PATH)?;
    for line_result in lines {
        if let Ok(line) = line_result {
            let val = line.parse::<i32>();
            if let Ok(value) = val {
                match smallers.peak() {
                    None => smallers.insert(value),
                    Some(head) => {
                        if head < &value {
                            largers.insert(value);
                        } else {
                            smallers.insert(value);
                        }
                    }
                }
                while smallers.size < largers.size {
                    smallers.insert(largers.poll().unwrap());
                }
                while smallers.size > largers.size + 1 {
                    largers.insert(smallers.poll().unwrap());
                }
                median_sum += smallers.peak().unwrap();
            }
        }
    }
    println!("Iteration finished. Module 10000 of the sum of medians is {:?}", median_sum % 10000);
    Ok(())
}

fn out_of_order_smallers(parent: &i32, child: &i32) -> bool {
    parent < child
}

fn out_of_order_largers(parent: &i32, child: &i32) -> bool {
    parent > child
}

fn read_lines<P>(path: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>
{
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

