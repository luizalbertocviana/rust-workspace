use crate::traits::{BBProblem, Solution};

type Sol<T> = <T as BBProblem>::Sol;
type SolCost<T> = <Sol<T> as Solution>::SolCost;

type QueuedProblem<T: BBProblem> = (T, Option<Sol<T>>);

pub struct ProblemPool<T: BBProblem> {
    problems: Vec<QueuedProblem<T>>
}

// constructors
impl<T: BBProblem> ProblemPool<T> {
    pub fn new() -> Self {
        let problems = Vec::new();

        Self {problems}
    }
}

// accessors
impl<T: BBProblem> ProblemPool<T> {
    pub fn min_solution_cost(&self) -> Option<SolCost<T>> {
        self.problems.iter()
            .filter(|(_prb, sol)| sol.is_some())
            .map(|(_prb, sol)| sol.as_ref().unwrap().get_cost())
            .min()
    }
}

// modifiers
impl<T: BBProblem> ProblemPool<T> {
    pub fn add_unsolved_problem(&mut self, problem: T) {
        self.problems.push((problem, None));
    }

    pub fn add_solved_problem(&mut self, problem: T, solution: Sol<T>) {
        self.problems.push((problem, Some(solution)));
    }

    pub fn extract_problem(&mut self) -> Option<QueuedProblem<T>> {
        self.problems.pop()
    }
}
