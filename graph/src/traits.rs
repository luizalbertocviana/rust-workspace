// we use these type aliases to define some of these traits
use crate::{Edge, Result};
/// a common interface to our graph representations
pub trait GraphImpl<'a> {
    /// type that provides iteration control through the edges of a
    /// graph representation
    type EdgeIterator: Iterator<Item = Edge>;
    /// returns number of vertices
    fn num_verts(&self) -> usize;
    /// returns number of edges
    fn num_edges(&self) -> usize;
    /// determines whether graph representation has edge with
    /// endpoints u and v
    fn has_edge(&self, u: usize, v: usize) -> bool;
    /// returns an iterator through the edges of graph representation
    fn edges(&'a self) -> Self::EdgeIterator;
    /// adds edge with endpoints i and j to graph representation. In
    /// case edge already exists, an error is returned
    fn add_edge(&mut self, i: usize, j: usize) -> Result;
    /// removes edge with endpoints i and j from graph
    /// representation. In case edge does not exist, an error is returned
    fn remove_edge(&mut self, i: usize, j: usize) -> Result;
}
/// interface to types responsible for controlling edge iteration
pub trait EdgeIterable<'a> {
    /// type of graph representation this type relates to
    type Parent: GraphImpl<'a>;
    /// returns reference to the graph representation whose edges are
    /// being iterated through
    fn parent(&self) -> &Self::Parent;
    /// returns current edge candidate. Current pair needs to be
    /// verified as an actual edge before being returned to next_edge
    fn current_pair(&self) -> (usize, usize);
    /// computes next edge candidate to be considered
    fn next_pair(&mut self);
    /// determines whether iteration process has ended
    fn stop(&self) -> bool {
        let (i, _) = self.current_pair();
        // we convention that whenever current edge candidate has
        // num_verts (minimum invalid vertex index) as its first
        // endpoint, no more edge candidates should be considered
        i == self.parent().num_verts()
    }
    /// returns either next edge or None, in case iteration process is
    /// over.
    fn next_edge(&mut self) -> Option<Edge> {
        // iteration process has ended
        if self.stop() {
            None
        } else {
            // otherwise copy current pair
            let pair = self.current_pair();
            // compute next
            self.next_pair();
            // if current pair is an edge, return it
            if self.parent().has_edge(pair.0, pair.1) {
                Some(pair)
            } else {
                // otherwise get next edge recursively
                self.next_edge()
            }
        }
    }
}
