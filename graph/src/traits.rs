use crate::Edge;

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
    fn current_i(&self) -> usize;
    fn current_j(&self) -> usize;

    fn next_pair(&mut self);

    fn stop(&self) -> bool {
        self.current_j() == 0 && self.current_i() == self.parent().num_verts()
    }

    fn next(&mut self) -> Option<Edge> {
        if self.stop() {
            None
        } else {
            let pair = (self.current_i(), self.current_j());

            self.next_pair();

            if self.parent().has_edge(pair.0, pair.1) {
                Some(pair)
            } else {
                self.next()
            }
        }
    }
}
