use std::collections::HashSet;

use crate::graph::Graph;

use crate::Edge;
use crate::Result;

pub struct Subgraph<'a> {
    parent: &'a Graph,

    included_edges: HashSet<Edge>,
    removed_edges: HashSet<Edge>,
}

// constructors
impl<'a> Subgraph<'a> {
    pub fn from_graph(parent: &'a Graph) -> Self {
        let included_edges = HashSet::new();
        let removed_edges = HashSet::new();

        Self {
            parent,
            included_edges,
            removed_edges,
        }
    }
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

// accessors
impl<'a> Subgraph<'a> {
    pub fn num_verts(&self) -> usize {
        self.parent.num_verts()
    }

    pub fn num_edges(&self) -> usize {
        let parent_edges = self.parent.num_edges();
        let plus_edges = self.included_edges.len();
        let minus_edges = self.removed_edges.len();

        parent_edges + plus_edges - minus_edges
    }

    pub fn has_edge(&self, i: usize, j: usize) -> bool {
        let edge = (i, j);

        if self.included_edges.contains(&edge) {
            true
        } else if self.removed_edges.contains(&edge) {
            false
        } else {
            self.parent.has_edge(i, j)
        }
    }
}

// modifiers
impl<'a> Subgraph<'a> {
    pub fn add_edge(&mut self, i: usize, j: usize) -> Result {
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

    pub fn remove_edge(&mut self, i: usize, j: usize) -> Result {
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
