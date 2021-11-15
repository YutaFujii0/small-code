// answer top 5 strongly connected components
// psuedocode
// assign leader node for each vertex
// group by leader node and count then order them descendently

use std::{fs::File, io::Read};
use regex::Regex;
use std::rc::{Rc};
use std::cell::RefCell;

mod node;
mod dfs;

use node::*;
use dfs::*;

const EDGE_PATH: &str = "./dataset_simple.txt";
// const EDGE_PATH: &str = "./dataset.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut nodes = Nodes::new();
    let mut nodes_rev = Nodes::new();
    let mut contents = String::new();
    let mut file = File::open(EDGE_PATH)?;
    file.read_to_string(&mut contents)?;

    let re = Regex::new(r"(?m)^(?P<from>\d+)\s(?P<to>\d+)").expect("Invalid Regex");

    for caps in re.captures_iter(&contents) {
        let from_id = caps["from"].parse::<usize>()?;
        let to_id = caps["to"].parse::<usize>()?;

        // for G(Graph)
        if nodes.contains_key(&from_id) {
            if !nodes.contains_key(&to_id) {
                nodes.insert(
                    to_id,
                    Node::new(to_id),
                );
            }
            let edge_to = nodes.get(&to_id).unwrap();
            let e = Rc::downgrade(edge_to);
            let node = nodes.get(&from_id).unwrap();
            node.borrow_mut().edges.push(e);
        } else {
            if !nodes.contains_key(&to_id) {
                nodes.insert(
                    to_id,
                    Node::new(to_id),
                );
            }
            let edge_to = nodes.get(&to_id).unwrap();
            let e = Rc::downgrade(edge_to);
            let node = Node {
                id: from_id,
                edges: vec![e],
            };
            nodes.insert(from_id, Rc::new(RefCell::new(node)));
        }

        // for Grev
        if nodes_rev.contains_key(&to_id) {
            if !nodes_rev.contains_key(&from_id) {
                nodes_rev.insert(
                    from_id,
                    Node::new(from_id),
                );
            }
            let edge_to = nodes_rev.get(&from_id).unwrap();
            let e = Rc::downgrade(edge_to);
            let node = nodes_rev.get(&to_id).unwrap();
            node.borrow_mut().edges.push(e);
        } else {
            if !nodes_rev.contains_key(&from_id) {
                nodes_rev.insert(
                    from_id,
                    Node::new(from_id),
                );
            }
            let edge_to = nodes_rev.get(&from_id).unwrap();
            let e = Rc::downgrade(edge_to);
            let node = Node {
                id: to_id,
                edges: vec![e],
            };
            nodes_rev.insert(to_id, Rc::new(RefCell::new(node)));
        }
    }

    let finishing_times = dfs_loop_first(&nodes_rev);
    let leaders = dfs_loop_second(&nodes, &finishing_times);
    println!("leaders-nodes {:?}", leaders);

    Ok(())
}

