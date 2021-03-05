use std::collections::HashMap;

use rand::Rng;

use graph::GraphImpl;

use crate::instance::{DependencyBound, WGraph};

pub fn create_constant_bounds(g: &WGraph, l: usize, u: usize) -> (DependencyBound, DependencyBound) {
    let mut l_bound = HashMap::new();
    let mut u_bound = HashMap::new();

    for e in g.edges() {
        l_bound.insert(e, l);
        u_bound.insert(e, u);
    }

    (l_bound, u_bound)
}

pub fn create_interval_bounds(
    g: &WGraph,
    lb: usize,
    ub: usize,
) -> (DependencyBound, DependencyBound) {
    let mut l_bound = HashMap::new();
    let mut u_bound = HashMap::new();

    let mut rng = rand::thread_rng();

    for e in g.edges() {
        let l = rng.gen_range(lb..ub);
        let u = rng.gen_range(l..ub);

        l_bound.insert(e, l);
        u_bound.insert(e, u);
    }

    (l_bound, u_bound)
}

pub fn create_from_custom_bounds(
    lb: &DependencyBound,
    ub: &DependencyBound,
) -> (DependencyBound, DependencyBound) {
    let mut l_bound = HashMap::new();
    let mut u_bound = HashMap::new();

    let mut rng = rand::thread_rng();

    for e in lb.keys() {
        let lower = *lb.get(e).unwrap();
        let upper = *ub.get(e).unwrap();

        let l = rng.gen_range(lower..upper);
        let u = rng.gen_range(l..upper);

        l_bound.insert(*e, l);
        u_bound.insert(*e, u);
    }

    (l_bound, u_bound)
}
