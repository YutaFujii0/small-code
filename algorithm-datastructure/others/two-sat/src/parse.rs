use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn parse<T>(path: T) -> Result<(usize, Vec<(i32, i32)>), Box<dyn std::error::Error>>
where T: AsRef<Path>
{
    let mut ret = vec![];
    let mut data_count = 0usize;
    let file = File::open(path)?;
    for (index, line) in BufReader::new(file).lines().enumerate() {
        let line = line?;
        if index == 0 {
            data_count = line.parse::<usize>().unwrap();
            continue;
        }
        let raw = line.split(" ")
            .map(|num| num.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        ret.push((raw[0], raw[1]));
    }

    Ok((data_count, ret))
}

pub fn form(dataset: Vec<(i32, i32)>) -> Vec<(usize, usize)> {
    let mut ret: Vec<(usize, usize)> = vec![];
    for (mut a, mut b) in dataset {
        match (a > 0, b > 0) {
            (true, true) => {
                ret.push((
                    usize::try_from(a*2-1).unwrap(),
                    usize::try_from(b*2).unwrap(),
                ));
                ret.push((
                    usize::try_from(b*2-1).unwrap(),
                    usize::try_from(a*2).unwrap(),
                ));
            },
            (true, false) => {
                b *= -1;
                ret.push((
                    usize::try_from(a*2-1).unwrap(),
                    usize::try_from(b*2-1).unwrap(),
                ));
                ret.push((
                    usize::try_from(b*2).unwrap(),
                    usize::try_from(a*2).unwrap(),
                ));
            },
            (false, true) => {
                a *= -1;
                ret.push((
                    usize::try_from(a*2).unwrap(),
                    usize::try_from(b*2).unwrap(),
                ));
                ret.push((
                    usize::try_from(b*2-1).unwrap(),
                    usize::try_from(a*2-1).unwrap(),
                ));
            },
            (false, false) => {
                a *= -1;
                b *= -1;
                ret.push((
                    usize::try_from(a*2).unwrap(),
                    usize::try_from(b*2-1).unwrap(),
                ));
                ret.push((
                    usize::try_from(b*2).unwrap(),
                    usize::try_from(a*2-1).unwrap(),
                ));
            },
        }
    }

    return ret
}