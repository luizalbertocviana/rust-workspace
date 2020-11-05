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
