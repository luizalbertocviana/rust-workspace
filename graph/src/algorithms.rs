use std::collections::HashSet;

use disjoint_set::DisjointSet;

use crate::Edge;
use crate::traits::GraphImpl;
use crate::weighted_graph::WeightedGraph;

pub fn kruskal<W: Ord + Default>(graph: &WeightedGraph<W>) -> HashSet<Edge> {
    let mut components = DisjointSet::new(graph.num_verts());
    let mut mst_edges = HashSet::new();

    let mut sorted_edges: Vec<Edge> = graph.edges().collect();
    sorted_edges.sort_unstable_by_key(|(u, v)| graph.get_edge_weight(*u, *v));

    for (u, v) in sorted_edges {
        if components.representative(u) != components.representative(v) {
            mst_edges.insert((u, v));
            components.join(u, v).unwrap();
        }
    }

    mst_edges
}
