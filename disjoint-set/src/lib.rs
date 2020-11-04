// disjoint_set module
mod disjoint_set;
// DisjointSet type
pub use crate::disjoint_set::DisjointSet;
// unit tests
#[cfg(test)]
mod tests {
    use super::*;
    /// tests DisjointSet instantiation and its operations
    #[test]
    fn disjoint_set_test() {
        let n = 10;
        let mut ds = DisjointSet::new(n);

        for e in 0..n {
            assert_eq!(ds.representative(e), Some(e));
        }

        assert_eq!(ds.representative(n), None);

        ds.join(0, 1).unwrap();
        assert_eq!(ds.representative(0), ds.representative(1));

        ds.join(1, 2).unwrap();
        assert_eq!(ds.representative(0), ds.representative(2));

        ds.join(0, 4).unwrap();
        assert_eq!(ds.representative(1), ds.representative(4));

        assert_ne!(ds.representative(1), ds.representative(3));
    }
}
