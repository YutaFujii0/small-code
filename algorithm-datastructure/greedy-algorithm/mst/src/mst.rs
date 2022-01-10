// psuedocode
// while T is not empty
//  - pick an edge crossing frontier such that additional cost is minimized
//  - remove node(v) of T-side of the edge and mark v as explored
//  - also, add the edge into V
//  - for edges of v that crossing new frontier(i.e. the other side of node is still in T)
//   - remove that node from heap
//   - recalculate minimum cost: compare current value vs the cost of the edge
//   - add the node into heap

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::cmp::{Ord, PartialOrd, Ordering};

#[derive(Debug)]
pub struct Edge {
    pub node: usize,
    pub cost: i64
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for Edge {}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub type Edges = HashMap<usize, Vec<Edge>>;

#[derive(Debug)]
pub struct Graph {
    pub nodes: Vec<usize>,
    pub edges: Edges,
}


pub fn mst(graph: Graph) -> Result<i64, Box<dyn std::error::Error>> {
    let mut explored = HashSet::new();
    let mut heap = BinaryHeap::new();
    let mut total_cost: i64 = 0;
    let e = graph.edges.get(&graph.nodes[0]);

    explored.insert(graph.nodes[0]);
    if let Some(e) = e {
        let initial_frontier_edges = e.iter()
            .filter(|&edge| explored.get(&edge.node) == None)
            .collect::<Vec<&Edge>>();
        for edge in initial_frontier_edges {
            heap.push(Reverse(edge));
        }
    }

    while explored.len() < graph.nodes.len() {
        let w = heap.pop();
        if let Some(w) = w {
            if explored.contains(&w.0.node) {
                continue;
            }

            explored.insert(w.0.node);
            total_cost += w.0.cost;
            let edges_from_w = graph
                .edges
                .get(&w.0.node)
                .unwrap()
                .iter()
                .filter(|&edge| !explored.contains(&edge.node))
                .collect::<Vec<&Edge>>();

            for edge in edges_from_w {
                heap.push(Reverse(edge));
            }
        }
    }

    Ok(total_cost)
}
// psuedocode
// while T is not empty
//  - pick an edge crossing frontier such that additional cost is minimized
//  - remove node(v) of T-side of the edge and mark v as explored
//  - also, add the edge into V
//  - for edges of v that crossing new frontier(i.e. the other side of node is still in T)
//   - remove that node from heap
//   - recalculate minimum cost: compare current value vs the cost of the edge
//   - add the node into heap
