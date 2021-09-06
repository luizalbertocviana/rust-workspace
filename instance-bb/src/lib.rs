use std::collections::{HashMap, HashSet};
use std::iter::{once, FromIterator};

use branch_bound as bb;
use branch_bound::{SolutionCost, Variable};
use graph::{neighbors, properties};
use graph::{GraphImpl, Subgraph};
use instance::{Instance, WGraph};

#[derive(PartialEq, Eq, Hash, Clone)]
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
    subgraph: Subgraph<'a, WGraph>,
    parent_problem: &'a BaseProblem<'a>,
}

#[derive(PartialEq)]
enum EdgeStatus {
    Feasible,
    TooFewDeps,
    TooManyDeps,
}

impl<'a> Solution<'a> {
    fn edge_status(&self, edge: &Edge) -> EdgeStatus {
        let num_deps = self.num_deps(edge);
        let edge_tuple = (edge.u, edge.v);
        let lb = *self
            .parent_problem
            .instance
            .dep_lb()
            .get(&edge_tuple)
            .unwrap();
        let ub = *self
            .parent_problem
            .instance
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

    fn deps(&self, edge: &Edge) -> Vec<&Edge> {
        let e_idx = self
            .parent_problem
            .edge_to_index
            .get(edge)
            .expect("instance does not contain such an edge");

        neighbors::in_neighbors(self.parent_problem.instance.dependencies(), *e_idx)
            .filter(|dep_idx| {
                self.edges
                    .contains(&self.parent_problem.index_to_edge[*dep_idx])
            })
            .map(|dep_idx| &self.parent_problem.index_to_edge[dep_idx])
            .collect()
    }

    fn num_deps(&self, edge: &Edge) -> usize {
        self.deps(edge).len()
    }
}

struct BaseProblem<'a> {
    instance: &'a Instance,
    edge_to_index: HashMap<Edge, usize>,
    index_to_edge: Vec<Edge>,

    relaxed_solution: Option<Solution<'a>>,
}

impl<'a> BaseProblem<'a> {
    fn new(instance: &'a Instance) -> Self {
        let mut edge_to_index = HashMap::new();
        let mut index_to_edge = Vec::new();

        let mut i = 0;

        for (u, v) in instance.graph().edges() {
            index_to_edge.push(Edge { u, v });
            edge_to_index.insert(Edge { u, v }, i);

            i += 1;
        }

        let relaxed_solution = None;

        BaseProblem {
            instance,
            edge_to_index,
            index_to_edge,
            relaxed_solution,
        }
    }
}

struct Subproblem<'a> {
    base: &'a BaseProblem<'a>,

    added_edges: HashSet<Edge>,
    removed_edges: HashSet<Edge>,

    relaxed_solution: Option<Solution<'a>>,
}

#[derive(PartialEq)]
enum Derivation {
    AddingEdges,
    RemovingEdges,
    NoChanges,
}

enum Problem<'a> {
    Base(BaseProblem<'a>),
    Derived(Subproblem<'a>),
}

impl Variable for Edge {
    type ValueType = bool;
}

impl SolutionCost for EdgeWeight {}

impl<'a> bb::Solution for Solution<'a> {
    type Var = Edge;
    type SolCost = EdgeWeight;

    fn is_feasible(&self) -> bool {
        properties::is_spanning_tree(&self.subgraph) && self.satisfies_dependencies()
    }

    fn get_cost(&self) -> EdgeWeight {
        let wg = self.subgraph.parent();

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

struct SubproblemIterator<'a> {
    parent_problem: &'a Problem<'a>,

    parent_solution: &'a Solution<'a>,

    infeasible_edge: &'a Edge,

    subproblems: Vec<Box<Problem<'a>>>,
}

impl<'a> SubproblemIterator<'a> {
    fn new(parent_problem: &'a Problem, parent_solution: &'a Solution<'a>) -> Self {
        let infeasible_edge = parent_solution
            .edges
            .iter()
            .find(|edge| parent_solution.edge_status(edge) != EdgeStatus::Feasible)
            .expect("SubproblemIterator instantiated for a feasible problem");

        let subproblems = Vec::new();

        Self {
            parent_problem,
            parent_solution,
            infeasible_edge,
            subproblems,
        }
    }
}

// impl<'a> Iterator for SubproblemIterator<'a> {
//     type Item = Box<Problem<'a>>;

//     fn next(&mut self) -> Option<Self::Item> {
//         todo!()
//     }
// }

// impl<'a> BBProblem for Problem<'a> {
//     type Sol = Solution<'a>;

//     type SubproblemIterator;

//     fn solve_relaxation(&self) -> Self::Sol {
//         todo!()
//     }

//     fn get_subproblems(&self) -> Self::SubproblemIterator {
//         todo!()
//     }
// }
