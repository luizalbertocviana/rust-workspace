use matrix::Matrix;

use crate::Result;

pub struct Digraph {
    data: Matrix<bool>,

    num_verts: usize,
    num_edges: usize
}

// constructors
impl Digraph {
    pub fn new(num_verts: usize) -> Self{
        let data = Matrix::square(num_verts);
        let num_edges = 0;

        Self {data, num_verts, num_edges}
    }
}

// accessors
impl Digraph {
    pub fn num_verts(&self) -> usize {
        self.num_verts
    }

    pub fn num_edges(&self) -> usize {
        self.num_edges
    }

    pub fn has_edge(&self, i: usize, j: usize) -> bool {
        *self.data.const_at(i, j)
    }

}

// modifiers
impl Digraph {
    pub fn add_edge(&mut self, i: usize, j: usize) -> Result {
        if self.has_edge(i, j) {
            Err("Digraph: attempting to add existent edge")
        }
        else {
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
        }
        else {
            Err("Digraph: attempting to remove nonexistent edge")
        }
    }
}
