use datastructures::HeapNode;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Node {
    id: usize,
    edges: Vec<(usize, usize)>,
}

#[derive(Debug)]
pub struct Graph {
    nodes: Vec<Node>
}

type HeapElement = (usize, usize);

impl Graph {
    pub fn new(nodes: Vec<(usize, Vec<(usize, usize)>)>) -> Self {
        let mut graph = Graph {
            nodes: vec![]
        };
        for (a, b) in nodes {
            graph.nodes.push(Node{ id: a, edges: b });
        }
        graph
    }

    pub fn dijstra(&self) {
        let mut score = HashMap::<usize, usize>::new();
        score.insert(1, 0);
        let mut heap = HeapNode::<HeapElement, _>::new(Self::out_of_order);

        // set up initial heap(node and length from source node)
        for (node, length) in &self.nodes[0].edges {
            heap.insert((*node, *length));
        }

        while score.len() < 200 {
            let head = heap.poll();
            if let Some((node_id, greedy_score)) = head {
                if !score.contains_key(&node_id) {
                    score.insert(node_id, greedy_score);
                    for edge in self.nodes[node_id - 1]
                        .edges
                        .iter()
                        .filter(|(node_id, _)| !score.contains_key(&node_id)) {
                            let (node, length) = edge;
                            heap.insert((*node, greedy_score + length));
                        }
                }
            }
        }

        for i in vec![7,37,59,82,99,115,133,165,188,197] {
            println!("distance of {:?} is {:?}", i, score.get(&i).unwrap());
        }
    }

    fn out_of_order(parent: &HeapElement, child: &HeapElement) -> bool {
        parent.1 > child.1
    }
}
