mod matrix;
pub use crate::matrix::Matrix;

mod triangular_matrix;

#[cfg(test)]
mod tests {
    use super::*;

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
}
