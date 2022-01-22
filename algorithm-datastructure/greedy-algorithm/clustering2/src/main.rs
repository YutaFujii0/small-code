mod parse;
use crate::parse::*;

use std::collections::HashSet;
use std::iter::FromIterator;

// const FILE_PATH: &str = "dataset-sample.txt";
const FILE_PATH: &str = "dataset.txt";

fn main() {
    println!("clustering!");
    if let Ok(nodes) = parse(FILE_PATH) {
        let count = count_nodes_of_length_within_2(nodes);
        println!("cluster needed {:?}", count);
    }
}

fn count_nodes_of_length_within_2(raw_nodes: Vec<String>) -> usize {
    let nodes: HashSet<String> = HashSet::from_iter(raw_nodes.clone());
    let mut nodes_in_length: HashSet<String> = HashSet::new();
    let mut second_iterations: Vec<(String, String)> = vec![];
    for node in raw_nodes {
        for i in 0..24 {
            let flipped = flip(&node, i);
            if nodes.contains(&flipped) {
                nodes_in_length.insert(node.clone());
                nodes_in_length.insert(flipped);
            } else {
                second_iterations.push((flipped, node.clone()));
            }
        }
    }
    for (middle_node, original_node) in second_iterations {
        for i in 0..24 {
            let flipped = flip(&middle_node, i);
            if nodes.contains(&flipped) && flipped != original_node {
                nodes_in_length.insert(original_node.clone());
                nodes_in_length.insert(flipped);
            }
        }
    }
    nodes.len() - nodes_in_length.len() + 1
}

fn flip(bit_num: &String, i: usize) -> String {
    let s1 = &bit_num[..i];
    let s2 = &bit_num[i..i + 1];
    let s3 = &bit_num[i + 1..];
    let mut owned = String::from(s1);
    match s2 {
        "0" => owned.push_str("1"),
        "1" => owned.push_str("0"),
        _ => (),
    }
    owned.push_str(s3);
    owned
}
// idea: hashmaps
// Psuedocode
// - store nodes into hashmap
// - explored: hashset
// - second_iterations: []
// for node in nodes
//  skip if explored
//  for each bit (24bits)
//   - flip the bit and look for node in hashmap
//     if found, store both nodes in explored
//     if not found, push that into second iterations(flipped, original)
// for virtual_node in second_iterations
//  for each bit (24bits)
//   - flip the bit and look for node in hashmap
//     if found, store both nodes in explored
