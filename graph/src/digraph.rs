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

    fn edges(&'a self) -> EdgeIterator<'a> {
        EdgeIterator::new(self)
    }
}

// modifiers
impl Digraph {
    pub fn add_edge(&mut self, i: usize, j: usize) -> Result {
        if self.has_edge(i, j) {
            Err("Digraph: attempting to add existent edge")
        } else {
            *self.data.at(i, j) = true;
            self.num_edges += 1;

            Ok(())
        }
    }

    pub fn remove_edge(&mut self, i: usize, j: usize) -> Result {
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

    current_i: usize,
    current_j: usize,
}

// constructors
impl<'a> EdgeIterator<'a> {
    fn new(parent: &'a Digraph) -> Self {
        let current_i = 0;
        let current_j = 0;

        Self {
            parent,
            current_i,
            current_j,
        }
    }
}

impl<'a> EdgeIterable<'a> for EdgeIterator<'a> {
    type Parent = Digraph;

    fn parent(&self) -> &Digraph {
        self.parent
    }

    fn current_pair(&self) -> (usize, usize) {
        (self.current_i, self.current_j)
    }

    fn next_pair(&mut self) {
        self.current_j += 1;

        if self.current_j == self.parent.num_verts() {
            self.current_j = 0;
            self.current_i += 1;
        }
    }
}

impl<'a> Iterator for EdgeIterator<'a> {
    type Item = Edge;

    fn next(&mut self) -> Option<Edge> {
        self.next_edge()
    }
}
