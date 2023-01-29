use std::io;
use std::error::Error;

// naive solution
// spread n pillows
// give 1 to kth
// if remaining >= 3, give 1 to kth, k-1th, k+1th
// if remaining >= 5, give 1 to kth, k{+,-}{1,2}th
// if remaining >= 7, give 1 to kth, k{+,-}{1...3}th
// for Frodo to have x extra pillows, x^2 pillows needed so that no one hurts
// but ...too strict, I can calculate if I work a bit more


// Solution concept: binary search
// considerations: 
// - given an answer, we can evaluate it
// - if it's valid, it always means the answer should be less.
// - possible answers, m at the maximum
// therfore, find the answer in a bit smarter bruteforce: binary search

// Complexity
// - time: O(logn): O(logn) searching and O(1) for each work
// - space: O(1) just storing variables, and works inside the loop

// Psuedocode
// l = 1, r = m
// while (l <= r)
//   - try = m (we can start from try = l+r/2)
//   - if try is valid, l = try + 1
//   - else, r = try
// return try
// fn valid(try)
//   pillows needed:
//     everyone 1 at least: n
//     + try in total for Frodo: try - 1
//     + trapezoid left: (min[n-k, try-1])*(try-1 + try-min[n-k, try-1])/2
//     + trapezoid right: (min[k-1, try-1])*(try-1 + try-min[k-1, try-1])/2
//   return true if pillows_needed <= m

// fig) o: pillows needed
//         k
// xxxxxxxxoxx
// xxxxxxxooox
// xxxxxxooooo
// xxxxxoooooo
// xxxxooooooo


fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    let args: Vec<&str> = buffer.trim()
        .split(' ')
        .collect();

    let (n, m, k) = (
        args[0].parse::<u64>()?, 
        args[1].parse::<u64>()?, 
        args[2].parse::<u64>()?);
    // println!("n:{:?} m:{:?} k:{:?}", n, m, k);

    println!("{}", Solve::solve(n, m, k));
    Ok(())
}

pub struct Solve;

impl Solve {
    pub fn solve(n: u64, m: u64, k: u64) -> u64 {
        let mut answer: u64;

        let mut l = 1u64;
        let mut r = m;
        loop {
            answer = (l+r)/2 + 1;
            if Self::feasible(n, m, k, answer) {
                l = answer;
            } else {
                r = answer - 1;
                answer -= 1;
            }
            if l >= r { break; }
        }

        answer
    }

    fn feasible(n: u64, m: u64, k: u64, answer: u64) -> bool {
        let mut pillows_needed = 0u64;

        pillows_needed += answer;

        // let r_height = m;
        let r_height = *[n - k, answer - 1].iter().min().unwrap();
        let l_height = *[k - 1, answer - 1].iter().min().unwrap();
        pillows_needed += ((answer - 1 + answer - r_height) * r_height)/2;
        pillows_needed += ((answer - 1 + answer - l_height) * l_height)/2;

        pillows_needed += n - 1 - r_height - l_height;

        if pillows_needed <= m { return true } else { return false }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_work_for_minimum_edge_case() {
        let (n, m, k) = (1, 1, 1);
        let expected = 1u64;
        let actual = Solve::solve(n, m, k);
        assert!(expected == actual);
    }

    #[test]
    fn case1() {
        let (n, m, k) = (4, 6, 2);
        let expected = 2u64;
        let actual = Solve::solve(n, m, k);
        assert!(expected == actual);
    }

    #[test]
    fn case2() {
        let (n, m, k) = (3, 10, 3);
        let expected = 4u64;
        let actual = Solve::solve(n, m, k);
        assert!(expected == actual);
    }

    #[test]
    fn case3() {
        let (n, m, k) = (3, 6, 1);
        let expected = 3u64;
        let actual = Solve::solve(n, m, k);
        assert!(expected == actual);
    }

    #[test]
    fn case4() {
        let (n, m, k) = (10, 10, 1);
        let expected = 1u64;
        let actual = Solve::solve(n, m, k);
        assert!(expected == actual);
    }

    #[test]
    fn case5() {
        let (n, m, k) = (100, 1000000000, 20);
        let expected = 10000034u64;
        let actual = Solve::solve(n, m, k);
        assert!(expected == actual, "expected:{}, actual:{}", expected, actual);
    }
}