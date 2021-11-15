use std::collections::HashMap;

use super::node::*;

// const MAX_NODE: usize = 9;
const MAX_NODE: usize = 875714;

pub fn dfs_loop_first(nodes: &Nodes) -> HashMap::<usize, usize> {
    let mut t: usize = 0;
    let mut exploreds = HashMap::<usize, bool>::new();
    let mut finishing_times = HashMap::<usize, usize>::new();

    for i in 0..MAX_NODE {
        let index = MAX_NODE - i;
        let explored = exploreds.entry(index).or_insert(false);
        if *explored == false {
            dfs(
                nodes,
                index,
                &mut t,
                &mut exploreds,
                &mut finishing_times,
            );
        }
    }

    finishing_times
}

pub fn dfs_loop_second(nodes: &Nodes, finishing_times: &HashMap::<usize, usize>) -> HashMap::<usize, usize> {
    let mut exploreds = HashMap::<usize, bool>::new();
    let mut leaders = HashMap::<usize, usize>::new();

    for i in 0..MAX_NODE {
        let index = finishing_times.get(&(MAX_NODE - i)).unwrap();
        let explored = exploreds.entry(*index).or_insert(false);
        if *explored == false {
            dfs_second(
                nodes,
                *index,
                *index,
                &mut exploreds,
                &mut leaders,
            );
        }
    }

    leaders
}

fn dfs(
    nodes: &Nodes,
    target: usize,
    finishing_time: &mut usize,
    exploreds: &mut HashMap<usize, bool>,
    finishing_times: &mut HashMap<usize, usize>,
) {
    println!("dfs loop for target{:?}", target);
    exploreds.insert(target, true);
    let node = nodes.get(&target).unwrap();
    for edge in &node.borrow().edges {
        let index = edge.upgrade().unwrap().borrow().id;
        let explored = exploreds.entry(index).or_insert(false);
        if *explored == false {
            dfs(
                nodes,
                index,
                finishing_time,
                exploreds,
                finishing_times
            );
        }
    }
    *finishing_time += 1;
    finishing_times.insert(*finishing_time, target);
}

fn dfs_second(
    nodes: &Nodes,
    target: usize,
    leader: usize,
    exploreds: &mut HashMap<usize, bool>,
    leaders: &mut HashMap<usize, usize>,
) {
    exploreds.insert(target, true);
    *leaders.entry(leader).or_insert(0) += 1;
    let node = nodes.get(&target).unwrap();
    for edge in &node.borrow().edges {
        let index = edge.upgrade().unwrap().borrow().id;
        let explored = exploreds.entry(index).or_insert(false);
        if *explored == false {
            dfs_second(
                nodes,
                index,
                leader,
                exploreds,
                leaders
            );
        }
    }
}
