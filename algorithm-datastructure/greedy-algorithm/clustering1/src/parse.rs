use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
use std::collections::VecDeque;

type Graph = VecDeque<(usize, usize, usize)>;

pub fn parse<T>(path: T) -> Result<VecDeque<(usize, usize, usize)>, Box<dyn std::error::Error>>
where T: AsRef<Path>
{
    let file = File::open(path)?;
    let mut result = VecDeque::new();
    let re = Regex::new(r"(?P<node1>\d+)\s(?P<node2>\d+)\s(?P<distance>\d+)").expect("invalid regex");
    for line in BufReader::new(file).lines() {
        let line = line?;
        for caps in re.captures_iter(&line) {
            let node1 = caps["node1"].parse::<usize>()?;
            let node2 = caps["node2"].parse::<usize>()?;
            let distance = caps["distance"].parse::<usize>()?;
            result.push_back((node1, node2, distance));
        }
    }
    Ok(result)
}

pub fn preprocess(graph: Graph) -> Graph {
    merge_sort(graph)
}

fn merge_sort(mut graph: Graph) -> Graph {
    if graph.len() == 1 {
        return graph.clone()
    }
    let half = graph.len() / 2;
    let right = merge_sort(graph.split_off(half));
    let left = merge_sort(graph);

    merge(left, right)
}

fn merge(mut left: Graph, mut right: Graph) -> Graph {
    let mut graph = VecDeque::new();
    while left.len() > 0 && right.len() > 0 {
        if left[0].2 > right[0].2 {
            if let Some(edge) = right.pop_front() {
                graph.push_back(edge);
            }
        } else {
            if let Some(edge) = left.pop_front() {
                graph.push_back(edge);
            }
        }
    }
    if left.len() > 0 {
        graph.append(&mut left);
    }
    if right.len() > 0 {
        graph.append(&mut right);
    }
    graph
}
