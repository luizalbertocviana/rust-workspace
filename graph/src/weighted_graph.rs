// we use HashMap as a mapping from edges to weights
use std::collections::HashMap;
// we use Default to determine the weight of newly inserted edges
use std::default::Default;
// we use Graph as our internal graph representation
use crate::graph::Graph;
// our weighted graph representation implements GraphImpl
use crate::traits::GraphImpl;
// utility function for adjusting endpoints of an edge
use crate::utils::adjust_endpoints;
// we use these type aliases in some implementations here
use crate::{Edge, Result};
// our weighted graph representation, combining a graph representation
// eith a mapping from edges to weights
pub struct WeightedGraph<W: Default> {
    graph: Graph,
    weight_map: HashMap<Edge, W>,
}
// constructors
impl<W: Default> WeightedGraph<W> {
    /// returns an WeightedGraph with num_verts vertices and no edges
    pub fn new(num_verts: usize) -> Self {
        let graph = Graph::new(num_verts);
        let weight_map = HashMap::new();

        Self { graph, weight_map }
    }
}
// GraphImpl implementation
impl<'a, W: Default> GraphImpl<'a> for WeightedGraph<W> {
    type EdgeIterator = crate::graph::EdgeIterator<'a>;

    fn num_verts(&self) -> usize {
        self.graph.num_verts()
    }

    fn num_edges(&self) -> usize {
        self.graph.num_edges()
    }

    fn has_edge(&self, u: usize, v: usize) -> bool {
        self.graph.has_edge(u, v)
    }

    fn edges(&'a self) -> Self::EdgeIterator {
        self.graph.edges()
    }

    fn add_edge(&mut self, mut i: usize, mut j: usize) -> Result {
        if self.graph.add_edge(i, j).is_ok() {
            adjust_endpoints(&mut i, &mut j);
            self.weight_map.insert((i, j), Default::default());

            Ok(())
        } else {
            Err("WeightedGraph: attempt to add existing edge")
        }
    }

    fn remove_edge(&mut self, mut i: usize, mut j: usize) -> Result {
        if self.graph.remove_edge(i, j).is_ok() {
            adjust_endpoints(&mut i, &mut j);
            self.weight_map.remove(&(i, j));

            Ok(())
        } else {
            Err("WeightedGraph: attempt to remove nonexisting edge")
        }
    }
}

// weight related functions
impl<W: Default> WeightedGraph<W> {
    /// gets weight of edge (i, j) ((j, i), in case i > j). Returns
    /// None in case edge does not exist
    pub fn get_edge_weight(&self, mut i: usize, mut j: usize) -> Option<&W> {
        adjust_endpoints(&mut i, &mut j);
        self.weight_map.get(&(i, j))
    }
    /// sets weight of edge (i, j) to w. Returns old weight in case
    /// edge exists, otherwise returns None
    pub fn set_edge_weight(&mut self, mut i: usize, mut j: usize, w: W) -> Option<W> {
        if self.graph.has_edge(i, j) {
            adjust_endpoints(&mut i, &mut j);

            self.weight_map.insert((i, j), w)
        } else {
            None
        }
    }
}
