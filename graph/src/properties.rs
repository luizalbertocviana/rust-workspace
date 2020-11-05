use disjoint_set::DisjointSet;

use crate::GraphImpl;

pub fn is_acyclic<'a, T: GraphImpl<'a>>(graph: &'a T) -> bool {
    let mut components = DisjointSet::new(graph.num_verts());

    for (u, v) in graph.edges() {
        if components.representative(u) == components.representative(v) {
            return false;
        } else {
            components.join(u, v).unwrap();
        }
    }

    true
}

pub fn number_components<'a, T: GraphImpl<'a>>(graph: &'a T) -> usize {
    let mut components = DisjointSet::new(graph.num_verts());

    for (u, v) in graph.edges() {
        components.join(u, v).unwrap();
    }

    components.num_sets()
}

pub fn is_connected<'a, T: GraphImpl<'a>>(graph: &'a T) -> bool {
    number_components(graph) == 1
}
