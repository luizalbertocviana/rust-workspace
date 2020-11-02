use crate::Edge;

pub trait GraphImpl<'a> {
    type EdgeIterator: Iterator<Item = Edge>;

    fn num_verts(&self) -> usize;
    fn edges(&'a self) -> Self::EdgeIterator;
}
