use matrix::Matrix;

use crate::traits::{EdgeIterable, GraphImpl};

use crate::{Edge, Result};

pub struct Digraph {
    data: Matrix<bool>,

    num_verts: usize,
    num_edges: usize,
}

// constructors
impl Digraph {
    pub fn new(num_verts: usize) -> Self {
        let data = Matrix::square(num_verts);
        let num_edges = 0;

        Self {
            data,
            num_verts,
            num_edges,
        }
    }
}

impl<'a> GraphImpl<'a> for Digraph {
    type EdgeIterator = EdgeIterator<'a>;

    fn num_verts(&self) -> usize {
        self.num_verts
    }

    fn num_edges(&self) -> usize {
        self.num_edges
    }

    fn has_edge(&self, i: usize, j: usize) -> bool {
        *self.data.const_at(i, j)
    }

    fn edges(&'a self) -> EdgeIterator {
        EdgeIterator::new(self)
    }

    fn add_edge(&mut self, i: usize, j: usize) -> Result {
        if self.has_edge(i, j) {
            Err("Digraph: attempting to add existent edge")
        } else {
            *self.data.at(i, j) = true;
            self.num_edges += 1;

            Ok(())
        }
    }

    fn remove_edge(&mut self, i: usize, j: usize) -> Result {
        if self.has_edge(i, j) {
            *self.data.at(i, j) = false;
            self.num_edges -= 1;

            Ok(())
        } else {
            Err("Digraph: attempting to remove nonexistent edge")
        }
    }
}

pub struct EdgeIterator<'a> {
    parent: &'a Digraph,

    current_pair: (usize, usize),
}

// constructors
impl<'a> EdgeIterator<'a> {
    fn new(parent: &'a Digraph) -> Self {
        let current_pair = (0, 0);

        Self {
            parent,
            current_pair,
        }
    }
}

impl<'a> EdgeIterable<'a> for EdgeIterator<'a> {
    type Parent = Digraph;

    fn parent(&self) -> &Digraph {
        self.parent
    }

    fn current_pair(&self) -> (usize, usize) {
        self.current_pair
    }

    fn next_pair(&mut self) {
        self.current_pair.1 += 1;

        if self.current_pair.1 == self.parent.num_verts() {
            self.current_pair.1 = 0;
            self.current_pair.0 += 1;
        }
    }
}

impl<'a> Iterator for EdgeIterator<'a> {
    type Item = Edge;

    fn next(&mut self) -> Option<Edge> {
        self.next_edge()
    }
}
