use super::errors::*;

pub struct Graph {
    matrix: Vec<Vec<i32>>,
    pub nodes: usize,
}

impl Graph {
    pub fn new(edges: Vec<(usize, usize, i32)>) -> Self {
        // find max index O(n)
        // create NxN matrix with i32::MIN
        // iterate edges and store cost
        let from_max = edges
            .iter()
            .map(|edge| edge.0)
            .max()
            .ok_or(NoArgumentError)
            .unwrap();
        let to_max = edges
            .iter()
            .map(|edge| edge.1)
            .max()
            .ok_or(NoArgumentError)
            .unwrap();
        let n: usize = from_max.max(to_max);
        let mut matrix = vec![vec![i32::MIN; n]; n];
        for (from, to, cost) in edges {
            matrix[from - 1][to - 1] = cost;
        }
        Graph { matrix, nodes: n }
    }

    // pub fn add_node(&self) {

    // }

    // pub fn add_edge(&self, edge: Vec<i32>) {

    // }

    pub fn neighbors(&self, index: usize) -> Vec<(usize, i32)> {
        self.matrix[index]
            .iter()
            .enumerate()
            .filter(|&item| *item.1 != i32::MIN)
            .map(|item| (item.0, *item.1))
            .collect::<Vec<(usize, i32)>>()
    }

    pub fn inverse_neighbors(&self, index: usize) -> Vec<(usize, i32)> {
        self.matrix
            .iter()
            .enumerate()
            .filter(|&item| item.1[index] != i32::MIN)
            .map(|item| (item.0, item.1[index]))
            .collect::<Vec<(usize, i32)>>()
    }
}
