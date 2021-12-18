#![allow(dead_code)]
use std::path::Path;
use std::fs::{File};
use std::io::{self, BufRead};

mod qsort_choose_first;
mod qsort_choose_last;
mod qsort_choose_median;
use qsort_choose_first::*;
use qsort_choose_last::*;
use qsort_choose_median::*;

const PATH: &str = "./dataset.txt";

fn main() {
    println!("Quick sort!");
    let res = parse(PATH);
    if let Ok(nums) = res {
        println!("comparisons(choose first): {:?}", quick_sort_choose_first(&mut nums.clone()));
        println!("comparisons(choose last): {:?}", quick_sort_choose_last(&mut nums.clone()));
        println!("comparisons(choose median): {:?}", quick_sort_choose_median(&mut nums.clone()));
    }
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
