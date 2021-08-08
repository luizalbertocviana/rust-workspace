use std::collections::{HashMap, HashSet};

use branch_bound as bb;
use branch_bound::{SolutionCost, Variable, BBProblem};
use graph::{neighbors, properties};
use graph::{GraphImpl, Subgraph};
use instance::{Instance, WGraph};

#[derive(PartialEq, Eq, Hash)]
struct Edge {
    u: usize,
    v: usize,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct EdgeWeight {
    weight: usize,
}

struct Solution<'a> {
    edges: HashSet<Edge>,
    subgraph: Subgraph<'a, WGraph>
}

impl Variable for Edge {
    type ValueType = bool;
}

impl SolutionCost for EdgeWeight {}

impl<'a> bb::Solution for Solution<'a> {
    type Var = Edge;
    type SolCost = EdgeWeight;

    fn is_feasible(&self) -> bool {
        properties::is_spanning_tree(&self.subgraph)
    }

    fn get_cost(&self) -> EdgeWeight {
        let wg = self.subgraph.parent();

        let mut cost = 0;
        for edge in &self.edges {
            cost += wg.get_edge_weight(edge.u, edge.v).unwrap();
        }

        EdgeWeight { weight: cost }
    }

    fn get_value(&self, var: &Edge) -> bool {
        self.edges.contains(var)
    }
}
