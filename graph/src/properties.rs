// we use disjoint set data structure for computing some graph
// properties
use disjoint_set::DisjointSet;
// we implement these properties for any type that implements
// GraphImpl trait
use crate::GraphImpl;
/// determines whether an instance of a GraphImpl T is acyclic. Notice
/// that, in case T has directed edges, their direction is ignored
pub fn is_acyclic<'a, T: GraphImpl<'a>>(graph: &'a T) -> bool {
    // we begin with one disjoint set for each vertex
    let mut components = DisjointSet::new(graph.num_verts());
    // for each edge
    for (u, v) in graph.edges() {
        // in case its endpoints have the same representative, the
        // graph is cyclic
        if components.representative(u) == components.representative(v) {
            return false;
        } else {
            // otherwise, we join the components containing vertices u
            // and v
            components.join(u, v).unwrap();
        }
    }
    // in case each edge joins to distinct components, the graph is acyclic
    true
}
/// returns the number of connected components of an instance of a
/// GraphImpl T. Notice that, in case T has directed edges, the number
/// of weakly connected components is returned
pub fn number_components<'a, T: GraphImpl<'a>>(graph: &'a T) -> usize {
    // we begin with one disjoint set for each vertex
    let mut components = DisjointSet::new(graph.num_verts());
    // for each edge, we join the components containing its endpoints
    for (u, v) in graph.edges() {
        components.join(u, v).unwrap();
    }
    // then we return the final number of disjoint sets
    components.num_sets()
}
/// determines whether an instance of a GraphImpl T is
/// connected. Notice that, in case T has directed edges, this whether
/// the instance is weakly connected
pub fn is_connected<'a, T: GraphImpl<'a>>(graph: &'a T) -> bool {
    number_components(graph) == 1
}
