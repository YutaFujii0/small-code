use std::collections::BinaryHeap;
use std::cmp::Ordering;

use crate::error::ArgumentError;

use self::Node::{Cons, Nil};


#[derive(Eq, Debug)]
pub enum Node {
    Cons(u32, Box<Node>, Box<Node>),
    Nil,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}


#[derive(Eq, Debug)]
struct CmpNode (i32, Node);

impl Ord for CmpNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for CmpNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for CmpNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

pub fn huffman(weights: Vec<u32>) -> Result<Node, Box<dyn std::error::Error>> {
    let weights = weights.iter()
        .map(|&w| CmpNode(-1 * w as i32, Cons(w, Box::new(Nil), Box::new(Nil))))
        .collect::<Vec<CmpNode>>();

    let mut heap = BinaryHeap::from(weights);
    let mut head = Cons(0, Box::new(Nil), Box::new(Nil));
    while heap.len() > 2 {
        let first = heap.pop().ok_or(ArgumentError)?;
        let second = heap.pop().ok_or(ArgumentError)?;
        let combined = Cons(0, Box::new(first.1), Box::new(second.1));
    }
    // println!("heap pop {:?}", );
    
    Ok(head)
}


