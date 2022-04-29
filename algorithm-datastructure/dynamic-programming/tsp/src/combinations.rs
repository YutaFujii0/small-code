pub fn combinations(n: usize) -> Vec<Vec<usize>> {
    let mut ans = vec![];

    fn recurse(
        i: usize,
        n: usize,
        vec: &mut Vec<usize>,
        ans: &mut Vec<Vec<usize>>
    ) {
        if vec.len() == n {
            return
        } else {
            for j in i..n {
                vec.push(j);
                ans.push(vec.clone());
                recurse(j+1, n, vec, ans);
                vec.pop();
            }
        }
    }

    recurse(0, n, &mut vec![], &mut ans);
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn n_two() {
        assert_eq!(combinations(2), vec![vec![0], vec![0, 1], vec![1]])
    }

    #[test]
    fn n_three() {
        let expected = vec![vec![0], vec![0,1], vec![0,1,2], vec![0,2], vec![1], vec![1,2], vec![2]];
        assert_eq!(combinations(3), expected)
    }
}
