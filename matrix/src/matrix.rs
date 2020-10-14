pub struct Matrix<T> {
    num_rows: usize,
    num_cols: usize,

    data: Vec<T>,
}   

// constructors
impl<T: Clone + Default> Matrix<T> {
    pub fn new(num_rows: usize, num_cols: usize) -> Self {
        let mut data: Vec<T> = Vec::new();

        data.resize(num_rows * num_cols, Default::default());

        Self {num_rows, num_cols, data}
    }

    pub fn square(dimension: usize) -> Self {
        Self::new(dimension, dimension)
    }
}

// accessors
impl<T> Matrix<T> {
    pub fn num_rows(&self) -> usize {
        self.num_rows
    }

    pub fn num_cols(&self) -> usize {
        self.num_cols
    }

    pub fn at(&mut self, i: usize, j: usize) -> &mut T {
        let position = self.index(i, j);
        
        &mut self.data[position]
    }

    fn index(&self, i: usize, j: usize) -> usize {
        self.num_cols() * i + j
    }
}
