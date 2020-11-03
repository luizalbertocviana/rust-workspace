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
