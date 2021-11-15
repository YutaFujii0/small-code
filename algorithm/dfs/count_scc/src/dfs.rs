use std::collections::HashMap;

use super::node::*;

const MAX_NODE: usize = 9;

pub fn dfs_loop(nodes: &Nodes) {
    let mut t: usize = 0;
    let mut exploreds = HashMap::<usize, bool>::new();
    let mut finishing_times = HashMap::<usize, usize>::new();

    for i in 0..MAX_NODE {
        let index = MAX_NODE - i;
        let explored = exploreds.entry(index).or_insert(false);
        if *explored == false {
            dfs(
                nodes,
                index,
                &mut t,
                &mut exploreds,
                &mut finishing_times,
            );
        }
    }

    println!("Finishing times {:?}", finishing_times);
}

// fn dfs_2nd_loop(graph: &Graph) {
//   let mut t: usize = 0;
//   // let mut s: Option<Rc<Node>>  = None;

//   for i in 0..MAX_NODE {
//       let nodes = graph.nodes.borrow();
//       let target = nodes.find_by_finishing_time((MAX_NODE - i) as usize);
//       match target {
//           Some(node) => {
//               if node.borrow().explored == false {
//                   dfs(graph, &node, &node, &mut t);
//               }
//           },
//           None => (),
//       }
//   }
// }

fn dfs(
    nodes: &Nodes,
    target: usize,
    finishing_time: &mut usize,
    exploreds: &mut HashMap<usize, bool>,
    finishing_times: &mut HashMap<usize, usize>,
) {
    exploreds.insert(target, true);
    let node = nodes.get(&target).unwrap();
    for edge in &node.borrow().edges {
        let index = edge.upgrade().unwrap().borrow().id;
        let explored = exploreds.entry(index).or_insert(false);
        if *explored == false {
            dfs(
                nodes,
                index,
                finishing_time,
                exploreds,
                finishing_times
            );
        }
    }
    *finishing_time += 1;
    finishing_times.insert(target, *finishing_time);
}
