use std::cmp;

use super::item::Item;

pub fn dp(items: &[Item], knapsack: usize) -> u32
{
    // println!("{:?}, {:?}", items, knapsack);
    let n = items.len();
    let mut dp_arr = vec![vec![0u32; knapsack + 1]; n+1];

    fn recurse(n: usize, items: &[Item], knapsack: usize, mut dp_arr: &mut Vec<Vec<u32>>) {
        if dp_arr[n][knapsack] != 0 {
            return
        }
        if n == 1 {
            if knapsack >= items[0].weight {
                dp_arr[n][knapsack] = items[0].value;
                return
            } else {
                // TODO: I don't expect to come here
                // If dataset is such that comes here, it can cause infinite loop
                return
            }
        }

        if knapsack < items[n-1].weight {
            recurse(n-1, &items, knapsack, &mut dp_arr);
            dp_arr[n][knapsack] = dp_arr[n-1][knapsack];
        } else {
            recurse(n-1, &items, knapsack - items[n-1].weight, &mut dp_arr);
            recurse(n-1, &items, knapsack, &mut dp_arr);
            dp_arr[n][knapsack] = cmp::max(dp_arr[n-1][knapsack], dp_arr[n-1][knapsack - items[n-1].weight] + items[n-1].value);
        }
    }

    recurse(n, &items, knapsack, &mut dp_arr);
    dp_arr[n][knapsack]
}

#[cfg(test)]
mod test {
    use super::*;

    fn itemize(raw: Vec<(u32, usize)>) -> Vec<Item> {
        raw.iter()
            .map(|&(value, weight)| Item{ value, weight })
            .collect()
    }

    #[test]
    fn return_zero_if_knapsack_cannot_contain_any_item() {
        let items = vec![(100, 10)];
        let knapsack = 1usize;
        assert_eq!(dp(&itemize(items), knapsack), 0);
    }

    #[test]
    fn knapsack_holding_one_item_returns_its_value() {
        let items = vec![(100, 10)];
        let knapsack = 10usize;
        assert_eq!(dp(&itemize(items), knapsack), 100);
    }

    #[test]
    fn two_items1() {
        let items = vec![(100, 10), (200, 10)];
        let knapsack = 10usize;
        assert_eq!(dp(&itemize(items), knapsack), 200);
    }

    #[test]
    fn two_items2() {
        let items = vec![(200, 10), (100, 10)];
        let knapsack = 10usize;
        assert_eq!(dp(&itemize(items), knapsack), 200);
    }

    #[test]
    fn two_items3() {
        let items = vec![(200, 5), (100, 5)];
        let knapsack = 10usize;
        assert_eq!(dp(&itemize(items), knapsack), 300);
    }

    #[test]
    fn complex_items1() {
        let items = vec![(200, 5), (100, 1), (300, 6)];
        let knapsack = 10usize;
        assert_eq!(dp(&itemize(items), knapsack), 400);
    }

    #[test]
    fn complex_items2() {
        let items = vec![(200, 5), (100, 1), (300, 6), (250, 5)];
        let knapsack = 10usize;
        assert_eq!(dp(&itemize(items), knapsack), 450);
    }

    #[test]
    fn complex_items3() {
        let items = vec![(200, 2), (100, 1), (300, 3), (250, 1), (50, 1), (150, 3), (200, 3), (500, 4), (1000, 7), (100, 1)];
        let knapsack = 10usize;
        assert_eq!(dp(&itemize(items), knapsack), 1450);
    }
}
