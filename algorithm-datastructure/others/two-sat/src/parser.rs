use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::rc::Rc;
use std::cell::RefCell;

use crate::node::{Node, Nodes};

#[derive(PartialEq, PartialOrd)]
struct Variant {
    value: i32,
}

struct VariantFactory {
    max: i32,
}

impl VariantFactory {
    pub fn new(value: i32) -> Self {
        if value < 0 {
            panic!("VariantFactory received max threthold of negative integer: {}", value);
        }
        VariantFactory { max: value }
    }

    pub fn create(&self, value: i32) -> Variant {
        if value < -1 * self.max || self.max < value {
            panic!("Variant exceeds the limit of {}, given {}", self.max, value);
        }
        Variant { value }
    }
}

pub struct Parser {}

impl Parser {
    pub fn run<T: AsRef<Path>>(path: T) -> Result<(usize, (Nodes, Nodes)), Box<dyn Error>> {
        let (data_count, ret) = Self::parse(path)?;
        let edges = Self::closure_to_edge(ret)?;
        let graphs = Self::graphize(edges)?;
        Ok((data_count, graphs))
    }

    fn parse<T>(path: T) -> Result<(usize, Vec<(Variant, Variant)>), Box<dyn Error>>
    where T: AsRef<Path>
    {
        let mut ret = vec![];
        let file = File::open(path)?;
        let mut lines = BufReader::new(file).lines();

        // first line should be total number of variants
        let data_count = lines.next()
            .ok_or_else(|| "First line is empty, which should be a total number of variants.")?
            .unwrap_or_else(|_| String::from("Unexpected error in reading line"))
            .parse::<usize>()?;

        let factory = VariantFactory::new(i32::try_from(data_count)?);

        for line in lines {
            let line = line?;
            let raw = line.split(" ")
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            ret.push((factory.create(raw[0]), factory.create(raw[1])));
        }
    
        Ok((data_count, ret))
    }
    
    fn closure_to_edge(dataset: Vec<(Variant, Variant)>) -> Result<Vec<(usize, usize)>, Box<dyn Error>> {
        let mut ret: Vec<(usize, usize)> = vec![];
        for (variant_a, variant_b) in dataset {
            let mut a = variant_a.value;
            let mut b = variant_b.value;
            match (a > 0, b > 0) {
                (true, true) => {
                    ret.push((
                        usize::try_from(a*2-1)?,
                        usize::try_from(b*2)?,
                    ));
                    ret.push((
                        usize::try_from(b*2-1)?,
                        usize::try_from(a*2)?,
                    ));
                },
                (true, false) => {
                    b *= -1;
                    ret.push((
                        usize::try_from(a*2-1)?,
                        usize::try_from(b*2-1)?,
                    ));
                    ret.push((
                        usize::try_from(b*2)?,
                        usize::try_from(a*2)?,
                    ));
                },
                (false, true) => {
                    a *= -1;
                    ret.push((
                        usize::try_from(a*2)?,
                        usize::try_from(b*2)?,
                    ));
                    ret.push((
                        usize::try_from(b*2-1)?,
                        usize::try_from(a*2-1)?,
                    ));
                },
                (false, false) => {
                    a *= -1;
                    b *= -1;
                    ret.push((
                        usize::try_from(a*2)?,
                        usize::try_from(b*2-1)?,
                    ));
                    ret.push((
                        usize::try_from(b*2)?,
                        usize::try_from(a*2-1)?,
                    ));
                },
            }
        }
    
        Ok(ret)
    }

    fn graphize(edges: Vec<(usize, usize)>) -> Result<(Nodes, Nodes), Box<dyn Error>> {
        let mut nodes = Nodes::new();
        let mut nodes_rev = Nodes::new();

        for (from_id, to_id) in edges {
            // for G(Graph)
            if nodes.contains_key(&from_id) {
                if !nodes.contains_key(&to_id) {
                    nodes.insert(
                        to_id,
                        Node::new(to_id),
                    );
                }
                let edge_to = nodes.get(&to_id).ok_or_else(|| "Unexpected look-up.")?;
                let e = Rc::downgrade(edge_to);
                let node = nodes.get(&from_id).ok_or_else(|| "Unexpected look-up.")?;
                node.borrow_mut().edges.push(e);
            } else {
                if !nodes.contains_key(&to_id) {
                    nodes.insert(
                        to_id,
                        Node::new(to_id),
                    );
                }
                let edge_to = nodes.get(&to_id).ok_or_else(|| "Unexpected look-up.")?;
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
                let edge_to = nodes_rev.get(&from_id).ok_or_else(|| "Unexpected look-up.")?;
                let e = Rc::downgrade(edge_to);
                let node = nodes_rev.get(&to_id).ok_or_else(|| "Unexpected look-up.")?;
                node.borrow_mut().edges.push(e);
            } else {
                if !nodes_rev.contains_key(&from_id) {
                    nodes_rev.insert(
                        from_id,
                        Node::new(from_id),
                    );
                }
                let edge_to = nodes_rev.get(&from_id).ok_or_else(|| "Unexpected look-up.")?;
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
