use graph::{GraphImpl, random};

use crate::instance::{DependencyBound, Instance};

use crate::dependency_bounds;

type Interval = (usize, usize);

struct GraphParameters {
    num_verts: usize,
    density: f64,
}

enum WeightingDistribution {
    // carries distribution interval
    Uniform(Interval),
    // carries distribution interval and biased subinterval
    Biased(Interval, Interval),
}

enum DigraphStructure {
    // carries branching factor
    Arborescence(usize),
    // carries density
    Dag(f64),
}

struct GraphParameters {
    num_verts: usize,
    density: f64,
}

// fn generate_instance(
//     g_param: GraphParameters,
//     d_param: DigraphStructure,
//     w_param: WeightingDistribution,
//     bounds: (DependencyBound, DependencyBound)
// ) -> Instance {
// }
