use std::collections::HashSet;

use disjoint_set::DisjointSet;

use crate::traits::GraphImpl;
use crate::weighted_graph::WeightedGraph;
use crate::Edge;

pub fn custom_kruskal<W: Ord + Default>(
    graph: &WeightedGraph<W>,
    initial_edges: &HashSet<Edge>,
    forbidden_edges: &HashSet<Edge>,
) -> HashSet<Edge> {
    let mut components = DisjointSet::new(graph.num_verts());
    let mut mst_edges = HashSet::new();

    let mut sorted_initial_edges: Vec<&Edge> = initial_edges.iter().collect();
    sorted_initial_edges.sort_unstable_by_key(|(u, v)| graph.get_edge_weight(*u, *v));

    let mut kruskal_step = |(u, v): &Edge| {
        if components.representative(*u) != components.representative(*v) {
            mst_edges.insert((*u, *v));
            components.join(*u, *v).unwrap();
        }
    };

    sorted_initial_edges.into_iter().for_each(&mut kruskal_step);

    let remaining_edges: HashSet<Edge> = graph.edges().collect();
    let mut sorted_remaining_non_forbidden_edges: Vec<&Edge> =
        remaining_edges.difference(forbidden_edges).collect();

    sorted_remaining_non_forbidden_edges
        .sort_unstable_by_key(|(u, v)| graph.get_edge_weight(*u, *v));

    sorted_remaining_non_forbidden_edges
        .into_iter()
        .for_each(&mut kruskal_step);

    mst_edges
}

pub fn kruskal<W: Ord + Default>(graph: &WeightedGraph<W>) -> HashSet<Edge> {
    let empty_edge_set = HashSet::new();

    custom_kruskal(graph, &empty_edge_set, &empty_edge_set)
}
