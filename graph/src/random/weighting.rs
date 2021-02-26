use std::collections::HashMap;

use rand::distributions::uniform::SampleUniform;
use rand::Rng;

use crate::traits::GraphImpl;
use crate::{weighted_graph::WeightedGraph, Edge};

/// creates a mapping from the edge set of wg to weight values
/// uniformly distributed in the interval [lu, wu]
pub fn create_uniformly_distributed_weighting<W: Default + Copy + PartialOrd + SampleUniform>(
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

/// creates a mapping from the edge set of wg to weight values in the
/// interval [lu, wu] as follows: each weight has a bias_chance chance
/// of being selected from the subinterval [bias_lw, bias_uw],
/// otherwise it is selected from [lw, uw] \ [bias_lw, bias_uw]
pub fn create_biased_distributed_weighting<W: Default + SampleUniform + PartialOrd + Copy>(
    wg: &WeightedGraph<W>,
    lw: W,
    uw: W,
    bias_chance: f64,
    bias_lw: W,
    bias_uw: W,
) -> HashMap<Edge, W> {
    let mut weighting = HashMap::new();

    let mut rng = rand::thread_rng();

    for e in wg.edges() {
        let chance: f64 = rng.gen();

        let biased_weight = if chance <= bias_chance {
            rng.gen_range(bias_lw..bias_uw)
        } else {
            let low_weight = rng.gen_range(lw..bias_lw);
            let high_weight = rng.gen_range(bias_uw..uw);

            if rng.gen::<f64>() <= 0.5 {
                low_weight
            } else {
                high_weight
            }
        };

        weighting.insert(e, biased_weight);
    }

    weighting
}
