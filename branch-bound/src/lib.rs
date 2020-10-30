mod solving_status;
mod traits;

pub use crate::traits::{BBProblem, RelaxedProblemPool, Solution, SolutionCost};

use crate::solving_status::SolvingStatus;

type Result<'a> = std::result::Result<(), &'a str>;

pub fn branch_and_bound<T: BBProblem, P: RelaxedProblemPool<Prob = T>>(
    problem: T,
) -> Option<T::Sol> {
    let mut relaxed_problems = P::new();
    let root_relaxed_sol = problem.solve_relaxation();

    relaxed_problems.add(problem, root_relaxed_sol);

    let mut status: SolvingStatus<T> = SolvingStatus::new();

    while !status.finished() && !relaxed_problems.empty() {
        if let Some((problem, relaxed_sol)) = relaxed_problems.extract() {
            if relaxed_sol.is_feasible() {
                if let Some(best_sol) = status.best_solution() {
                    if relaxed_sol.get_cost() < best_sol.get_cost() {
                        status.set_best_solution(relaxed_sol);
                    }
                } else {
                    status.set_best_solution(relaxed_sol);
                }
            } else {
                for subproblem in problem.get_subproblems() {
                    let relaxed_sol = subproblem.solve_relaxation();

                    if let Some(best_sol) = status.best_solution() {
                        if relaxed_sol.get_cost() <= best_sol.get_cost() {
                            relaxed_problems.add(*subproblem, relaxed_sol);
                        }
                    } else {
                        relaxed_problems.add(*subproblem, relaxed_sol);
                    }
                }
            }
        }

        if let Some(min_relax_sol) = relaxed_problems.min_relaxed_solution() {
            status.set_lower_bound(min_relax_sol.get_cost());
        }
    }

    status.extract_best_solution()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
