use std::collections::HashMap;

#[derive(Debug)]
struct Edge {
    nodes: (usize, usize),
    length: usize,
}

#[derive(Debug)]
pub struct Graph {
    edges: Vec<Edge>
}

impl Graph {
    pub fn new(edges: Vec<(usize, usize, usize)>) -> Self {
        let mut graph = Graph {
        edges: vec![]
        };
        for (a, b, l) in edges {
        graph.edges.push(Edge{ nodes: (a, b), length: l });
        }
        graph
    }

    pub fn dijstra(&self) {
        let mut score = HashMap::<usize, usize>::new();
        score.insert(1, 0);

        for _ in 0..200 {
            // n times loop
            let mut min_score: usize = 1000000;
            let mut min_node: usize = 1000000;
            for edge in &self.edges {
                // find edges go out/in frontier
                // if it is, reck on greedy score
                // this logic is to be modified
                if let Some((v, w)) = Self::is_frontier(edge.nodes, &score) {
                    // score(w) = L(v) + length
                    let length_v = score.get(&v).unwrap();
                    if min_score > edge.length + length_v {
                        min_score = edge.length + length_v;
                        min_node = w;
                    }
                }
            }
            if min_node != 1000000 {
                score.insert(min_node, min_score);
            }
        }

        // println!("score {:?}", score);
        for i in vec![7,37,59,82,99,115,133,165,188,197] {
            println!("distance of {:?} is {:?}", i, score.get(&i).unwrap());
        }
    }

    fn is_frontier(edge: (usize, usize), state: &HashMap<usize, usize>) -> Option<(usize, usize)> {
        let (left, right) = edge;
        if state.contains_key(&left) && !state.contains_key(&right) {
            return Some((left, right))
        } else if !state.contains_key(&left) && state.contains_key(&right) {
            return Some((right, left))
        }
        return None
    }
}
