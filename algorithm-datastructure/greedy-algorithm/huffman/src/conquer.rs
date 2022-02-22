use crate::huffman::Node::{self, Cons, Nil};
use std::cmp::{min, max};

// divide and conquer
pub fn min_max_depth(tree: Node, depth: usize) -> (usize, usize) {
    match tree {
        Cons(_, left, right) => {
            match (*left, *right) {
                (Nil, Nil) => return (depth, depth),
                (left, right) => {
                    let (l_min, l_max) = min_max_depth(left, depth + 1);
                    let (r_min, r_max) = min_max_depth(right, depth + 1);
                    return (min(l_min, r_min), max(l_max, r_max))
                },
            }
        },
        _ => return (0, 0),
    }
}