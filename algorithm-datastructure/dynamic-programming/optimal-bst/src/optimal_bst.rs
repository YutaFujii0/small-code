
pub fn optimal_bst(tree: Vec<f32>) -> f32 {
    let n = tree.len();
    let mut dp = vec![vec![0f32; n]; n];
    // stride between i and j
    for s in 0..n {
        for i in 0..n-s {
            println!("s: {} i: {}", s, i);
            let mut candidates = vec![];
            for r in i..=i+s {
                let mut local = 0f32;
                if r != i {
                    local +=  dp[i][r-1];
                }
                if r != i+s {
                    local +=  dp[r+1][i+s];
                }
                candidates.push(local);
            }
            println!("{:?}", candidates);
            let sum_p = tree[i..=i+s].iter().sum::<f32>();
            dp[i][i+s] = sum_p + candidates.into_iter().reduce(f32::min).unwrap();
        }
    }
    dp[0][n-1]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let tree = vec![0.05, 0.40, 0.08, 0.04, 0.1, 0.1, 0.23];
        assert_eq!(optimal_bst(tree), 2.18);
    }

    #[test]
    fn test2() {
        let tree = vec![0.2, 0.05, 0.17, 0.1, 0.2, 0.03, 0.25];
        assert_eq!(optimal_bst(tree), 2.23);
    }
}
