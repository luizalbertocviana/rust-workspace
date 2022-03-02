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
        let non_fixed_infeasible_edge = match parent_problem {
            Problem::Base(_base_problem) => parent_solution
                .edges()
                .iter()
                .find(|edge| parent_solution.edge_status(edge) != EdgeStatus::Feasible)
                .expect("SubproblemIterator instantiated for a base problem whose solution edges are all feasible"),
            Problem::Derived(subproblem) => parent_solution
                .edges()
                .iter()
                .filter(|edge| !subproblem.fixed_edge(edge))
                .find(|edge| parent_solution.edge_status(edge) != EdgeStatus::Feasible)
                .expect("SubproblemIterator instantiated for a subproblem whose solution free edges are all feasible")
        };

        let mut subproblems = Vec::new();

        let mut add_subproblem =
            |subproblem| subproblems.push(Box::new(Problem::Derived(subproblem)));

        let subproblem_without_infeasible_edge = Subproblem::from_problem(
            parent_problem,
            iter::empty(),
            iter::once(non_fixed_infeasible_edge.clone()),
        );

        add_subproblem(subproblem_without_infeasible_edge);

        let non_fixed_infeasible_edge_status =
            parent_solution.edge_status(non_fixed_infeasible_edge);

        let non_fixed_deps_of_infeasible_edge = {
            let deps = parent_solution.deps(non_fixed_infeasible_edge);

            match parent_problem {
                Problem::Base(_base_problem) => deps,
                Problem::Derived(subproblem) => deps
                    .into_iter()
                    .filter(|edge| !subproblem.fixed_edge(edge))
                    .collect(),
            }
        };

        for non_fixed_infeasible_edge_dep in non_fixed_deps_of_infeasible_edge {
            let derived_subproblem = match non_fixed_infeasible_edge_status {
                EdgeStatus::Feasible => {
                    Subproblem::from_problem(parent_problem, iter::empty(), iter::empty())
                }
                EdgeStatus::TooFewDeps => Subproblem::from_problem(
                    parent_problem,
                    vec![
                        non_fixed_infeasible_edge.clone(),
                        non_fixed_infeasible_edge_dep.clone(),
                    ]
                    .into_iter(),
                    iter::empty(),
                ),
                EdgeStatus::TooManyDeps => Subproblem::from_problem(
                    parent_problem,
                    iter::once(non_fixed_infeasible_edge.clone()),
                    iter::once(non_fixed_infeasible_edge_dep.clone()),
                ),
            };

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
