// answer top 5 strongly connected components
// psuedocode
// assign leader node for each vertex
// group by leader node and count then order them descendently

use std::{fs::File, io::Read};
use regex::Regex;
use std::rc::Rc;

const EDGE_PATH: &str = "./dataset.txt";
const MAX_NODE: u32 = 875714;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let mut edges = Edges::new();
    let mut edges_rev = Edges::new();
    let mut nodes = Nodes::new();
    // parse edges.txt
    let mut contents = String::new();
    let mut file = File::open(EDGE_PATH)?;
    file.read_to_string(&mut contents)?;
    // println!("{}", contents);

    for i in 1..MAX_NODE + 1 {
        let node = Node {
            id: i,
            leader: None,
            finishing_time: 0,
            explored: false
        };
        nodes.push(node);
    }
    // create edges, with creation of nodes if neccesary
    let re = Regex::new(r"(?m)^(?P<from>\d+)\s(?P<to>\d+)").expect("Invalid Regex");

    // let mut edges = Vec<Edge>;
    for caps in re.captures_iter(&contents) {
        let node_from = nodes.find(caps["from"].parse::<usize>().unwrap());
        let node_to = nodes.find(caps["to"].parse::<usize>().unwrap());
        edges.push(
            Edge {
                from: Rc::clone(node_from),
                to: Rc::clone(node_to),
            }
        );
        edges_rev.push(
            Edge {
                from: Rc::clone(node_to),
                to: Rc::clone(node_from),
            }
        );
    }

    println!("{:?}", edges.items.len());
    println!("{:?}", edge_of(&edges, &nodes.items[0]).len());
    // two pass algorithm
    // reverse edge
    // dfs loop for Grev

    // dfs loop for G

    // count leader node
    Ok(())
}


#[derive(Debug)]
struct Node {
    id: u32,
    leader: Option<Rc<Node>>,
    finishing_time: usize,
    explored: bool,
}


struct Nodes{
    items: Vec<Rc<Node>>
}

impl Nodes {
    fn new() -> Self {
        Nodes { items: vec![] }
    }

    fn find(&self, index: usize) -> &Rc<Node> {
        &self.items[index - 1]
    }

    fn push(&mut self, item: Node) {
        self.items.push(Rc::new(item));
    }
}

#[derive(Debug)]
struct Edge {
    from: Rc<Node>,
    to: Rc<Node>,
}

struct Edges {
    items: Vec<Edge>
}

impl Edges {
    fn new() -> Self {
        Edges { items: vec![] }
    }

    fn push(&mut self, item: Edge) {
        self.items.push(item);
    }
}

// fn dfs_loop(graph: Nodes) {
//     let mut t: usize = 0;
//     let mut s: Option<Rc<Node>>  = None;

//     for i in 0..MAX_NODE {
//         let target = graph.find((MAX_NODE - i) as usize);
//         if target.explored == false {
//             s = Some(target);
//             dfs(graph, &mut target, target, t);
//         }
//     }
// }

// fn dfs(graph: Nodes, target: &mut Node, leader: &Node, finishing_time: usize) {
//     target.explored = true;
//     target.leader = Some(Box::new(leader));

//     // for each edge
//     // dfs(graph, node)
//     finishing_time += 1;
//     target.finishing_time = finishing_time;
// }

fn edge_of<'a>(edges: &'a Edges, node: &Node) -> Vec<&'a Edge> {
    edges.items
        .iter()
        .filter(|&edge| edge.from.id == node.id)
        .collect::<Vec<&Edge>>()
}
