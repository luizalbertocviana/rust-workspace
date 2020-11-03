use crate::{Edge, Result};

pub trait GraphImpl<'a> {
    type EdgeIterator: Iterator<Item = Edge>;

    fn num_verts(&self) -> usize;
    fn num_edges(&self) -> usize;
    fn has_edge(&self, u: usize, v: usize) -> bool;
    fn edges(&'a self) -> Self::EdgeIterator;
}

pub trait EdgeIterable<'a> {
    type Parent: GraphImpl<'a>;

    fn parent(&self) -> &Self::Parent;
    fn current_pair(&self) -> (usize, usize);

    fn next_pair(&mut self);

    fn stop(&self) -> bool {
        let (i, j) = self.current_pair();

        j == 0 && i == self.parent().num_verts()
    }

    fn next_edge(&mut self) -> Option<Edge> {
        if self.stop() {
            None
        } else {
            let pair = self.current_pair();

            self.next_pair();

            if self.parent().has_edge(pair.0, pair.1) {
                Some(pair)
            } else {
                self.next_edge()
            }
        }
    }
}
