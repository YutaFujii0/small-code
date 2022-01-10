use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

use std::collections::HashMap;
use super::mst::*;

pub fn parse<T>(path: T) -> Result<Vec<(usize, usize, i64)>, Box<dyn std::error::Error>>
where T: AsRef<Path>
{
    let file = File::open(path)?;
    let re = Regex::new(r"(?P<node1>\d+)\s(?P<node2>\d+)\s(?P<cost>-?\d+)").expect("invalid regex");
    let mut results = vec![];

    for line in BufReader::new(file).lines() {
        let line = line?;
        for caps in re.captures_iter(&line) {
            let node1 = caps["node1"].parse::<usize>()?;
            let node2 = caps["node2"].parse::<usize>()?;
            let cost = caps["cost"].parse::<i64>()?;
            results.push((node1, node2, cost));
        }
    }
    Ok(results)
}

pub fn graphize(dataset: Vec<(usize, usize, i64)>) -> Graph
{
    let mut nodes = Vec::<usize>::new();
    let mut edges = HashMap::<usize, Vec<Edge>>::new();
    for (node1, node2, cost) in dataset {
        if nodes.iter().find(|&n| n == &node1) == None {
            nodes.push(node1);
        }
        if nodes.iter().find(|&n| n == &node2) == None {
            nodes.push(node2);
        }
        let edge_of1 = edges.entry(node1).or_insert(vec![]);
        edge_of1.push(Edge { node: node2, cost: cost });
        let edge_of2 = edges.entry(node2).or_insert(vec![]);
        edge_of2.push(Edge { node: node1, cost: cost });
    }
    Graph { nodes: nodes, edges: edges }
}
