use std::collections::HashMap;
use std::marker::Send;
use std::sync::mpsc;
use std::thread;

use crate::solving_status::SolvingStatus;
use crate::traits::{BBProblem, Solution};
use crate::Result;

type Sol<T> = <T as BBProblem>::Sol;
type SolCost<T> = <Sol<T> as Solution>::SolCost;

enum Message<T: BBProblem> {
    Finish,
    Solve(T),
}

enum SolvingInformation<T: BBProblem> {
    Feasible(SolCost<T>, T::Sol),
    Branched(SolCost<T>, Vec<(T, T::Sol)>),
}

type ChannelFromWorker<T: BBProblem> = (
    mpsc::Sender<SolvingInformation<T>>,
    mpsc::Receiver<SolvingInformation<T>>,
);

type ChannelToWorker<T: BBProblem> = (mpsc::Sender<Message<T>>, mpsc::Receiver<Message<T>>);

fn worker<'a, T: BBProblem>(
    sender: mpsc::Sender<SolvingInformation<T>>,
    receiver: mpsc::Receiver<Message<T>>,
) -> Result<'a> {
    loop {
        match receiver.recv() {
            Err(_) => return Err("worker: something went wrong when receiving a message from parallel::branch_and_bound"),
            Ok(msg) => match msg {
                Message::Finish => return Ok(()),
                Message::Solve(problem) => {
                    let relaxed_solution = problem.solve_relaxation();

                    if relaxed_solution.is_feasible(){
                        sender.send(SolvingInformation::Feasible(relaxed_solution));
                    } else {
                        let mut relaxed_problems = Vec::new();

                        for subproblem in problem.get_subproblems() {
                            let relaxed_sol = subproblem.solve_relaxation();

                            relaxed_problems.push((*subproblem, relaxed_sol));
                        }

                        sender.send(SolvingInformation::Branched(relaxed_solution.get_cost(), relaxed_problems));
                    }
                }
            }
        }
    }
}

pub fn branch_and_bound<T: 'static + BBProblem + Send>(
    problem: T,
    num_workers: usize,
) -> Option<T::Sol> {
    let lower_bounds: HashMap<SolCost<T>, u64> = HashMap::new();
    let register_lower_bound = |lb| {
        if lower_bounds.contains_key(lb) {
            lower_bounds[lb] += 1;
        } else {
            lower_bounds[lb] = 1;
        }
    };

    let status: SolvingStatus<T> = SolvingStatus::new();

    let (worker_sender, main_receiver): ChannelFromWorker<T> = mpsc::channel();

    let mut main_sender = Vec::new();
    let mut worker_handler = Vec::new();
    main_sender.reserve(num_workers);
    worker_handler.reserve(num_workers);

    for i in 0..num_workers {
        let (sender, receiver): ChannelToWorker<T> = mpsc::channel();
        let clonned_worker_sender = worker_sender.clone();

        main_sender.push(sender);

        let handler = thread::spawn(move || {
            worker(clonned_worker_sender, receiver);
        });

        worker_handler.push(handler);
    }

    let mut main_sender_cycle = main_sender.iter().cycle();
    let mut cyclic_send = |msg| main_sender_cycle.next().unwrap().send(msg);

    cyclic_send(Message::Solve(problem));

    while !status.finished() {
        match main_receiver.recv() {
            Err(_) => Err("parallel::branch_and_bound: something went wrong when receiving data from the worker threads"),
            Ok(solving_information) => match solving_information {
                Feasible(parent_lower_bound, solution) => if let Some(best_sol) = status.best_solution() {
                    if solution.get_cost < best_sol.get_cost {
                        status.set_best_solution(solution);
                    }
                } else {
                    status.set_best_solution(solution);
                }
                // Branched(parent_lower_bound, relaxed_subproblems) => 
            }
        }

        status.extract_best_solution()
    }
}
