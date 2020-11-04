// references are abstracted by some types which implement these
// traits
use std::ops::{Deref, DerefMut};
// we use Matrix as the internal representation of
// UpperTriangularMatrix
use crate::matrix::Matrix;
/// UpperTriangularMatrix<T> represents an upper triangular matrix
/// whose positions of its upper triangle contain values of T
pub struct UpperTriangularMatrix<T> {
    dimension: usize,
    // number_rows is about the internal representation
    number_rows: usize,

    data: Matrix<T>,
}

// constructors
impl<T: Clone + Default> UpperTriangularMatrix<T> {
    /// creates a square UpperTriangularMatrix with specified
    /// dimension
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
    /// returns number of rows
    pub fn num_rows(&self) -> usize {
        self.dimension
    }
    /// returns number of columns
    pub fn num_cols(&self) -> usize {
        self.dimension
    }
    /// returns a mutable reference to position (i, j). If (i, j) is
    /// not an upper triangle position, the returned reference is
    /// dummy
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
    /// returns a reference to position (i, j). If (i, j) is not an
    /// upper triangle position,  the returned reference is dummy
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

// this trait is internally used for code reuse
trait ElementReference {
    // type of matrix elements
    type Target;
    // returns reference to matrix which contains the referenced
    // position
    fn parent(&self) -> &UpperTriangularMatrix<Self::Target>;
    // returns row index of reference
    fn row_index(&self) -> usize;
    // returns column index of reference
    fn col_index(&self) -> usize;
    // returns reference to dummy element. This is used to represent
    // lower triangle references
    fn dummy_element(&self) -> &Self::Target;
}
/// a mutable reference to a position of UpperTriangularMatrix
pub struct Reference<'a, T> {
    // reference to matrix it refers to
    parent: &'a mut UpperTriangularMatrix<T>,
    // position indices of reference
    row_index: usize,
    col_index: usize,
    // in case this is a lower triangle reference, it must refer to
    // this element
    dummy_t_member: T,
}
// ElementReference implementation for Reference
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
// returns reference to element represented by r
fn resolve_reference<T: ElementReference>(r: &T) -> &T::Target {
    // r reference information
    let n = r.parent().dimension;
    let row = r.row_index();
    let col = r.col_index();
    // UpperTriangularMatrix r refers to
    let parent: &UpperTriangularMatrix<T::Target> = r.parent();
    // if r is a lower triangle reference
    if row > col {
        let dummy_reference = r.dummy_element();
        // returns reference to dummy_element
        dummy_reference
    } else {
        // this resolves r in terms of our internal representation layout
        if row < parent.number_rows {
            parent.data.const_at(row, 1 + col)
        } else {
            parent.data.const_at((n - 1) - row, (n - 1) - col)
        }
    }
}
// Deref and DerefMut implementations for Reference. Now Reference can
// be dereferenced ...
impl<'a, T> Deref for Reference<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        resolve_reference(self)
    }
}
// ... and assigned to
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
/// a reference to a position of UpperTriangularMatrix
pub struct ConstReference<'a, T> {
    parent: &'a UpperTriangularMatrix<T>,

    row_index: usize,
    col_index: usize,

    dummy_t_member: T,
}
// ElementReference implementation for ConstReference
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
// Deref implementation for ConstReference so it can be dereferenced
impl<'a, T> Deref for ConstReference<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        resolve_reference(self)
    }
}
