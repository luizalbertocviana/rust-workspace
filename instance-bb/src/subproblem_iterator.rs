use std::iter;

use crate::{
    problem::{Problem, Subproblem},
    solution::{EdgeStatus, Solution},
};

pub struct SubproblemIterator {
    subproblems: Vec<Box<Problem>>,
}

impl SubproblemIterator {
    pub fn new(parent_problem: &Problem, parent_solution: &Solution) -> Self {
        let infeasible_edge = match parent_problem {
            Problem::Base(_base_problem) => parent_solution
                .edges()
                .iter()
                .find(|edge| parent_solution.edge_status(edge) != EdgeStatus::Feasible)
                .expect("SubproblemIterator instantiated for a problem whose edges are all feasible"),
            Problem::Derived(subproblem) => parent_solution
                .edges()
                .iter()
                .filter(|edge| !subproblem.fixed_edge(edge))
                .find(|edge| parent_solution.edge_status(edge) != EdgeStatus::Feasible)
                .expect("SubproblemIterator instantiated for a subproblem whose free edges are all feasible")
        };

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

impl Iterator for SubproblemIterator {
    type Item = Box<Problem>;

    fn next(&mut self) -> Option<Self::Item> {
        self.subproblems.pop()
    }
}
