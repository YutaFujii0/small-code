use std::collections::HashMap;
use std::cmp::Ordering;
use super::combinations::*;

pub fn tsp(graph: Vec<(f32, f32)>) -> i32 {
    let n = graph.len();
    let num_of_subsets = usize::pow(2, n as u32);
    let mut costs = vec![vec![f32::MAX; n]; num_of_subsets];
    costs[0][0] = 0.0;

    let subsets = combinations(n).into_iter()
        .filter(|item| item.contains(&0))
        .collect::<Vec<Vec<usize>>>();
    
    let mut subsets_index_map = HashMap::new();
    for i in 0..subsets.len() {
        subsets_index_map.insert(&subsets[i], i);
    }

    // for m in 1..n
    for m in 1..n+1 {
        // for every subset S that contains 0th node and length equals to m
        for subset in &subsets {
            if subset.len() != m {
                continue;
            }
            let subset_index = subsets_index_map.get(subset).unwrap();

            // for each j in S s.t j != 0
            for i in 0..subset.len() {
                let j = subset[i];
                if j == 0 {
                    continue;
                }

                // A[hash(S), j] = min A[hash(S-{j}), k] + c_kj (k in S and k != j)
                let mut candidates = vec![];
                for k in 0..m {
                    let k = subset[k];
                    if k == j {
                        continue;
                    }
                    let mut dummy = subset.clone();
                    dummy.remove(i);
                    let index = subsets_index_map.get(&dummy).unwrap();
                    let prev_cost = costs[*index][k];
                    let c_jk = ((graph[j].0 - graph[k].0).powi(2) + (graph[j].1 - graph[k].1).powi(2)).sqrt();
                    candidates.push(prev_cost + c_jk);
                }
                costs[*subset_index][j] = candidates.into_iter()
                    .reduce(f32::min)
                    .unwrap_or_else(|| f32::MAX);
            }
        }
    }

    let index_of_universe = subsets_index_map.get(&(0..n).collect::<Vec<usize>>()).unwrap();

    let final_trips = (0..n)
        .map(|i| ((graph[i].0 - graph[0].0).powi(2) + (graph[i].1 - graph[0].1).powi(2)).sqrt())
        .collect::<Vec<f32>>();
    let min = costs[*index_of_universe].clone()
        .into_iter()
        .enumerate()
        .min_by(|(i_a, a), (i_b, b)| (a + final_trips[*i_a]).partial_cmp(&(b + final_trips[*i_b])).unwrap_or(Ordering::Equal))
        .unwrap_or_else(|| (0, f32::MAX));
    let c_ji = ((graph[min.0].0 - graph[0].0).powi(2) + (graph[min.0].1 - graph[0].1).powi(2)).sqrt();
    println!("raw result: {} = {} + {}| from {}th node", (min.1 + c_ji), min.1, c_ji, min.0);
    (min.1 + c_ji).floor() as i32
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn two_nodes1() {
        let graph = vec![(0.0, 0.0), (0.0, 1.0)];
        assert_eq!(tsp(graph), 2)
    }

    #[test]
    fn two_nodes2() {
        let graph = vec![(0.0, 0.0), (10.0, 10.0)];
        assert_eq!(tsp(graph), 28)
    }

    #[test]
    fn three_nodes1() {
        let graph = vec![(0.0, 0.0), (10.0, 0.0), (10.0, 10.0)];
        assert_eq!(tsp(graph), 34)
    }

    #[test]
    fn four_nodes1() {
        let graph = vec![(0.0, 0.0), (10.0, 0.0), (0.0, 10.0), (10.0, 10.0)];
        assert_eq!(tsp(graph), 40)
    }

    #[test]
    fn nodes_in_the_same_line() {
        let graph = vec![(0.0, 0.0), (10.0, 0.0), (11.1, 0.0)];
        assert_eq!(tsp(graph), 22)
    }

    #[test]
    fn complex_graph() {
        let graph = vec![
            (0.0, 0.0), (10.0, 0.0), (0.0, 10.0), (10.0, 10.0), (10.0, -10.0), (0.0, -10.0), (-10.0, 0.0)
        ];
        assert_eq!(tsp(graph), 74)
    }

    #[test]
    fn complex_graph2() {
        let graph = vec![
            (0.0000000549963, 0.00000000985808),
            (-28.8733,-0.0000000797739),
            (-79.2916, -21.4033),
            (-14.6577, -43.3896),
            (-64.7473,  21.8982),
            (-29.0585, -43.2167),
            (-72.0785, 0.181581),
            (-36.0366, -21.6135),
            (-50.4808,  7.37447),
            (-50.5859, -21.5882),
            (-0.135819, -28.7293),
            (-65.0866, -36.0625),
            (-21.4983,  7.31942),
            (-57.5687, -43.2506),
            (-43.0700,  14.5548)
        ];
        assert_eq!(tsp(graph), 284)
    }

    #[test]
    fn round_down() {
        let graph = vec![
            (0.0, 0.0), (10.0, 10.0), (20.0, 20.0), (30.0, 30.0), (40.0, 40.0)
        ];
        assert_eq!(tsp(graph), 113)
    }
}

