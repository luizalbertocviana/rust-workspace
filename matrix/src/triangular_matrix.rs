use std::ops::DerefMut;
use std::ops::Deref;

use crate::matrix::Matrix;

pub struct UpperTriangularMatrix<T> {
    dimension: usize,
    number_rows: usize,
    
    data: Matrix<T>
}

pub struct Reference<'a, T> {
    parent: &'a mut UpperTriangularMatrix<T>,

    row_index: usize,
    col_index: usize,

    dummy_t_member: T
}

impl<'a, T> Deref for Reference<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let n = self.parent.dimension;

        if self.row_index > self.col_index {
            &self.dummy_t_member
        }
        else {
            if self.row_index < self.parent.number_rows {
                self.parent.data.const_at(self.row_index, 1 + self.col_index)
            }
            else {
                self.parent.data.const_at((n - 1) - self.row_index, (n - 1) - self.col_index)
            }
        }
    }
}

impl<'a, T> DerefMut for Reference<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let n = self.parent.dimension;

        if self.row_index > self.col_index {
            &mut self.dummy_t_member
        }
        else {
            if self.row_index < self.parent.number_rows {
                self.parent.data.at(self.row_index, 1 + self.col_index)
            }
            else {
                self.parent.data.at((n - 1) - self.row_index, (n - 1) - self.col_index)
            }
        }
    }
}

// constructors
impl<T: Clone + Default> UpperTriangularMatrix<T> {
    pub fn new(dimension: usize) -> Self {
        let number_rows = (dimension / 2) + (dimension % 2);
        let data = Matrix::new(number_rows, dimension + 1);

        Self {dimension, number_rows, data}
    }
}

// accessors
impl<T: Default> UpperTriangularMatrix<T> {
    pub fn num_rows(&self) -> usize {
        self.dimension
    }

    pub fn num_cols(&self) -> usize {
        self.dimension
    }

    pub fn at(&mut self, i: usize, j: usize) -> Reference<T> {
        let parent = self;
        let row_index = i;
        let col_index = j;
        let dummy_t_member = Default::default();

        Reference {parent, row_index, col_index, dummy_t_member}
    }
}
