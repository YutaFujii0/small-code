mod error;
mod node;
mod parse;
mod scc;

use std::rc::Rc;
use std::cell::RefCell;

use node::*;
use parse::*;
use scc::*;

const FILE_PATH: &str = "dataset-sample.txt";

fn main() {
    println!("2SAT Problem!");
    if let Ok(ret) = parse(FILE_PATH) {
        let (data_count, ret) = ret;
        // println!("{:?}", sat(ret));

        let mut nodes = Nodes::new();
        let mut nodes_rev = Nodes::new();

        for (from_id, to_id) in form(ret) {
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
    
        let finishing_times = dfs_loop_first(&nodes_rev, data_count * 2);
        let leaders = dfs_loop_second(&nodes, &finishing_times, data_count * 2);

        for i in 1..(data_count+1) {
            println!("{} {:?}", i, leaders.get(&(i*2)).unwrap_or(&0) != leaders.get(&(i*2-1)).unwrap_or(&1));
        }
    }
}
