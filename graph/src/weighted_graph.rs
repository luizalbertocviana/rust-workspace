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
    /// returns a WeightedGraph with num_verts vertices and no edges
    pub fn new(num_verts: usize) -> Self {
        let graph = Graph::new(num_verts);
        let weight_map = HashMap::new();

        Self { graph, weight_map }
    }
    /// returns a WeightedGraph with num_verts vertices and edges
    pub fn from_weighted_edges(num_verts: usize, edges: impl IntoIterator<Item = (usize, usize, W)>) -> Self {
        let mut wg = Self::new(num_verts);

        for (u, v, w) in edges {
            wg.add_weighted_edge(u, v, w).unwrap();
        }

        wg
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

    fn add_edge(&mut self, i: usize, j: usize) -> Result {
        if self.graph.add_edge(i, j).is_ok() {
            let mut edge = (i, j);
            adjust_endpoints(&mut edge.0, &mut edge.1);

            self.weight_map.insert((edge.0, edge.1), Default::default());

            Ok(())
        } else {
            Err("WeightedGraph: attempt to add existing edge")
        }
    }

    fn remove_edge(&mut self, i: usize, j: usize) -> Result {
        if self.graph.remove_edge(i, j).is_ok() {
            let mut edge = (i, j);
            adjust_endpoints(&mut edge.0, &mut edge.1);

            self.weight_map.remove(&(edge.0, edge.1));

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
    pub fn get_edge_weight(&self, i: usize, j: usize) -> Option<&W> {
        let mut edge = (i, j);
        adjust_endpoints(&mut edge.0, &mut edge.1);

        self.weight_map.get(&(edge.0, edge.1))
    }
    /// sets weight of edge (i, j) to w. Returns old weight in case
    /// edge exists, otherwise returns None
    pub fn set_edge_weight(&mut self, i: usize, j: usize, w: W) -> Option<W> {
        if self.graph.has_edge(i, j) {
            let mut edge = (i, j);
            adjust_endpoints(&mut edge.0, &mut edge.1);

            self.weight_map.insert((edge.0, edge.1), w)
        } else {
            None
        }
    }
}
