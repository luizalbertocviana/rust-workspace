use std::{fmt::Display, sync::Arc};

use instance::Instance;
use instance_bb::problem::{BaseProblem, Problem};
use solving_manager::traits::{Benchmark, Header, ReadBenchmark, Solve};

struct SolvableInstance {
    instance: Arc<Instance>,
    num_workers: usize,
}

impl Solve for SolvableInstance {
    type SolvingInfo = SolvingInfo;

    fn solve(&self) -> Self::SolvingInfo {
        let problem = Problem::Base(Arc::new(BaseProblem::new(self.instance.clone())));

        let solution = branch_bound::parallel(problem, self.num_workers);

        let (status, solution_cost) = match solution {
            Some(sol) => (SolutionStatus::Feasible, sol.cost()),
            None => (SolutionStatus::Infeasible, 0),
        };

        SolvingInfo {
            status,
            solution_cost,
        }
    }
}

impl Benchmark for SolvableInstance {}

enum SolutionStatus {
    Feasible,
    Infeasible,
}

struct SolvingInfo {
    status: SolutionStatus,
    solution_cost: usize,
}

impl Header for SolvingInfo {
    fn header() -> String {
        "solution_cost".to_string()
    }
}

impl Display for SolvingInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.solution_cost)
    }
}
