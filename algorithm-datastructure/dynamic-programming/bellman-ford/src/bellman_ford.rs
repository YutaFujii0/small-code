use super::graph::Graph;

pub fn bellman_ford(graph: Graph, source_index: usize) -> (Vec<i32>, bool) {
    // cost_matrix[i hops limitation][destination]
    let mut cost_matrix = vec![vec![i32::MAX; graph.nodes]; graph.nodes];
    cost_matrix[0][source_index] = 0;

    // n loop, note that last loop is for checking negative cycle
    for i in 1..graph.nodes {
        for dest_index in 0..graph.nodes {
            let prev = cost_matrix[i-1][dest_index];
            // let around = 1;
            let around = graph.inverse_neighbors(dest_index)
                .iter()
                .map(|&(src_index, cost)| -> i32 {
                    let prev = cost_matrix[i-1][src_index];
                    if prev == i32::MAX {
                        prev
                    } else {
                        prev + cost
                    }
                })
                .min()
                .unwrap_or_else(|| i32::MAX);
            cost_matrix[i][dest_index] = prev.min(around);
        }
    }
    (cost_matrix[graph.nodes - 1].clone(), cost_matrix[graph.nodes - 1] == cost_matrix[graph.nodes - 2])
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn three_nodes_graph_source1() {
        let graph = Graph::new(vec![(1,2,10), (1,3,5), (2,3,-2)]);
        assert_eq!(bellman_ford(graph, 0), (vec![0, 10, 5], true))
    }

    #[test]
    fn three_nodes_graph_source2() {
        let graph = Graph::new(vec![(1,2,10), (1,3,5), (2,3,-2)]);
        assert_eq!(bellman_ford(graph, 1), (vec![i32::MAX, 0, -2], true))
    }

    #[test]
    fn three_nodes_graph_source3() {
        let graph = Graph::new(vec![(1,2,10), (1,3,5), (2,3,-2)]);
        assert_eq!(bellman_ford(graph, 2), (vec![i32::MAX, i32::MAX, 0], true))
    }

    #[test]
    fn five_nodes_graph_source0() {
        let graph = Graph::new(vec![(1,2,2), (1,3,4), (2,3,1), (2,4,2), (3,5,4), (4,5,2)]);
        assert_eq!(bellman_ford(graph, 0), (vec![0,2,3,4,6], true))
    }

    #[test]
    fn detect_negative_cost_cycle() {
        let graph = Graph::new(vec![(1,2,2), (1,3,4), (2,3,1), (2,4,2), (3,5,4), (5,2,-10)]);
        assert_eq!(bellman_ford(graph, 0).1, false)
    }
}
