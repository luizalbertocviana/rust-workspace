use crate::Edge;

pub trait GraphImpl<'a> {
    type EdgeIterator: Iterator<Item = Edge>;

    fn num_verts(&self) -> usize;
    fn num_edges(&self) -> usize;
    fn has_edge(&self, u: usize, v: usize) -> bool;
    fn edges(&'a self) -> Self::EdgeIterator;
}
