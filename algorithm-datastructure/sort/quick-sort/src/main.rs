use std::path::Path;
use std::fs::{File};
use std::io::{self, BufRead};


const PATH: &str = "./dataset-sample.txt";

fn main() {
    println!("Quick sort!");
    let res = parse(PATH);
    if let Ok(mut nums) = res {
        println!("comparisons {:?}", quick_sort_choose_first(&mut nums));
    }
}

fn quick_sort_choose_first(nums: &mut [i64]) -> usize {
    if nums.len() <= 1 {
        return 0
    }
    let pivot_index = 0;
    let pivot = nums[pivot_index];
    let mut i = 1;
    let mut j = 1;
    let comparisons = nums.len() - 1;
    // compare
    while j < nums.len() {
        if nums[j] < pivot {
            let tmp = nums[j];
            nums[j] = nums[i];
            nums[i] = tmp;
            i += 1;
        }
        j += 1;
    }
    i -= 1;
    let tmp = nums[i];
    nums[i] = nums[pivot_index];
    nums[pivot_index] = tmp;

    // split
    let comps_left = quick_sort_choose_first(&mut nums[..i]);
    let comps_right = quick_sort_choose_first(&mut nums[i+1..]);

    comparisons + comps_left + comps_right
}


fn parse<T>(path: T) -> io::Result<Vec<i64>>
where T: AsRef<Path>
{
    let mut nums = Vec::<i64>::new();
    let file = File::open(path)?;
    for line in io::BufReader::new(file).lines() {
        let line = line?;
        if let Ok(val) = line.parse::<i64>() {
            nums.push(val);
        }
    }
    Ok(nums)
}
