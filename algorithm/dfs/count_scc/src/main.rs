// answer top 5 strongly connected components
// psuedocode
// assign leader node for each vertex
// group by leader node and count then order them descendently

use std::{fs::File, io::Read};
use regex::Regex;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

mod node;
mod dfs;

use node::*;
// use dfs::*;

const EDGE_PATH: &str = "./dataset_simple.txt";
// const EDGE_PATH: &str = "./dataset.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut nodes = Nodes::new();
    let mut contents = String::new();
    let mut file = File::open(EDGE_PATH)?;
    file.read_to_string(&mut contents)?;

    let re = Regex::new(r"(?m)^(?P<from>\d+)\s(?P<to>\d+)").expect("Invalid Regex");

    for caps in re.captures_iter(&contents) {
        let from_id = caps["from"].parse::<usize>()?;
        let to_id = caps["to"].parse::<usize>()?;

        if nodes.contains_key(&from_id) {
            if !nodes.contains_key(&to_id) {
                nodes.insert(
                    to_id,
                    Node::new(),
                );
            }
            let edge_to = nodes.get(&to_id).unwrap();
            let e = Rc::downgrade(edge_to);
            let node = nodes.get(&from_id).unwrap();
            println!("{:?}", node);
            node.borrow_mut().edges.push(e);
        } else {
            if !nodes.contains_key(&to_id) {
                nodes.insert(
                    to_id,
                    Node::new(),
                );
            }
            let edge_to = nodes.get(&to_id).unwrap();
            // let e = Rc::downgrade(&edge_to.upgrade().unwrap());
            let e = Rc::downgrade(edge_to);
            let node = Node {
                edges: vec![e],
            };
            nodes.insert(from_id, Rc::new(RefCell::new(node)));
        }
    }

    println!("{:?}", nodes);
    // dfs_loop(&graph);
    // graph.edges = RefCell::new(edges);
    // graph.nodes.borrow().reset_explored();
    // dfs_2nd_loop(&graph);
    // for n in &graph.nodes.borrow().items {
    //     println!("{:?}-leader:{:?}", n, n.borrow().leader.upgrade().unwrap().borrow().id);
    // }

    Ok(())
}

