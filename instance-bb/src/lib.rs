use std::collections::{HashMap, HashSet};
use std::iter::once;

use bb::BBProblem;
use branch_bound as bb;
use branch_bound::{SolutionCost, Variable};
use graph::{Graph, algorithms, neighbors, properties};
use graph::{GraphImpl};
use instance::{Instance};

#[derive(PartialEq, Eq, Hash, Clone)]
struct Edge {
    u: usize,
    v: usize,
}

type EdgeTuple = (usize, usize);

impl Variable for Edge {
    type ValueType = bool;
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct EdgeWeight {
    weight: usize,
}

impl SolutionCost for EdgeWeight {}

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

struct BaseProblem<'a> {
    instance: &'a Instance,
    edge_to_index: HashMap<Edge, usize>,
    index_to_edge: Vec<Edge>,
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

        BaseProblem {
            instance,
            edge_to_index,
            index_to_edge,
        }
    }
}

struct Subproblem<'a> {
    base: &'a BaseProblem<'a>,

    added_edges: HashSet<Edge>,
    removed_edges: HashSet<Edge>,
}

#[derive(PartialEq)]
enum Derivation {
    AddingEdges,
    RemovingEdges,
    NoChanges,
}

impl<'a> Subproblem<'a> {
    fn new(
        base: &'a BaseProblem,
        added_edges: HashSet<Edge>,
        removed_edges: HashSet<Edge>,
    ) -> Self {
        Self {
            base,
            added_edges,
            removed_edges,
        }
    }

    fn from_base_problem(base: &'a BaseProblem) -> Self {
        Self::new(base, HashSet::new(), HashSet::new())
    }

    fn from_subproblem(subproblem: &Subproblem<'a>) -> Self {
        let base = subproblem.base;
        let added_edges = subproblem.added_edges.clone();
        let removed_edges = subproblem.removed_edges.clone();

        Self::new(base, added_edges, removed_edges)
    }

    fn from_problem<I: Iterator<Item = Edge>>(
        problem: &'a Problem,
        derivation: &Derivation,
        edges: I,
    ) -> Self {
        let mut base = match problem {
            Problem::Base(base_problem) => Self::from_base_problem(base_problem),
            Problem::Derived(subproblem) => Self::from_subproblem(subproblem),
        };

        match derivation {
            Derivation::AddingEdges => {
                for edge in edges {
                    base.added_edges.insert(edge);
                }
            }
            Derivation::RemovingEdges => {
                for edge in edges {
                    base.removed_edges.insert(edge);
                }
            }
            Derivation::NoChanges => (),
        }

        base
    }
}

enum Problem<'a> {
    Base(BaseProblem<'a>),
    Derived(Subproblem<'a>),
}

struct SubproblemIterator<'a> {
    subproblems: Vec<Box<Problem<'a>>>,
}

impl<'a> SubproblemIterator<'a> {
    fn new(parent_problem: &'a Problem, parent_solution: &'a Solution<'a>) -> Self {
        let infeasible_edge = parent_solution
            .edges
            .iter()
            .find(|edge| parent_solution.edge_status(edge) != EdgeStatus::Feasible)
            .expect("SubproblemIterator instantiated for a problem whose edges are all feasible");

        let mut subproblems = Vec::new();

        let mut add_subproblem =
            |subproblem| subproblems.push(Box::new(Problem::Derived(subproblem)));

        let subproblem_without_infeasible_edge = Subproblem::from_problem(
            parent_problem,
            &Derivation::RemovingEdges,
            once(infeasible_edge.clone()),
        );

        add_subproblem(subproblem_without_infeasible_edge);

        let derivation_strategy = match parent_solution.edge_status(infeasible_edge) {
            EdgeStatus::TooManyDeps => Derivation::RemovingEdges,
            EdgeStatus::TooFewDeps => Derivation::AddingEdges,
            EdgeStatus::Feasible => Derivation::NoChanges,
        };

        for infeasible_edge_dep in parent_solution.deps(infeasible_edge) {
            let derived_subproblem = Subproblem::from_problem(
                parent_problem,
                &derivation_strategy,
                once(infeasible_edge_dep.clone()),
            );

            add_subproblem(derived_subproblem);
        }

        Self { subproblems }
    }
}

impl<'a> Iterator for SubproblemIterator<'a> {
    type Item = Box<Problem<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.subproblems.pop()
    }
}

// impl<'a> BBProblem for Problem<'a> {
//     type Sol = Solution<'a>;

//     type SubproblemIterator = SubproblemIterator<'a>;

//     fn solve_relaxation(&self) -> Self::Sol {
//         let edges = match self {
//             Problem::Base(base_problem) => algorithms::kruskal(base_problem.instance.graph()),
//             Problem::Derived(subproblem) => {
//                 let edge_tuple_set = |edge_struct_set: HashSet<Edge>| {
//                     edge_struct_set
//                         .iter()
//                         .map(|edge| (edge.u, edge.v))
//                         .collect()
//                 };

//                 let initial_edges: HashSet<EdgeTuple> = edge_tuple_set(subproblem.added_edges);
//                 let forbidden_edges: HashSet<EdgeTuple> = edge_tuple_set(subproblem.removed_edges);

//                 algorithms::custom_kruskal(
//                     subproblem.base.instance.graph(),
//                     &initial_edges,
//                     &forbidden_edges,
//                 )
//             }
//         };

//         let subgraph = {
//             let mut subgraph = 
//         };
//     }

//     fn get_subproblems(&self) -> Self::SubproblemIterator {
//         todo!()
//     }
// }
