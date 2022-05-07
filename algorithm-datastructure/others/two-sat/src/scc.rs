use std::collections::HashMap;

use crate::node::{Nodes};

pub struct Solution {}

impl Solution {
    pub fn dfs_loop_first(nodes: &Nodes) -> Result<HashMap::<usize, usize>, String> {
        let mut t: usize = 0;
        let mut exploreds = HashMap::<usize, bool>::new();
        let mut finishing_times = HashMap::<usize, usize>::new();
        let index_max = nodes.keys().max().ok_or_else(|| "Graph seems empty.")?;
    
        for i in 0..*index_max {
            let index = index_max - i;
            let explored = exploreds.entry(index).or_insert(false);
            if *explored == false {
                Solution::dfs(
                    nodes,
                    index,
                    &mut t,
                    &mut exploreds,
                    &mut finishing_times,
                )?;
            }
        }
    
        Ok(finishing_times)
    }
    
    pub fn dfs_loop_second(
        nodes: &Nodes,
        finishing_times: &HashMap::<usize, usize>
    ) -> Result<HashMap::<usize, usize>, String> {
        let mut exploreds = HashMap::<usize, bool>::new();
        let mut leaders = HashMap::<usize, usize>::new();
        let index_max = nodes.keys().max().ok_or_else(|| "Graph seems empty.")?;
    
        for i in 0..*index_max {
            // Since finishing_times are calculated to be consective, we do not expect to get None here.
            let index = finishing_times.get(&(index_max - i)).ok_or_else(|| "Unexpected look-up occured.")?;
            let explored = exploreds.entry(*index).or_insert(false);
            if *explored == false {
                Solution::dfs_second(
                    nodes,
                    *index,
                    *index,
                    &mut exploreds,
                    &mut leaders,
                )?;
            }
        }
    
        Ok(leaders)
    }
    
    fn dfs(
        nodes: &Nodes,
        target: usize,
        finishing_time: &mut usize,
        exploreds: &mut HashMap<usize, bool>,
        finishing_times: &mut HashMap<usize, usize>,
    ) -> Result<(), String> {
        println!("dfs loop for target{:?}", target);
        exploreds.insert(target, true);
        let node = nodes.get(&target).ok_or_else(|| "Unexpected look-up.")?;
        for edge in &node.borrow().edges {
            let index = edge.upgrade()
                .ok_or_else(|| "Referenced value is already dropped.")?
                .borrow()
                .id;
            let explored = exploreds.entry(index).or_insert(false);
            if *explored == false {
                Solution::dfs(
                    nodes,
                    index,
                    finishing_time,
                    exploreds,
                    finishing_times
                )?;
            }
        }
        *finishing_time += 1;
        finishing_times.insert(*finishing_time, target);

        Ok(())
    }
    
    fn dfs_second(
        nodes: &Nodes,
        target: usize,
        leader: usize,
        exploreds: &mut HashMap<usize, bool>,
        leaders: &mut HashMap<usize, usize>,
    ) -> Result<(), String> {
        exploreds.insert(target, true);
        *leaders.entry(target).or_insert(0) = leader;
        let node = nodes.get(&target).ok_or_else(|| "Unexpected look-up.")?;

        for edge in &node.borrow().edges {
            let index = edge.upgrade()
                .ok_or_else(|| "Referenced value is already dropped.")?
                .borrow()
                .id;
            let explored = exploreds.entry(index).or_insert(false);
            if *explored == false {
                Solution::dfs_second(
                    nodes,
                    index,
                    leader,
                    exploreds,
                    leaders
                )?;
            }
        }

        Ok(())
    }
}
