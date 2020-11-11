use std::collections::HashMap;
use std::sync::mpsc;

use crate::solving_status::SolvingStatus;
use crate::traits::BBProblem;

enum Status {
    Feasible,
    Infeasible,
}

enum Message<T: BBProblem> {
    Finish,
    Solve(T),
}

type RelaxedProblems<T: BBProblem> = Vec<(T, T::Sol)>;

type ChannelFromWorker<T: BBProblem> = (
    mpsc::Sender<RelaxedProblems<T>>,
    mpsc::Receiver<RelaxedProblems<T>>,
);

type ChannelToWorker<T: BBProblem> = (mpsc::Sender<Message<T>>, mpsc::Receiver<Message<T>>);

pub fn branch_and_bound<T: BBProblem>(problem: T, num_workers: usize) -> Option<T::Sol> {
    let status: SolvingStatus<T> = SolvingStatus::new();

    let (worker_sender, main_receiver): ChannelFromWorker<T> = mpsc::channel();

    let mut main_sender     = Vec::new();
    let mut worker_receiver = Vec::new();
    main_sender.reserve(num_workers);
    worker_receiver.reserve(num_workers);

    for i in 0..num_workers {
        let (sender, receiver): ChannelToWorker<T> = mpsc::channel();

        main_sender.push(sender);
        worker_receiver.push(receiver);
    }

    status.extract_best_solution()
}
