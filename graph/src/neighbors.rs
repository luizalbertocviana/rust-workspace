use crate::GraphImpl;

/// returns number of in-neighbors of vertex v in graph. This is
/// equivalent to num_out_neighbors when graph is undirected
pub fn num_in_neighbors<'a, T: GraphImpl<'a>>(graph: &T, v: usize) -> usize {
    let mut num = 0;

    for u in 0..graph.num_verts() {
        if graph.has_edge(u, v) {
            num += 1;
        }
    }

    num
}
/// returns number of out-neighbors of vertex u in graph. This is
/// equivalent to num_in_neighbors when graph is undirected
pub fn num_out_neighbors<'a, T: GraphImpl<'a>>(graph: &T, u: usize) -> usize {
    let mut num = 0;

    for v in 0..graph.num_verts() {
        if graph.has_edge(u, v) {
            num += 1;
        }
    }

    num
}

pub fn in_neighbors<'a, T: GraphImpl<'a>>(graph: &'a T, v: usize) -> InNeighborIterator<T> {
    InNeighborIterator {
        data: NeighborIteratorData {
            vertex: v,
            current_vertex: 0,
            parent: graph,
        },
    }
}

pub fn out_neighbors<'a, T: GraphImpl<'a>>(graph: &'a T, u: usize) -> OutNeighborIterator<T> {
    OutNeighborIterator {
        data: NeighborIteratorData {
            vertex: u,
            current_vertex: 0,
            parent: graph,
        },
    }
}

struct NeighborIteratorData<'a, T: GraphImpl<'a>> {
    vertex: usize,
    current_vertex: usize,
    parent: &'a T,
}

pub struct InNeighborIterator<'a, T: GraphImpl<'a>> {
    data: NeighborIteratorData<'a, T>,
}

impl<'a, T: GraphImpl<'a>> Iterator for InNeighborIterator<'a, T> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.current_vertex < self.data.parent.num_verts() {
            let candidate = self.data.current_vertex;

            self.data.current_vertex += 1;

            if self.data.parent.has_edge(candidate, self.data.vertex) {
                Some(candidate)
            } else {
                self.next()
            }
        } else {
            None
        }
    }
}

pub struct OutNeighborIterator<'a, T: GraphImpl<'a>> {
    data: NeighborIteratorData<'a, T>,
}

impl<'a, T: GraphImpl<'a>> Iterator for OutNeighborIterator<'a, T> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.current_vertex < self.data.parent.num_verts() {
            let candidate = self.data.current_vertex;

            self.data.current_vertex += 1;

            if self.data.parent.has_edge(self.data.vertex, candidate) {
                Some(candidate)
            } else {
                self.next()
            }
        } else {
            None
        }
    }
}
