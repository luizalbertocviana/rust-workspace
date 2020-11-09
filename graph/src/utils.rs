// we use swap to adjust edge endpoints passed as arguments in order
// to ensure that they correspond to an upper triangle position
use std::mem::swap;
// swaps its arguments in case they do not represent an upper triangle
// position
pub fn adjust_endpoints(i: &mut usize, j: &mut usize) {
    if i > j {
        swap(i, j);
    }
}
