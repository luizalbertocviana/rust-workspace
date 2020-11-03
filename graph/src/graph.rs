use std::mem::swap;

use matrix::UpperTriangularMatrix;

use crate::traits::{EdgeIterable, GraphImpl};

use crate::{Result, Edge};

pub struct Graph {
    data: UpperTriangularMatrix<bool>,

    num_verts: usize,
    num_edges: usize,
}

// constructors
impl Graph {
    pub fn new(num_verts: usize) -> Self {
        let data = UpperTriangularMatrix::new(num_verts);
        let num_edges = 0;

        Self {
            data,
            num_verts,
            num_edges,
        }
    }

    pub fn complete(num_verts: usize) -> Self {
        let mut g = Self::new(num_verts);

        for u in 0..num_verts - 1 {
            for v in u + 1..num_verts {
                g.add_edge(u, v).unwrap();
            }
        }

        g
    }
}

fn adjust_endpoints(i: &mut usize, j: &mut usize) {
    if i > j {
        swap(i, j);
    }
}

impl<'a> GraphImpl<'a> for Graph {
    type EdgeIterator = EdgeIterator<'a>;

    fn num_verts(&self) -> usize {
        self.num_verts
    }

    fn num_edges(&self) -> usize {
        self.num_edges
    }

    fn has_edge(&self, mut i: usize, mut j: usize) -> bool {
        adjust_endpoints(&mut i, &mut j);

        *self.data.const_at(i, j)
    }

    fn edges(&self) -> EdgeIterator {
        EdgeIterator::new(self)
    }

    fn add_edge(&mut self, mut i: usize, mut j: usize) -> Result {
        adjust_endpoints(&mut i, &mut j);

        if self.has_edge(i, j) {
            Err("Graph: attempting to add an existent edge")
        } else {
            *self.data.at(i, j) = true;
            self.num_edges += 1;

            Ok(())
        }
    }

    fn remove_edge(&mut self, mut i: usize, mut j: usize) -> Result {
        adjust_endpoints(&mut i, &mut j);

        if self.has_edge(i, j) {
            *self.data.at(i, j) = false;
            self.num_edges -= 1;

            Ok(())
        } else {
            Err("Graph: attempting to remove a nonexistent edge")
        }
    }
}

pub struct EdgeIterator<'a> {
    parent: &'a Graph,

    current_i: usize,
    current_j: usize,
}

impl<'a> EdgeIterator<'a> {
    pub fn new(parent: &'a Graph) -> Self {
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
    type Parent = Graph;

    fn parent(&self) -> &Graph {
        self.parent
    }

    fn current_pair(&self) -> (usize, usize) {
        (self.current_i, self.current_j)
    }

    fn next_pair(&mut self) {
        self.current_j += 1;

        if self.current_j == self.parent.num_verts() {
            self.current_i += 1;
            self.current_j = self.current_i;
        }
    }
}

impl<'a> Iterator for EdgeIterator<'a> {
    type Item = Edge;

    fn next(&mut self) -> Option<Edge> {
        self.next_edge()
    }
}
