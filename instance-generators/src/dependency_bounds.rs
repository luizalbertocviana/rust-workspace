use std::{cmp::max, collections::HashMap};

use rand::Rng;

use graph::{properties::num_in_neighbors, Digraph, GraphImpl};

use crate::{
    generator::{LowerBoundDependencyType, UpperBoundDependencyType},
    instance::{DependencyBound, WGraph},
};

pub fn create_constant_bounds(
    g: &WGraph,
    l: usize,
    u: usize,
) -> (DependencyBound, DependencyBound) {
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
    g: &WGraph,
    d: &Digraph,
    lb_param: &LowerBoundDependencyType,
    ub_param: &UpperBoundDependencyType,
) -> (DependencyBound, DependencyBound) {
    let mut l_bound = HashMap::new();
    let mut u_bound = HashMap::new();

    let mut rng = rand::thread_rng();

    for (e_idx, e) in g.edges().enumerate() {
        let num_deps = num_in_neighbors(d, e_idx);

        let l = match lb_param {
            LowerBoundDependencyType::Interval((min_l, max_l)) => rng.gen_range(*min_l..*max_l),
            LowerBoundDependencyType::NearlyStrong => {
                if 0 >= num_deps {
                    0
                } else {
                    rng.gen_range(0..num_deps)
                }
            }
            LowerBoundDependencyType::Strong => num_deps,
        };

        let u = match ub_param {
            UpperBoundDependencyType::Interval((min_u, max_u)) => {
                rng.gen_range(max(l, *min_u)..*max_u)
            }
            UpperBoundDependencyType::NearlyWeak => {
                if l >= num_deps {
                    l
                } else {
                    rng.gen_range(l..num_deps)
                }
            }
            UpperBoundDependencyType::Weak => num_deps,
        };

        l_bound.insert(e, l);
        u_bound.insert(e, u);
    }

    (l_bound, u_bound)
}
