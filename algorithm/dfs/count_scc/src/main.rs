// answer top 5 strongly connected components
// psuedocode
// assign leader node for each vertex
// group by leader node and count then order them descendently

use std::{fs::File, io::Read};
use regex::Regex;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

const EDGE_PATH: &str = "./dataset_simple.txt";
const MAX_NODE: u32 = 9;
// const EDGE_PATH: &str = "./dataset.txt";
// const MAX_NODE: u32 = 875714;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut edges = Edges::new();
    let edges_rev = Edges::new();
    let nodes = Nodes::new();
    let mut graph = Graph { nodes: RefCell::new(nodes), edges: RefCell::new(edges_rev) };
    // parse edges.txt
    let mut contents = String::new();
    let mut file = File::open(EDGE_PATH)?;
    file.read_to_string(&mut contents)?;

    for i in 1..MAX_NODE + 1 {
        let node = Node {
            id: i,
            leader: Weak::new(),
            finishing_time: 0,
            explored: false,
        };
        graph.nodes.borrow_mut().push(RefCell::new(node));
    }
    // create edges, with creation of nodes if neccesary
    let re = Regex::new(r"(?m)^(?P<from>\d+)\s(?P<to>\d+)").expect("Invalid Regex");

    // let mut edges = Vec<Edge>;
    for caps in re.captures_iter(&contents) {
        let nodes = graph.nodes.borrow();
        let node_from = Rc::new(nodes.find(caps["from"].parse::<usize>().unwrap()));
        let node_to = Rc::new(nodes.find(caps["to"].parse::<usize>().unwrap()));
        edges.push(
            Edge {
                from: Rc::downgrade(&node_from),
                to: Rc::downgrade(&node_to),
            }
        );
        graph.edges.borrow_mut().push(
            Edge {
                from: Rc::downgrade(&node_to),
                to: Rc::downgrade(&node_from),
            }
        );
    }

    dfs_loop(&graph);
    graph.edges = RefCell::new(edges);
    graph.nodes.borrow().reset_explored();
    dfs_2nd_loop(&graph);
    // for n in &graph.nodes.borrow().items {
    //     println!("{:?}-leader:{:?}", n, n.borrow().leader.upgrade().unwrap().borrow().id);
    // }

    // count leader node
    Ok(())
}


#[derive(Debug)]
struct Node {
    id: u32,
    leader: Weak<RefCell<Node>>,
    finishing_time: usize,
    explored: bool,
}

struct Nodes{
    items: Vec<Rc<RefCell<Node>>>
}

impl Nodes {
    fn new() -> Self {
        Nodes { items: vec![] }
    }

    fn find(&self, index: usize) -> &Rc<RefCell<Node>> {
        &self.items[index - 1]
    }

    fn find_by_finishing_time(&self, index: usize) -> Option<&Rc<RefCell<Node>>> {
        self.items.iter().find(|&el| el.borrow().finishing_time == index)
    }

    fn push(&mut self, item: RefCell<Node>) {
        self.items.push(Rc::new(item));
    }

    fn reset_explored(&self) {
        for node in &self.items {
            node.borrow_mut().explored = false;
        }
    }
}

#[derive(Debug)]
struct Edge {
    from: Weak<RefCell<Node>>,
    to: Weak<RefCell<Node>>,
}

struct Edges {
    items: Vec<Edge>
}

struct Graph {
    nodes: RefCell<Nodes>,
    edges: RefCell<Edges>,
}

impl Edges {
    fn new() -> Self {
        Edges { items: vec![] }
    }

    fn push(&mut self, item: Edge) {
        self.items.push(item);
    }
}

fn dfs_loop(graph: &Graph) {
    let mut t: usize = 0;
    // let mut s: Option<Rc<Node>>  = None;

    for i in 0..MAX_NODE {
        let nodes = graph.nodes.borrow();
        let target = nodes.find((MAX_NODE - i) as usize);
        // target.borrow_mut().explored = true;
        if target.borrow().explored == false {
            // s = Some(target);
            dfs(graph, &target, &target, &mut t);
        }
    }
}

fn dfs_2nd_loop(graph: &Graph) {
    let mut t: usize = 0;
    // let mut s: Option<Rc<Node>>  = None;

    for i in 0..MAX_NODE {
        let nodes = graph.nodes.borrow();
        let target = nodes.find_by_finishing_time((MAX_NODE - i) as usize);
        match target {
            Some(node) => {
                if node.borrow().explored == false {
                    dfs(graph, &node, &node, &mut t);
                }
            },
            None => (),
        }
    }
}

fn dfs(graph: &Graph, target: &Rc<RefCell<Node>>, leader: &Rc<RefCell<Node>>, finishing_time: &mut usize) {
    target.borrow_mut().explored = true;
    target.borrow_mut().leader = Rc::downgrade(leader);

    // for each edge
    // dfs(graph, node)
    let edges = graph.edges.borrow();
    let edges_connected = edge_of(&edges, &target.borrow());

    for edge in edges_connected {
        let target= edge.to.upgrade().unwrap();
        if target.borrow().explored == false {
            dfs(graph, &target, leader, finishing_time);
        }
    }
    *finishing_time += 1;
    target.borrow_mut().finishing_time = *finishing_time;
}

fn edge_of<'a>(edges: &'a Edges, node: &Node) -> Vec<&'a Edge> {
    edges.items
        .iter()
        .filter(|&edge| edge.from.upgrade().unwrap().borrow().id == node.id)
        .collect::<Vec<&Edge>>()
}
