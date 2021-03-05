use graph::{GraphImpl, properties::is_connected, random};

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
    // carries distribution interval, bias chance and biased subinterval
    Biased(Interval, f64, Interval),
}

enum DigraphStructure {
    // carries branching factor
    Arborescence(usize),
    // carries density
    Dag(f64),
}

enum DependencyBoundsType {
    Constant(usize, usize),
    Interval(Interval),
    Custom(DependencyBound, DependencyBound),
}

fn generate_instance(
    g_param: GraphParameters,
    d_param: DigraphStructure,
    w_param: WeightingDistribution,
    dep_bounds_param: DependencyBoundsType,
) -> Instance {
    let mut g = random::graph::random_weighted_graph_density(g_param.num_verts, g_param.density);

    let weighting = match w_param {
        WeightingDistribution::Uniform((lw, uw)) => {
            random::weighting::create_uniformly_distributed_weighting(&g, lw, uw)
        }
        WeightingDistribution::Biased((lw, uw), bias_chance, (bias_lw, bias_uw)) => {
            random::weighting::create_biased_distributed_weighting(
                &g,
                lw,
                uw,
                bias_chance,
                bias_lw,
                bias_uw,
            )
        }
    };

    g.set_weighting(weighting);

    let d = match d_param {
        DigraphStructure::Arborescence(branching_factor) => {
            random::digraph::random_arborescence(g.num_edges(), branching_factor)
        }
        DigraphStructure::Dag(density) => {
            random::digraph::random_dag(g.num_edges(), density)
        }
    };

    let (dep_lb, dep_ub) = match dep_bounds_param {
        DependencyBoundsType::Constant(l, u) => {
            dependency_bounds::create_constant_bounds(&g, l, u)
        }
        DependencyBoundsType::Interval((lb, ub)) => {
            dependency_bounds::create_interval_bounds(&g, lb, ub)
        }
        DependencyBoundsType::Custom(lb, ub) => {
            dependency_bounds::create_from_custom_bounds(lb, ub)
        }
    };

    Instance::new(g, d, dep_lb, dep_ub)
}
