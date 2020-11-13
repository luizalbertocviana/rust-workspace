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
    Treat(T, T::Sol),
}

enum SolvingInformation<T: BBProblem> {
    Feasible(T::Sol),
    Branched(Vec<(T, T::Sol)>),
}

type ChannelFromWorker<T: BBProblem> = (
    mpsc::Sender<(SolCost<T>, SolvingInformation<T>)>,
    mpsc::Receiver<(SolCost<T>, SolvingInformation<T>)>,
);

type ChannelToWorker<T: BBProblem> = (mpsc::Sender<Message<T>>, mpsc::Receiver<Message<T>>);

fn worker<'a, T: BBProblem>(
    sender: mpsc::Sender<(SolCost<T>, SolvingInformation<T>)>,
    receiver: mpsc::Receiver<Message<T>>,
) -> Result<'a> {
    loop {
        match receiver.recv() {
            Err(_) => return Err("worker: something went wrong when receiving a message from parallel::branch_and_bound"),
            Ok(msg) => match msg {
                Message::Finish => return Ok(()),
                Message::Treat(problem, relaxed_solution) => {
                    if relaxed_solution.is_feasible(){
                        sender.send((relaxed_solution.get_cost(), SolvingInformation::Feasible(relaxed_solution)));
                    } else {
                        let mut relaxed_problems = Vec::new();

                        for subproblem in problem.get_subproblems() {
                            let relaxed_sol = subproblem.solve_relaxation();

                            relaxed_problems.push((*subproblem, relaxed_sol));
                        }

                        sender.send((relaxed_solution.get_cost(), SolvingInformation::Branched(relaxed_problems)));
                    }
                }
            }
        }
    }
}

struct LowerBoundManager<T: BBProblem> {
    lower_bounds: HashMap<SolCost<T>, u64>,
}

impl<T: BBProblem> LowerBoundManager<T> {
    fn new() -> Self {
        let lower_bounds = HashMap::new();

        Self { lower_bounds }
    }

    fn register_lower_bound(&mut self, lb: SolCost<T>) {
        let count = *self.lower_bounds.get(&lb).unwrap_or(&0);

        self.lower_bounds.insert(lb, count + 1);
    }

    fn discard_lower_bound(&mut self, lb: SolCost<T>) {
        match self.lower_bounds.get(&lb).cloned() {
            None => (),
            Some(count) => {
                if count == 1 {
                    self.lower_bounds.remove(&lb);
                } else {
                    self.lower_bounds.insert(lb, count - 1);
                }
            }
        }
    }

    fn min_lower_bound(&self) -> Option<&SolCost<T>> {
        self.lower_bounds.keys().min()
    }
}

pub fn branch_and_bound<T: 'static + BBProblem + Send>(
    problem: T,
    num_workers: usize,
) -> Option<T::Sol> {
    let mut lb_manager: LowerBoundManager<T> = LowerBoundManager::new();
    
    let mut status: SolvingStatus<T> = SolvingStatus::new();

    let (worker_sender, main_receiver): ChannelFromWorker<T> = mpsc::channel();

    let mut main_sender = Vec::new();
    let mut worker_handler = Vec::new();
    main_sender.reserve(num_workers);
    worker_handler.reserve(num_workers);

    for _ in 0..num_workers {
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

    let root_relaxed_sol = problem.solve_relaxation();

    lb_manager.register_lower_bound(root_relaxed_sol.get_cost());
    status.set_lower_bound(root_relaxed_sol.get_cost());

    cyclic_send(Message::Treat(problem, root_relaxed_sol));

    while !status.finished() {
        match main_receiver.recv() {
            Err(_) => return None,
            Ok((parent_lower_bound, solving_information)) => {
                lb_manager.discard_lower_bound(parent_lower_bound);

                match solving_information {
                    SolvingInformation::Feasible(solution) => {
                        if let Some(best_sol) = status.best_solution() {
                            if solution.get_cost() < best_sol.get_cost() {
                                status.set_best_solution(solution);
                            }
                        } else {
                            status.set_best_solution(solution);
                        }
                    }
                    SolvingInformation::Branched(relaxed_subproblems) => {
                        for (problem, relaxed_sol) in relaxed_subproblems {
                            lb_manager.register_lower_bound(relaxed_sol.get_cost());

                            cyclic_send(Message::Treat(problem, relaxed_sol));
                        }
                    }
                }
            }
        }

        if let Some(min_lb) = lb_manager.min_lower_bound() {
            status.set_lower_bound(*min_lb);
        }
    }

    status.extract_best_solution()
}
