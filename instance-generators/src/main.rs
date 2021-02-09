use std::collections::HashMap;

use rand::distributions::uniform::SampleUniform;
use rand::prelude::*;

use graph::*;

fn random_graph_density(n: usize, density: f64) -> Graph {
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

fn random_weighted_graph_density<W: Default>(n: usize, density: f64) -> WeightedGraph<W> {
    let g = random_graph_density(n, density);

    let mut wg = WeightedGraph::new(n);

    for (u, v) in g.edges() {
        wg.add_edge(u, v).unwrap();
    }

    wg
}

fn create_uniformly_distributed_weighting<W: Default + Copy + PartialOrd + SampleUniform>(
    wg: &WeightedGraph<W>,
    lw: W,
    uw: W,
) -> HashMap<Edge, W> {
    let mut weighting = HashMap::new();

    let mut rng = rand::thread_rng();

    for e in wg.edges() {
        let random_weight = rng.gen_range(lw..uw);

        weighting.insert(e, random_weight);
    }

    weighting
}

fn main() {
    println!("Hello, world!");
}
