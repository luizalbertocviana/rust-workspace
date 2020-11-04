// matrix module and type
mod matrix;
pub use crate::matrix::Matrix;
// triangular_matrix module and UpperTriangularMatrix type
mod triangular_matrix;
pub use crate::triangular_matrix::UpperTriangularMatrix;
// unit tests
#[cfg(test)]
mod tests {
    use super::*;
    /// tests Matrix instantiation, access and assignment
    #[test]
    fn matrix_instantiation() {
        let mut m: Matrix<u32> = Matrix::new(10, 20);

        assert_eq!(m.num_rows(), 10);
        assert_eq!(m.num_cols(), 20);

        let default_u32: u32 = Default::default();
        assert_eq!(*m.at(0, 0), default_u32);

        *m.at(2, 3) = 4;
        assert_eq!(*m.at(2, 3), 4);
    }
    /// tests Matrix square constructor and some more assignment and
    /// access
    #[test]
    fn square_matrix() {
        let mut m: Matrix<bool> = Matrix::square(3);

        assert_eq!(m.num_rows(), 3);
        assert_eq!(m.num_cols(), 3);

        *m.at(2, 2) = true;
        *m.at(1, 1) = false;

        assert_eq!(*m.at(2, 2), true);
        assert_eq!(*m.at(1, 1), false);
    }
    /// tests UpperTriangularMatrix instantiation, access and
    /// assignment
    #[test]
    fn upper_triangular_matrix_instantiation() {
        let mut m: UpperTriangularMatrix<bool> = UpperTriangularMatrix::new(4);

        assert_eq!(m.num_rows(), 4);
        assert_eq!(m.num_cols(), 4);

        *m.at(2, 2) = true;
        assert_eq!(*m.at(2, 2), true);

        assert_eq!(*m.at(3, 3), false);
        *m.at(3, 3) = true;
        assert_eq!(*m.at(3, 3), true);
    }
}
