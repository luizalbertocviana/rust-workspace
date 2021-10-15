use branch_bound::{SolutionCost, Variable};

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Edge {
    pub u: usize,
    pub v: usize,
}

pub type EdgeTuple = (usize, usize);

impl Variable for Edge {
    type ValueType = bool;
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct EdgeWeight {
    pub weight: usize,
}

impl SolutionCost for EdgeWeight {}
