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

    let mut kruskal_step = |(u, v): &Edge| {
        if components.representative(*u) != components.representative(*v) {
            mst_edges.insert((*u, *v));
            components.join(*u, *v).unwrap();
        }
    };

    let sorted_initial_edges: Vec<&Edge> = {
        let mut edges: Vec<&Edge> = initial_edges.iter().collect();

        edges.sort_unstable_by_key(|(u, v)| graph.get_edge_weight(*u, *v));

        edges
    };

    sorted_initial_edges.into_iter().for_each(&mut kruskal_step);

    let sorted_remaining_edges: Vec<Edge> = {
        let mut edges: Vec<Edge> = graph
            .edges()
            .filter(|edge| !initial_edges.contains(edge))
            .filter(|edge| !forbidden_edges.contains(edge))
            .collect();

        edges.sort_unstable_by_key(|(u, v)| graph.get_edge_weight(*u, *v));

        edges
    };

    sorted_remaining_edges
        .iter()
        .for_each(&mut kruskal_step);

    mst_edges
}

pub fn kruskal<W: Ord + Default>(graph: &WeightedGraph<W>) -> HashSet<Edge> {
    let empty_edge_set = HashSet::new();

    custom_kruskal(graph, &empty_edge_set, &empty_edge_set)
}
