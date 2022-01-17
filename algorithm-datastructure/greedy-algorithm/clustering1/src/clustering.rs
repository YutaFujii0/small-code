type Graph = Vec<(usize, usize, usize)>;

pub fn cluster(graph: Graph, k: usize) {
    
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
