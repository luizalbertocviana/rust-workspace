use crate::matrix::Matrix;

struct UpperTriangularMatrix<T> {
    dimension: usize,
    
    data: Matrix<T>
}

impl<T: Clone + Default> UpperTriangularMatrix<T> {
    fn new(dimension: usize) -> Self {
        let number_rows = (dimension / 2) + (dimension % 2);
        let data = Matrix::new(number_rows, dimension + 1);

        Self {dimension, data}
    }
}
