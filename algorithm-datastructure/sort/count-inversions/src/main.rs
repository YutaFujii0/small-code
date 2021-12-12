use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::VecDeque;

const FILE_PATH: &str = "dataset.txt";
// const FILE_PATH: &str = "dataset-sample.txt";

fn main() -> io::Result<()> {
    println!("Counting inversions!");
    let nums = parse(FILE_PATH)?;
    let (_, inversions) = find_inversion(&nums.as_slice());
    println!("answer {:?}", inversions);
    Ok(())
}

fn find_inversion(nums: &[i64]) -> (VecDeque<i64>, usize) {
    if nums.len() == 1 {
        return (VecDeque::from(Vec::from(nums)), 0)
    }
    let half = nums.len() / 2;
    // (1) merge sort and count for left
    let (mut left_sorted, left_count) = find_inversion(&nums[..half]);
    // (2) merge sort and count for right
    let (mut right_sorted, right_count) = find_inversion(&nums[half..]);
    // (3) merge left and right with counting inversions
    let mut merged = VecDeque::from(vec![]);
    let mut count = 0;
    while left_sorted.len() > 0 && right_sorted.len() > 0 {
        if left_sorted[0] > right_sorted[0] {
            if let Some(item) = right_sorted.pop_front() {
                merged.push_back(item);
                count += left_sorted.len();
            }
        } else {
            if let Some(item) = left_sorted.pop_front() {
                merged.push_back(item);
            }
        }
    }
    while left_sorted.len() > 0 {
        if let Some(item) = left_sorted.pop_front() {
            merged.push_back(item);
        }
    }
    while right_sorted.len() > 0 {
        if let Some(item) = right_sorted.pop_front() {
            merged.push_back(item);
        }
    }
    // (1) + (2) + (3)
    (merged, left_count + right_count + count)
}

fn parse<T>(path: T) -> io::Result<Vec::<i64>>
where T: AsRef<Path>
{
    let file = File::open(path)?;
    let mut nums = Vec::<i64>::new();
    for line in io::BufReader::new(file).lines() {
        let line = line?;
        if let Ok(val) = line.parse::<i64>() {
            nums.push(val);
        }
    }
    Ok(nums)
}
