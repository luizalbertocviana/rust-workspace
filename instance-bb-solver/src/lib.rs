use std::{fmt::Display, rc::Rc, sync::Arc};

use instance::Instance;
use instance_bb::problem::{BaseProblem, Problem};
use solving_manager::traits::{Benchmark, Header, ReadBenchmark, Solve};

pub struct SolvableInstance {
    instance: Arc<Instance>,
    info: Rc<ReadingInfo>,
}

impl Solve for SolvableInstance {
    type SolvingInfo = SolvingInfo;

    fn solve(&self) -> Self::SolvingInfo {
        let problem = Problem::Base(Arc::new(BaseProblem::new(self.instance.clone())));

        let solution = branch_bound::parallel(problem, self.info.num_workers);

        let (status, solution_cost) = match solution {
            Some(sol) => (SolutionStatus::Feasible, sol.cost()),
            None => (SolutionStatus::Infeasible, 0),
        };

        SolvingInfo {
            status,
            solution_cost,
            info: self.info.clone(),
        }
    }
}

impl Benchmark for SolvableInstance {}

enum SolutionStatus {
    Feasible,
    Infeasible,
}

pub struct SolvingInfo {
    status: SolutionStatus,
    solution_cost: usize,
    info: Rc<ReadingInfo>,
}

impl Header for SolvingInfo {
    fn header() -> String {
        "description status solution_cost".to_string()
    }
}

impl Display for SolvingInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = match self.status {
            SolutionStatus::Feasible => "feasible",
            SolutionStatus::Infeasible => "infeasible",
        };

        write!(f, "{} {} {}", self.info.description, status, self.solution_cost)
    }
}

pub struct ReadingInfo {
    pub graph_file: String,
    pub deps_file: String,
    pub bounds_file: String,
    pub description: String,
    pub num_workers: usize,
}

pub struct ReadBenchmarkImpl;

impl ReadBenchmark for ReadBenchmarkImpl {
    type Benchmark = SolvableInstance;

    type ReadingInfo = ReadingInfo;

    fn read_benchmark(reading_info: Self::ReadingInfo) -> Self::Benchmark {
        let instance = Instance::from_files(
            &reading_info.graph_file,
            &reading_info.deps_file,
            &reading_info.bounds_file,
        );

        SolvableInstance {
            instance: Arc::new(instance),
            info: Rc::new(reading_info),
        }
    }
}
