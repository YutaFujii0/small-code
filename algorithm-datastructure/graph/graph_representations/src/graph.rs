/// Graph trait
/// 
/// A common behavior that a graph have.

pub trait Graph {
    type Vertex;
    type VertexValue;
    type Edge;

    fn add_vertex(&self, vertex: Self::Vertex);

    fn detete_vertex(&mut self, vertex_id: usize);

    fn add_edge(&mut self, edge: Self::Edge);

    fn delete_edge(&mut self, edge: Self::Edge) -> Option<Self::Edge>;

    fn neighbors(&self, vertex_id: usize) -> Vec<&Self::Edge>;

    fn search_vertex(&self, vertex: Self::VertexValue) -> Option<Self::Vertex>;
}