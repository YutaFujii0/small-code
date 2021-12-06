use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

// 10000 to 10000
// -1 to 1
// x = 1 - y
// x = 1.1 thdn y can be -0.1,-0.3,-0.5 ... to -1.1
// if floor x, and push 1 - x into key, lookup should be ceil of y

// O(n) * O(k) -> O(10^10)

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Distinct two sum count!");
    const FILE_PATH: &str = "dataset.txt";
    let nums = parse(FILE_PATH)?;

    let mut sub_sum = HashMap::new();
    for (index, num) in nums.iter().enumerate() {
        let nega = (-1 * num) / 10000;
        let value = sub_sum.entry(nega).or_insert(vec![]);
        value.push((num, index));
    }
    let mut hits = HashMap::new();
    for (index, num) in nums.iter().enumerate() {
        let target = num / 10000;
        if let Some(value) = sub_sum.get(&target) {
            for (num_stored, index_stored) in value {
                let sum = *num_stored + num;
                if index != *index_stored && -10000 <= sum && sum <= 10000 {
                    let hit = hits.entry(sum).or_insert(HashSet::new());
                    match index > *index_stored {
                        true => hit.insert(index),
                        false => hit.insert(*index_stored),
                    };
                }
            }
        }
        if let Some(value) = sub_sum.get(&(target - 1)) {
            for (num_stored, index_stored) in value {
                let sum = *num_stored + num;
                if index != *index_stored && -10000 <= sum && sum <= 10000 {
                    let hit = hits.entry(sum).or_insert(HashSet::new());
                    match index > *index_stored {
                        true => hit.insert(index),
                        false => hit.insert(*index_stored),
                    };
                }
            }
        }
        if let Some(value) = sub_sum.get(&(target + 1)) {
            for (num_stored, index_stored) in value {
                let sum = *num_stored + num;
                if index != *index_stored && -10000 <= sum && sum <= 10000 {
                    let hit = hits.entry(sum).or_insert(HashSet::new());
                    match index > *index_stored {
                        true => hit.insert(index),
                        false => hit.insert(*index_stored),
                    };
                }
            }
        }
    }
    println!("Hits length{:?}", hits.len());
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
