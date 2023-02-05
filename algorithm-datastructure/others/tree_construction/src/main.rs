use std::io;
use std::error::Error;

use std::collections::VecDeque;

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();

    // first line of the input
    io::stdin().read_line(&mut buffer)?;
    let _n: u32 = buffer.trim().parse::<u32>()?;

    // second line of the input
    buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let seq: Vec<u64> = buffer.trim()
        .split(' ')
        .map(|item| item.parse::<u64>().unwrap())
        .collect();

    let mut deq = VecDeque::from(seq);

    // println!("{} and {:?}", _n, deq);

    let ans = Solve::solve(&mut deq);
    println!("{}", ans.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
    Ok(())
}

struct Solve;

impl Solve {
    pub fn solve(seq: &mut VecDeque<u64>) -> Vec<u64> {
        let mut head = Tree::new(seq.pop_front().unwrap());

        let mut parents = vec![];

        while seq.len() > 0 {
            let val = seq.pop_front().unwrap();
            // back to root
            let mut cur = &mut head;
            let mut found = false;

            while !found {
                let cur_val = cur.val.unwrap();
                // println!("cur_val {} val {}", cur_val, val);
                if cur_val > val {
                    match cur.right {
                        None => {
                            cur.right = Some(Box::new(Tree::new(val)));
                            parents.push(cur_val);
                            found = true;
                        },
                        Some(ref mut tree) => {
                            cur = &mut *tree;
                        }
                    }
                } else {
                    match cur.left {
                        None => {
                            cur.left = Some(Box::new(Tree::new(val)));
                            parents.push(cur_val);
                            found = true;
                        },
                        Some(ref mut tree) => {
                            cur = &mut *tree;
                        }
                    }
                }
            }
        }
        parents
    }
}

struct Tree {
    left: Option<Box<Tree>>,
    right: Option<Box<Tree>>,
    val: Option<u64>,
}

impl Tree {
    pub fn new(val: u64) -> Tree {
        Self {
            left: None,
            right: None,
            val: Some(val),
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_work_for_edge_case_minimum_n() {
        let mut seq: VecDeque<u64> = VecDeque::from(vec![1, 2]);
        let actual = Solve::solve(&mut seq);
        let expected: Vec<u64> = vec![1];
        assert!(expected == actual);
    }

    #[test]
    fn should_work_for_edge_case_maximum_a_i() {
        let mut seq: VecDeque<u64> = VecDeque::from(vec![1_000_000_000, 999_999_999]);
        let actual = Solve::solve(&mut seq);
        let expected: Vec<u64> = vec![1_000_000_000];
        assert!(expected == actual);
    }

    #[test]
    fn case1() {
        let mut seq: VecDeque<u64> = VecDeque::from(vec![4, 2, 3, 1, 6]);
        let actual = Solve::solve(&mut seq);
        let expected: Vec<u64> = vec![4, 2, 2, 4];
        assert!(expected == actual, "expected: {:?} vs actual: {:?}", expected, actual);
    }

    #[test]
    fn case2() {
        let mut seq: VecDeque<u64> = VecDeque::from(vec![4, 1, 2, 3, 6]);
        let actual = Solve::solve(&mut seq);
        let expected: Vec<u64> = vec![4, 1, 2, 4];
        assert!(expected == actual, "expected: {:?} vs actual: {:?}", expected, actual);
    }

    #[test]
    fn case3() {
        let mut seq: VecDeque<u64> = VecDeque::from(vec![2, 6, 4, 3, 1]);
        let actual = Solve::solve(&mut seq);
        let expected: Vec<u64> = vec![2, 6, 4, 2];
        assert!(expected == actual, "expected: {:?} vs actual: {:?}", expected, actual);
    }

    #[test]
    fn case4() {
        let mut seq: VecDeque<u64> = VecDeque::from(vec![6, 4, 3, 2, 1]);
        let actual = Solve::solve(&mut seq);
        let expected: Vec<u64> = vec![6, 4, 3, 2];
        assert!(expected == actual, "expected: {:?} vs actual: {:?}", expected, actual);
    }
}
