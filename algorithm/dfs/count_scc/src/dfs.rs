

// fn dfs_loop(graph: &Graph) {
//   let mut t: usize = 0;
//   // let mut s: Option<Rc<Node>>  = None;

//   for i in 0..MAX_NODE {
//       let nodes = graph.nodes.borrow();
//       let target = nodes.find((MAX_NODE - i) as usize);
//       // target.borrow_mut().explored = true;
//       if target.borrow().explored == false {
//           // s = Some(target);
//           dfs(graph, &target, &target, &mut t);
//       }
//   }
// }

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

// fn dfs(graph: &Graph, target: &Rc<RefCell<Node>>, leader: &Rc<RefCell<Node>>, finishing_time: &mut usize) {
//   target.borrow_mut().explored = true;
//   target.borrow_mut().leader = Rc::downgrade(leader);

//   // for each edge
//   // dfs(graph, node)
//   let edges = graph.edges.borrow();
//   let edges_connected = edge_of(&edges, &target.borrow());

//   for edge in edges_connected {
//       let target= edge.to.upgrade().unwrap();
//       if target.borrow().explored == false {
//           dfs(graph, &target, leader, finishing_time);
//       }
//   }
//   *finishing_time += 1;
//   target.borrow_mut().finishing_time = *finishing_time;
// }
