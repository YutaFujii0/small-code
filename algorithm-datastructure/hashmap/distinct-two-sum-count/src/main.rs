use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Distinct two sum count!");
    const FILE_PATH: &str = "dataset.txt";
    let nums = parse(FILE_PATH)?;
    let mut one_hits = 0;
    // println!("{:?}", nums);
    for k in -10000i64..10001i64 {
        let mut sub_sum = HashMap::new();
        for num in &nums {
            sub_sum.insert(k - num, 0);
        }
        let mut hits = 0;
        for num in &nums {
            if sub_sum.contains_key(&num) {
                // println!("hit! {}", num);
                hits += 1;
            }
            if hits > 2 {
                break;
            }
        }
        if hits == 2 {
            one_hits += 1;
        }
    }
    println!("t such that has distinct pair of numbers are {}", one_hits);
    Ok(())
}

fn parse<T>(path: T) -> Result<Vec::<i64>, Box<dyn std::error::Error>>
where T: AsRef<Path>
{
    let file = File::open(path)?;
    let mut nums = Vec::<i64>::new();
    for line_result in io::BufReader::new(file).lines() {
        let line_result = line_result?;
        let val = line_result.parse::<i64>()?;
        nums.push(val);
    }
    Ok(nums)
}
