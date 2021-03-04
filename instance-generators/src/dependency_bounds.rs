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
