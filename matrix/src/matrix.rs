/// Matrix<T> type: represents a matrix whose positions contains
/// values of T
pub struct Matrix<T> {
    num_rows: usize,
    num_cols: usize,

    data: Vec<T>,
}

// constructors
impl<T: Clone + Default> Matrix<T> {
    /// creates a Matrix with num_rows rows and num_cols columns. Each
    /// position has initial value of default()
    pub fn new(num_rows: usize, num_cols: usize) -> Self {
        let mut data: Vec<T> = Vec::new();

        data.resize(num_rows * num_cols, Default::default());

        Self {
            num_rows,
            num_cols,
            data,
        }
    }
    /// creates a square Matrix
    pub fn square(dimension: usize) -> Self {
        Self::new(dimension, dimension)
    }
}

// accessors
impl<T> Matrix<T> {
    /// returns number of rows
    pub fn num_rows(&self) -> usize {
        self.num_rows
    }
    /// returns number of columns
    pub fn num_cols(&self) -> usize {
        self.num_cols
    }
    /// returns a mutable reference to position (i, j)
    pub fn at(&mut self, i: usize, j: usize) -> &mut T {
        let position = self.index(i, j);

        &mut self.data[position]
    }
    /// returns a reference to position (i, j)
    pub fn const_at(&self, i: usize, j: usize) -> &T {
        &self.data[self.index(i, j)]
    }
    // calculates index of position (i, j) used by the internal
    // representation of matrix
    fn index(&self, i: usize, j: usize) -> usize {
        self.num_cols() * i + j
    }
}
