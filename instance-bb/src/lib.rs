use branch_bound::{SolutionCost, Variable};
use graph::Subgraph;
use instance::WGraph;

struct Edge {
    u: usize,
    v: usize,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct EdgeWeight {
    weight: usize,
}

struct Solution<'a> {
    subgraph: Subgraph<'a, WGraph>
}

impl Variable for Edge {
    type ValueType = bool;
}

impl SolutionCost for EdgeWeight {}
