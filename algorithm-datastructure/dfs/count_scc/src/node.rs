use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::collections::HashMap;

pub type RefNode = Rc<RefCell<Node>>;
pub type Nodes = HashMap<usize, RefNode>;

#[derive(Debug)]
pub struct Node {
    // pub leader: RefNode,
    // finishing_time: usize,
    // explored: bool,
    pub id: usize,
    pub edges: Vec<Weak<RefCell<Node>>>,
}

impl Node {
    pub fn new(id: usize) -> RefNode {
        Rc::new(RefCell::new(Node {
            id: id,
            edges: vec![],
        }))
    }
}
