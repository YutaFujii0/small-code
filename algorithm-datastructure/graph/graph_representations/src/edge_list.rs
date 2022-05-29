use super::graph::Graph;

/// Graph - Edge List representation
///
/// # Examples
///
/// ```
/// fn compare(parent: &i32, child: &i32) -> bool {
///     parent < child
/// }
///
/// let mut heap = datastructures::HeapNode::new(compare);
/// heap.insert(3);
/// heap.insert(5);
/// heap.insert(1);
/// let poll = heap.poll();
/// assert_eq!(5, poll.unwrap());
/// ```

#[allow(dead_code)]
pub struct Edge<T> {
    left: usize,
    right: usize,
    weight: T,
}

pub struct EdgeList<T> {
    edges: Vec<Edge<T>>,
}

impl<T> Graph for EdgeList<T> {
    type Vertex = usize;
    type VertexValue = usize;
    type Edge = Edge<T>;

    fn add_vertex(&self, _vertex: Self::Vertex) {
        unimplemented!();
    }

    fn detete_vertex(&mut self, vertex_id: usize) {
        let result = self.edges.iter()
            .enumerate()
            .filter(|&(_, edge)| edge.left == vertex_id || edge.right == vertex_id)
            .map(|(index, _)| index)
            .collect::<Vec<usize>>();
        
        // Iterate from the latter index to detele all with `swap_remove`
        for index in 0..result.len() {
            self.edges.swap_remove(result.len() - index);
        }
    }

    fn add_edge(&mut self, edge: Self::Edge) {
        self.edges.push(edge);
    }

    fn delete_edge(&mut self, target: Self::Edge) -> Option<Self::Edge> {
        let result = self.edges.iter()
            .enumerate()
            .find(|&(_, edge)| edge.left == target.left && edge.right == target.right);
        match result {
            Some((index, _)) => {
                let edge = self.edges.swap_remove(index);
                return Some(edge)
            },
            None => return None,
        }
    }

    fn neighbors(&self, vertex_id: usize) -> Vec<&Self::Edge> {
        self.edges.iter()
            .filter(|&edge| edge.left == vertex_id || edge.right == vertex_id)
            .collect::<Vec<&Edge<T>>>()
    }

    fn search_vertex(&self, vertex_id: Self::VertexValue) -> Option<Self::Vertex> {
        let edge = self.edges.iter()
            .find(|&edge| edge.left == vertex_id || edge.right == vertex_id);
        match edge {
            Some(_edge) => return Some(vertex_id),
            None => return None,
        }
    }
}
