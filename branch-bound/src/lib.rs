mod traits;

pub use crate::traits::{ProblemPool, BBProblem, Solution, SolutionCost};

// type Result<'a> = std::result::Result<(), &'a str>;

pub fn branch_and_bound<T: BBProblem, P: ProblemPool<Prob = T>>(problem: T) -> Option<T::Sol> {
    let mut unsolved_problems = P::new();
    unsolved_problems.add(problem);

    let mut best_solution: Option<T::Sol> = None;

    while let Some(problem) = unsolved_problems.extract() {
        let solution = problem.solve_relaxation();

        if solution.is_feasible() {
            if let Some(sol) = &best_solution {
                if solution.get_cost() < sol.get_cost() {
                    best_solution = Some(solution);
                }
            }
            else {
                best_solution = Some(solution);
            }
        }
        else {
            for subp in problem.get_subproblems() {
                unsolved_problems.add(*subp);
            }
        }
    }

    best_solution
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
