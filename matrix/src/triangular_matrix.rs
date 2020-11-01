use std::ops::{Deref, DerefMut};

use crate::matrix::Matrix;

pub struct UpperTriangularMatrix<T> {
    dimension: usize,
    number_rows: usize,

    data: Matrix<T>,
}

// constructors
impl<T: Clone + Default> UpperTriangularMatrix<T> {
    pub fn new(dimension: usize) -> Self {
        let number_rows = (dimension / 2) + (dimension % 2);
        let data = Matrix::new(number_rows, dimension + 1);

        Self {
            dimension,
            number_rows,
            data,
        }
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

        Reference {
            parent,
            row_index,
            col_index,
            dummy_t_member,
        }
    }

    pub fn const_at(&self, i: usize, j: usize) -> ConstReference<T> {
        let parent = self;
        let row_index = i;
        let col_index = j;
        let dummy_t_member = Default::default();

        ConstReference {
            parent,
            row_index,
            col_index,
            dummy_t_member,
        }
    }
}

// reference implementation

trait ElementReference {
    type Target;

    fn parent(&self) -> &UpperTriangularMatrix<Self::Target>;
    fn row_index(&self) -> usize;
    fn col_index(&self) -> usize;
    fn dummy_element(&self) -> &Self::Target;
}

pub struct Reference<'a, T> {
    parent: &'a mut UpperTriangularMatrix<T>,

    row_index: usize,
    col_index: usize,

    dummy_t_member: T,
}

impl<'a, T> ElementReference for Reference<'a, T> {
    type Target = T;

    fn parent(&self) -> &UpperTriangularMatrix<T> {
        self.parent
    }

    fn row_index(&self) -> usize {
        self.row_index
    }

    fn col_index(&self) -> usize {
        self.col_index
    }

    fn dummy_element(&self) -> &T {
        &self.dummy_t_member
    }
}

fn resolve_reference<T: ElementReference>(r: &T) -> &T::Target {
    let n = r.parent().dimension;
    let row = r.row_index();
    let col = r.col_index();

    let parent: &UpperTriangularMatrix<T::Target> = r.parent();

    if row > col {
        let dummy_reference = r.dummy_element();

        dummy_reference
    } else {
        if row < parent.number_rows {
            parent.data.const_at(row, 1 + col)
        } else {
            parent.data.const_at((n - 1) - row, (n - 1) - col)
        }
    }
}

impl<'a, T> Deref for Reference<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        resolve_reference(self)
    }
}

impl<'a, T> DerefMut for Reference<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let n = self.parent.dimension;

        if self.row_index > self.col_index {
            &mut self.dummy_t_member
        } else {
            if self.row_index < self.parent.number_rows {
                self.parent.data.at(self.row_index, 1 + self.col_index)
            } else {
                self.parent
                    .data
                    .at((n - 1) - self.row_index, (n - 1) - self.col_index)
            }
        }
    }
}

pub struct ConstReference<'a, T> {
    parent: &'a UpperTriangularMatrix<T>,

    row_index: usize,
    col_index: usize,

    dummy_t_member: T,
}

impl<'a, T> ElementReference for ConstReference<'a, T> {
    type Target = T;

    fn parent(&self) -> &UpperTriangularMatrix<T> {
        self.parent
    }

    fn row_index(&self) -> usize {
        self.row_index
    }

    fn col_index(&self) -> usize {
        self.col_index
    }

    fn dummy_element(&self) -> &T {
        &self.dummy_t_member
    }
}

impl<'a, T> Deref for ConstReference<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        resolve_reference(self)
    }
}
