use std::cmp::PartialEq;
use num::traits::cast::FromPrimitive;

use super::graph::Graph;

#[derive(Debug)]
pub struct Vertex<T: PartialEq> {
    pub value: T,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Weight<T>
where T: FromPrimitive + Clone + PartialEq
{
    Real(T),
    NonExist,
}


#[derive(Debug)]
pub struct AdjacencyMatrix<T, U>
where
    T: PartialEq,
    U: FromPrimitive + Clone + PartialEq,
{
    matrix: Vec<Vec<Weight<U>>>,
    vertices: Vec<Vertex<T>>,
}

impl<T, U> AdjacencyMatrix<T, U>
where
    T: PartialEq,
    U: FromPrimitive + Clone + PartialEq,
{
    fn new() -> Self {
        AdjacencyMatrix { matrix: vec![], vertices: vec![] }
    }
}

impl<T, U> Graph for AdjacencyMatrix<T, U>
where
    T: PartialEq,
    U: FromPrimitive + Clone + PartialEq,
{
    type Vertex = Vertex<T>;
    type VertexValue = T;
    type Edge = (usize, usize, U);
    type EdgeNodes = (usize, usize);

    fn add_vertex(&mut self, value: T) {
        let n = self.vertices.len();
        self.vertices.push(Vertex { value });
        for row in &mut self.matrix {
            row.push(Weight::NonExist);
        }
        
        self.matrix.push(vec![Weight::NonExist; n]);
    }

    fn remove_vertex(&mut self, vertex_index: usize) {
        // Deliberately not using swap_remove because it causes us to swap 
        // corresponding columns of all other rows for consistency.
        self.matrix.remove(vertex_index);
        for row in &mut self.matrix {
            row.remove(vertex_index);
        }
    }

    fn add_edge(&mut self, edge: Self::Edge) {
        self.matrix[edge.0][edge.1] = Weight::Real(edge.2);
    }

    fn remove_edge(&mut self, edge: Self::EdgeNodes) -> Option<Self::Edge> {
        match &mut self.matrix[edge.0][edge.1] {
            Weight::Real(weight) => {
                let weight = weight.clone();
                self.matrix[edge.0][edge.1] = Weight::NonExist;
                return Some((edge.0, edge.1, weight))
            },
            Weight::NonExist => return None,
        }
    }

    /// only neighbor to which there exists an edge
    fn neighbors(&self, vertex_index: usize) -> Vec<&Self::Vertex> {
        let indices = self
            .matrix[vertex_index]
            .iter()
            .enumerate()
            .filter(|&(_, v)| v != &Weight::NonExist)
            .map(|(index, _)| index)
            .collect::<Vec<usize>>();
        indices.iter().map(|&index| &self.vertices[index]).collect::<Vec<&Self::Vertex>>()
    }

    fn find_vertex(&self, vertex_value: Self::VertexValue) -> Option<&Self::Vertex> {
        self.vertices.iter().find(|&vertex| vertex.value == vertex_value)
    }
}