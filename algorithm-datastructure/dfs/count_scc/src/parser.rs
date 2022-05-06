use std::error::Error;
use std::path::Path;
use std::fs::File;
use std::io::{self, Read};
use std::cell::RefCell;
use std::rc::{Rc};

use regex::Regex;

use crate::node::{Node, Nodes};

pub struct Parser<T: AsRef<Path>> {
    path: T
}

impl<T: AsRef<Path>> Parser<T> {
    pub fn new(path: T) -> Self {
        Parser { path }
    }

    pub fn call(&self) -> Result<(Nodes, Nodes), Box<dyn Error>> {
        let contents = self.parse()?;
        let nodes = self.format(contents)?;
        Ok(nodes)
    }

    fn parse(&self) -> io::Result<String> {
        let mut file = File::open(&self.path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Ok(contents)
    }

    fn format(&self, contents: String) -> Result<(Nodes, Nodes), Box<dyn Error>> {
        let mut nodes = Nodes::new();
        let mut nodes_rev = Nodes::new();

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
                let edge_to = nodes.get(&to_id).ok_or_else(|| "Unexpected key.")?;
                let e = Rc::downgrade(edge_to);
                let node = nodes.get(&from_id).ok_or_else(|| "Unexpected key.")?;
                node.borrow_mut().edges.push(e);
            } else {
                if !nodes.contains_key(&to_id) {
                    nodes.insert(
                        to_id,
                        Node::new(to_id),
                    );
                }
                let edge_to = nodes.get(&to_id).ok_or_else(|| "Unexpected key.")?;
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
                let edge_to = nodes_rev.get(&from_id).ok_or_else(|| "Unexpected key.")?;
                let e = Rc::downgrade(edge_to);
                let node = nodes_rev.get(&to_id).ok_or_else(|| "Unexpected key.")?;
                node.borrow_mut().edges.push(e);
            } else {
                if !nodes_rev.contains_key(&from_id) {
                    nodes_rev.insert(
                        from_id,
                        Node::new(from_id),
                    );
                }
                let edge_to = nodes_rev.get(&from_id).ok_or_else(|| "Unexpected key.")?;
                let e = Rc::downgrade(edge_to);
                let node = Node {
                    id: to_id,
                    edges: vec![e],
                };
                nodes_rev.insert(to_id, Rc::new(RefCell::new(node)));
            }
        }

        Ok((nodes, nodes_rev))
    } 
}