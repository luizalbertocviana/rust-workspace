mod traits;

use crate::traits::{BBProblem, Solution};

fn branch_and_bound<T: BBProblem>(problem: &T) -> T::Sol {
    let root_relaxed_solution = problem.solve_relaxation();

    if root_relaxed_solution.is_feasible() {
        root_relaxed_solution
    }
    else {
        type SolutionCost<T> = <<T as BBProblem>::Sol as Solution>::SolutionCost;

        let lower_bound: SolutionCost<T>  = root_relaxed_solution.get_cost();
        let upper_bound: Option<SolutionCost<T>> = None;

        while upper_bound.is_none() || lower_bound < upper_bound.unwrap_or() {

        }

        root_relaxed_solution
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
