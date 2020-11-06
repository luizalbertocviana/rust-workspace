// we use HashSet to represent subgraph edges
use std::collections::hash_set;
use std::collections::HashSet;
// we use these for some iterator combination
use std::iter::{Chain, Copied};
// Subgraph (and its related types) implements these traits
use crate::traits::{EdgeIterable, GraphImpl};
// currently we implement Subgraph as subgraphs of Graph type. We see
// this can be further generalized
use crate::graph::Graph;
// some type aliases we use in these implementations
use crate::{Edge, Result};
/// represents a subgraph of an undirected graph
pub struct Subgraph<'a> {
    // supergraph reference
    parent: &'a Graph,
    // edges (either included or removed) that distinguish this
    // subgraph from its supergraph
    included_edges: HashSet<Edge>,
    removed_edges: HashSet<Edge>,
}
// constructors
impl<'a> Subgraph<'a> {
    /// returns a subgraph of parent with the same vertex and edge
    /// sets
    pub fn from_graph(parent: &'a Graph) -> Self {
        let included_edges = HashSet::new();
        let removed_edges = HashSet::new();

        Self {
            parent,
            included_edges,
            removed_edges,
        }
    }
    /// returns a subgraph of parent with the same vertex and edge
    /// sets
    pub fn from_subgraph(parent: &Self) -> Self {
        let included_edges = parent.included_edges.clone();
        let removed_edges = parent.removed_edges.clone();
        let parent = parent.parent;

        Self {
            parent,
            included_edges,
            removed_edges,
        }
    }
}
// GraphImpl implementation
impl<'a> GraphImpl<'a> for Subgraph<'a> {
    type EdgeIterator = EdgeIterator<'a>;

    fn num_verts(&self) -> usize {
        self.parent.num_verts()
    }

    fn num_edges(&self) -> usize {
        let parent_edges = self.parent.num_edges();
        let plus_edges = self.included_edges.len();
        let minus_edges = self.removed_edges.len();

        parent_edges + plus_edges - minus_edges
    }

    fn has_edge(&self, i: usize, j: usize) -> bool {
        let edge = (i, j);

        if self.included_edges.contains(&edge) {
            true
        } else if self.removed_edges.contains(&edge) {
            false
        } else {
            self.parent.has_edge(i, j)
        }
    }

    fn edges(&self) -> EdgeIterator {
        EdgeIterator::new(self)
    }

    fn add_edge(&mut self, i: usize, j: usize) -> Result {
        let edge = (i, j);

        let rem_edges = &mut self.removed_edges;
        let inc_edges = &mut self.included_edges;

        let error = Err("Subgraph: attempt to add an existent edge");

        if rem_edges.contains(&edge) {
            rem_edges.remove(&edge);

            Ok(())
        } else if self.parent.has_edge(i, j) {
            error
        } else {
            if inc_edges.contains(&edge) {
                error
            } else {
                inc_edges.insert(edge);

                Ok(())
            }
        }
    }

    fn remove_edge(&mut self, i: usize, j: usize) -> Result {
        let edge = (i, j);

        let rem_edges = &mut self.removed_edges;
        let inc_edges = &mut self.included_edges;

        let error = Err("Subgraph: attempt to remove a nonexistent edge");

        if inc_edges.contains(&edge) {
            inc_edges.remove(&edge);

            Ok(())
        } else if !self.parent.has_edge(i, j) {
            error
        } else {
            if rem_edges.contains(&edge) {
                error
            } else {
                rem_edges.insert(edge);

                Ok(())
            }
        }
    }
}
/// controls iteration through the edges of a Subgraph
pub struct EdgeIterator<'a> {
    // reference to the Subgraph whose edges are being iterated through
    parent: &'a Subgraph<'a>,

    current_pair: (usize, usize),
    // iterator that combines iterators of the included edges HashSet
    // and the Subgraph
    edge_it: Chain<Copied<hash_set::Iter<'a, Edge>>, crate::graph::EdgeIterator<'a>>,
}
// constructors
impl<'a> EdgeIterator<'a> {
    // returns a new EdgeIterator
    fn new(parent: &'a Subgraph) -> Self {
        let mut edge_it = parent.included_edges.iter().copied().chain(parent.parent.edges());
        let current_pair = match edge_it.next() {
            Some(pair) => pair,
            None => (parent.num_verts(), 0),
        };

        Self {parent, current_pair, edge_it}
    }
}
// EdgeIterable implementation
impl<'a> EdgeIterable<'a> for EdgeIterator<'a> {
    type Parent = Subgraph<'a>;

    fn parent(&self) -> &Subgraph<'a> {
        self.parent
    }

    fn current_pair(&self) -> (usize, usize) {
        self.current_pair
    }

    fn next_pair(&mut self) {
        if let Some(pair) = self.edge_it.next() {
            if self.parent.removed_edges.contains(&pair) {
                self.next_pair()
            } else {
                self.current_pair = pair;
            }
        } else {
            self.current_pair = (self.parent.num_verts(), 0);
        }
    }
}
// Iterator implementation for EdgeIterator
impl<'a> Iterator for EdgeIterator<'a> {
    type Item = Edge;

    fn next(&mut self) -> Option<Edge> {
        self.next_edge()
    }
}
