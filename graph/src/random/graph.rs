use rand::Rng;

use crate::{GraphImpl, graph::Graph, weighted_graph::WeightedGraph};

/// creates a graph with n vertices such that each possible edge has a
/// density chance of being added to it
pub fn random_graph_density(n: usize, density: f64) -> Graph {
    let mut g = Graph::new(n);

    let mut rng = rand::thread_rng();

    for u in 0..(n - 1) {
        for v in (u + 1)..n {
            let chance: f64 = rng.gen();

            if chance <= density {
                g.add_edge(u, v).unwrap();
            }
        }
    }

    g
}

/// creates a weighted graph with n vertices such that each of its
/// possible edges has a density chance of being added to it
pub fn random_weighted_graph_density<W: Default>(n: usize, density: f64) -> WeightedGraph<W> {
    let g = random_graph_density(n, density);

    let mut wg = WeightedGraph::new(n);

    for (u, v) in g.edges() {
        wg.add_edge(u, v).unwrap();
    }

    wg
}
