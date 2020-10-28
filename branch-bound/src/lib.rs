mod solving_status;
mod traits;

pub use crate::traits::{BBProblem, Solution, SolutionCost};
use crate::solving_status::SolvingStatus;

type Result<'a> = std::result::Result<(), &'a str>;

pub fn branch_and_bound<T: BBProblem>(problem: &T) -> T::Sol {
    let root_relaxed_solution = problem.solve_relaxation();

    if root_relaxed_solution.is_feasible() {
        root_relaxed_solution
    }
    else {
        let mut status: SolvingStatus<T> = SolvingStatus::new();
        status.set_lower_bound(root_relaxed_solution.get_cost()).unwrap();

        let subproblems = problem.get_subproblems();

        while !status.finished() {
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
