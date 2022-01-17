use std::collections::HashMap;
use std::collections::VecDeque;

use super::error::ArgumentError;

type Graph = VecDeque<(usize, usize, usize)>;
type Parent = HashMap<usize, usize>;
type Rank = HashMap<usize, usize>;

pub fn cluster(mut graph: Graph, k: usize) -> Result<(), Box<dyn std::error::Error>> {
    let mut parent = HashMap::<usize, usize>::new();
    let mut rank = HashMap::<usize, usize>::new();
    for (node1, node2, _) in &graph {
        parent.insert(*node1, *node1);
        parent.insert(*node2, *node2);
        rank.insert(*node1, 0);
        rank.insert(*node2, 0);
    }
    let mut num_of_clusters = parent.len();

    while num_of_clusters > k {
        let edge = graph.pop_front().ok_or(ArgumentError)?;
        let cluster1 = find_leader(edge.0, &parent)?;
        let cluster2 = find_leader(edge.1, &parent)?;
        if cluster1 == cluster2 {
            continue;
        }
        union(cluster1, cluster2, &mut parent, &mut rank)?;
        num_of_clusters -= 1;
    }
    println!("next closest distance is {:?}", graph.get(0).unwrap());
    Ok(())
}

fn find_leader(mut x: usize, parent: &Parent) -> Result<usize, Box<dyn std::error::Error>> {
    loop {
        let parent_node = parent.get(&x).ok_or(ArgumentError)?;
        if parent_node == &x {
            break;
        }
        x = *parent.get(&x).ok_or(ArgumentError)?;
    }
    Ok(x)
}

fn union(s1: usize, s2: usize, parent: &mut Parent, rank: &mut Rank) -> Result<(), Box<dyn std::error::Error>>
{
    let rank1 = rank.get(&s1).ok_or(ArgumentError)?;
    let rank2 = rank.get(&s2).ok_or(ArgumentError)?;
    if rank1 == rank2 {
        // *rank2 += 1;
        let new_rank = rank2 + 1;
        rank.insert(s2, new_rank);
        parent.insert(s1, s2);
    } else if rank1 > rank2 {
        parent.insert(s2, s1);
    } else {
        parent.insert(s1, s2);
    }

    Ok(())
}

// psuedocode
// set initial state
//  - set cluster leader to self for each node
//  - set node rank to 0
//  - clusters = # of nodes
// while clusters > k
//  - find closest pair in separeted cluster ... need heap? or sort?
//  - merge these clusters
//  - decrement clusters
// return next minimum distance(is max space)

// How to find closest pair in separeted cluster
// - sort distances in increasing order
// - pop front
// - if nodes of that edge are in the same cluster, continue to next

// Check if two nodes are in the same cluster:
// - find(x) and find(y)  find leader... additional cost if I adopt Lazy Union
// - return find(x) == find(y)

// Find
// while parent[x] == x
// - x = parent[x]

// Merge clusters
// if rank(s1) > rank(s2)
//  - parent[s2] = s1
// else if rank(s1) < rank(s2)
//  - parent[s1] = s2
// else: ...rank(s1) == rank(s2)
//  - parent[s1] = s2
//  - rank(s2) +1
