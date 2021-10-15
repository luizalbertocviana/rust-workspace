use std::{collections::HashSet, rc::Rc};

use branch_bound as bb;
use graph::{neighbors, properties, Graph, GraphImpl};

use crate::problem::BaseProblem;

use crate::edge::{Edge, EdgeWeight};

pub struct Solution {
    edges: HashSet<Edge>,
    subgraph: Graph,
    parent_problem: Rc<BaseProblem>,
}

#[derive(PartialEq)]
pub enum EdgeStatus {
    Feasible,
    TooFewDeps,
    TooManyDeps,
}

impl Solution {
    pub fn new(edges: HashSet<Edge>, subgraph: Graph, parent_problem: Rc<BaseProblem>) -> Self {
        Self {
            edges,
            subgraph,
            parent_problem,
        }
    }

    pub fn cost(&self) -> usize {
        self.subgraph
            .edges()
            .map(|(u, v)| {
                self.parent_problem
                    .instance()
                    .graph()
                    .get_edge_weight(u, v)
                    .unwrap()
            })
            .sum()
    }

    pub fn edges(&self) -> &HashSet<Edge> {
        &self.edges
    }

    pub fn edge_status(&self, edge: &Edge) -> EdgeStatus {
        let num_deps = self.num_deps(edge);
        let edge_tuple = (edge.u, edge.v);
        let lb = *self
            .parent_problem
            .instance()
            .dep_lb()
            .get(&edge_tuple)
            .unwrap();
        let ub = *self
            .parent_problem
            .instance()
            .dep_ub()
            .get(&edge_tuple)
            .unwrap();

        if num_deps < lb {
            EdgeStatus::TooFewDeps
        } else if num_deps > ub {
            EdgeStatus::TooManyDeps
        } else {
            EdgeStatus::Feasible
        }
    }

    fn satisfies_dependencies(&self) -> bool {
        self.edges
            .iter()
            .all(|edge| self.edge_status(edge) == EdgeStatus::Feasible)
    }

    pub fn deps(&self, edge: &Edge) -> Vec<&Edge> {
        let e_idx = self
            .parent_problem
            .edge_to_index(edge)
            .expect("instance does not contain such an edge");

        neighbors::in_neighbors(self.parent_problem.instance().dependencies(), *e_idx)
            .filter(|dep_idx| {
                self.edges
                    .contains(&self.parent_problem.index_to_edge(*dep_idx))
            })
            .map(|dep_idx| self.parent_problem.index_to_edge(dep_idx))
            .collect()
    }

    fn num_deps(&self, edge: &Edge) -> usize {
        self.deps(edge).len()
    }
}

impl bb::Solution for Solution {
    type Var = Edge;
    type SolCost = EdgeWeight;

    fn is_feasible(&self) -> bool {
        properties::is_spanning_tree(&self.subgraph) && self.satisfies_dependencies()
    }

    fn get_cost(&self) -> EdgeWeight {
        let wg = self.parent_problem.instance().graph();

        EdgeWeight {
            weight: self
                .edges
                .iter()
                .map(|edge| wg.get_edge_weight(edge.u, edge.v).unwrap())
                .sum(),
        }
    }

    fn get_value(&self, var: &Edge) -> bool {
        self.edges.contains(var)
    }
}
