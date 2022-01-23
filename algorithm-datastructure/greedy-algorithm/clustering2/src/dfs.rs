use std::collections::HashSet;
use std::iter::FromIterator;

type NODES = HashSet<String>;

pub fn connected_components(raw_nodes: Vec<String>) -> usize {
    let mut cc = 0usize;
    let mut explored = HashSet::<String>::new();
    let nodes = HashSet::<String>::from_iter(raw_nodes.clone());

    for node in raw_nodes {
        if !explored.contains(&node) {
            dfs(node, &nodes, &mut explored);
            cc += 1;
        }
    }
    cc
}

fn dfs(node: String, nodes: &NODES, explored: &mut NODES) {
    if explored.contains(&node) {
        return;
    }
    explored.insert(node.clone());
    for i in 0..24 {
        let flipped = flip(&node, i);
        if nodes.contains(&flipped) {
            dfs(flipped, nodes, explored);
        } else {
            // seek for 2-length-nodes
            for j in 0..24 {
                if i == j {
                    continue;
                }
                let flipped_twice = flip(&flipped, j);
                if nodes.contains(&flipped_twice) {
                    dfs(flipped_twice, nodes, explored);
                }
            }
        }
    }
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

// idea: dfs and retrieve connected components
// psuedocode
// (global variables)
// explored = hashset
// cc(connected components) = 0
// for v in noces
//  - dfs(v)
// dfs(node)
//  - skip if v is explored
//  - mark v as explored
//  - cc ++
//  for i in 0..24
//   - flip v[i]
//   if flip is found
//     - dfs(flip)
//   else(seek for length 2 nodes)
//     for j in 0..24
//       - skip if i==j
//       - flip v[j]
//       if flip is found
//         - dfs(flip)
