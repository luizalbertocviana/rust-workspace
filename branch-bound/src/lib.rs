// solving_status module
mod solving_status;
// traits module
mod traits;
// some exported traits
pub use crate::traits::{BBProblem, RelaxedProblemPool, Solution, SolutionCost};
// SolvingStatus type
use crate::solving_status::SolvingStatus;
// alias to return type of functions that can return an error
type Result<'a> = std::result::Result<(), &'a str>;
/// serial branch and bound method: given a (minimization) BBProblem
/// type T and a RelaxedProblemPool tyoe P, receives a problem and
/// returns a T::Sol solution, in case one exists
pub fn branch_and_bound<T: BBProblem, P: RelaxedProblemPool<Prob = T>>(
    problem: T,
) -> Option<T::Sol> {
    // creates a pool for problems whose relaxation has been solved
    let mut relaxed_problems = P::new();
    // solves relaxation of problem
    let root_relaxed_sol = problem.solve_relaxation();
    // adds problem and its relaxed solution to the pool
    relaxed_problems.add(problem, root_relaxed_sol);
    // creates an status for the solving process
    let mut status: SolvingStatus<T> = SolvingStatus::new();
    // while solving is not finished and there are problems in the pool
    while !status.finished() && !relaxed_problems.empty() {
        // extracts problem and its relaxed solution from the pool
        if let Some((problem, relaxed_sol)) = relaxed_problems.extract() {
            // in case relaxed_sol is feasible
            if relaxed_sol.is_feasible() {
                // if a best solution is known, compare it with relaxed_sol
                if let Some(best_sol) = status.best_solution() {
                    // if relaxed_sol is better, keep it
                    if relaxed_sol.get_cost() < best_sol.get_cost() {
                        status.set_best_solution(relaxed_sol);
                    }
                } else {
                    // in case no best solution is known yet, keep
                    // relaxed_sol as the best solution
                    status.set_best_solution(relaxed_sol);
                }
            } else {
                // in case relaxed_sol is infeasible, we branch problem
                for subproblem in problem.get_subproblems() {
                    // solves relaxation of each subproblem
                    let relaxed_sol = subproblem.solve_relaxation();
                    // if a best solution is known
                    if let Some(best_sol) = status.best_solution() {
                        // add subproblem and its relaxed_sol to the
                        // pool in case its relaxed solution cost is
                        // less than best solution cost
                        if relaxed_sol.get_cost() <= best_sol.get_cost() {
                            relaxed_problems.add(*subproblem, relaxed_sol);
                        }
                    } else {
                        // in case no best solution is known yet, add
                        // problem and its relaxed_sol to the pool
                        relaxed_problems.add(*subproblem, relaxed_sol);
                    }
                }
            }
        }
        // gets minimum value of a relaxed solution at the pool
        if let Some(min_relax_sol) = relaxed_problems.min_relaxed_solution() {
            // sets its value as new lower bound
            status.set_lower_bound(min_relax_sol.get_cost());
        }
    }
    // when solvving is done, returns best solution, if any
    status.extract_best_solution()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
