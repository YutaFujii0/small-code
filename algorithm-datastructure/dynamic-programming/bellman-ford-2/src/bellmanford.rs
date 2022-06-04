use std::error::Error;

use gcollections::{adjacency_matrix::AdjacencyMatrix, Graph};

pub type Cost = i64;
pub type VertexValue = usize;
pub type RawData = Vec<(VertexValue, VertexValue, Cost)>;

pub struct Solution;

impl Solution {
    pub fn call(edge_list: RawData) -> Result<Cost, Box<dyn Error>> {
        let mut graph = AdjacencyMatrix::<VertexValue, Cost>::new();

        // scan edge list to collect nodes
        let n = &edge_list.iter()
            .map(|item| item.0)
            .max()
            .ok_or_else(|| "Unable to find max index of node from argument.")?;

        // add node
        for i in 0..*n {
            graph.add_vertex(i);
        }

        // add edges
        for edge in edge_list {
            let edge = (edge.0 - 1, edge.1 - 1, edge.2);
            graph.add_edge(edge);
        }

        // let A as 2D array
        let mut cost_matrix = vec![vec![i64::MAX; *n]; *n];
        // set initial value for A[0, s]
        cost_matrix[0][0] = 0;

        // for i = 1.. n (nth iteration is for checking negative cycle)
        for i in 1..*n {
            // for each v in V
            for j in 0..*n {
                // A[i,v] = min (A[i-1, v], min(A[i-1, w] + c_wv))
                let cost_of_last_iter = cost_matrix[i - 1][j];
                // let min_cost_from_neighbor = graph.neighbors(j)
            }
        }
        Ok(1i64)
    }
}