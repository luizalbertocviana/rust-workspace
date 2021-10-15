// we use this trait to ensure some types can be used as HashMap keys
use std::hash::Hash;
// we use a HashMap to store lower bounds inside LowerBoundManager
use std::collections::HashMap;
// this trait is implemented (by the compiler) for types which are
// safe to be sent between threads
use std::marker::Send;
// multiple producer single consumer concurrency model facilities
use std::sync::mpsc;
// thread module
use std::thread;
// we use this to manage the solving process stop condition
use crate::solving_status::SolvingStatus;
// some traits we need to be in scope
use crate::traits::{BBProblem, Solution};
// type alias for return value of functions that can return an error
use crate::Result;
// types aliases for easy access of associated types from the
// BBProblem trait
type Sol<T> = <T as BBProblem>::Sol;
type SolCost<T> = <Sol<T> as Solution>::SolCost;
// message to be delivired to a worker thread: instructs worker to
// either finish execution or treat a relaxed subproblem
enum Message<T: BBProblem> {
    Finish,
    Treat(T, T::Sol),
}
// message to be delivred from a worker thread to the main thread:
// indicates that either a feasible solution was found or a subproblem
// has been branched into some other subproblems
enum SolvingInformation<T: BBProblem> {
    Feasible(T::Sol),
    Branched(Vec<(T, T::Sol)>),
}
// type alias for a pair of sender and receiver representing a channel
// from a worker thread to the main thread. Besides
// SolvingInformation, the solution cost of the relaxed solution of
// the subproblem just treated is part of the message to the main thread
type ChannelFromWorker<T> = (
    mpsc::Sender<(SolCost<T>, SolvingInformation<T>)>,
    mpsc::Receiver<(SolCost<T>, SolvingInformation<T>)>,
);
// type alias for a pair of sender and receiver representing a channel
// from the main thread to a worker thread
type ChannelToWorker<T> = (mpsc::Sender<Message<T>>, mpsc::Receiver<Message<T>>);
// this function represents the behavior of a worker thread. Its
// arguments are a sender to the main thread and a receiver to get
// messages from the main thread. Returns an error in case something
// goes wrong while sending or receiving messages
fn worker<'a, T: BBProblem>(
    sender: mpsc::Sender<(SolCost<T>, SolvingInformation<T>)>,
    receiver: mpsc::Receiver<Message<T>>,
) -> Result<'a> {
    // this thread only finishes when it receives the corresponding message
    loop {
        // try receiving a message from the main thread
        let msg = receiver.recv().expect(
            "worker: something went wrong when receiving a message from parallel::branch_and_bound",
        );
        match msg {
            // if message tells to finish execution, returns Ok
            Message::Finish => return Ok(()),
            // if message brings a problem and its relaxed
            // solution, treat that information
            Message::Treat(problem, relaxed_solution) => {
                // if the relaxed solution is feasible
                if relaxed_solution.is_feasible() {
                    // send a message to the main thread telling
                    // just that
                    sender.send((relaxed_solution.get_cost(), SolvingInformation::Feasible(relaxed_solution))).expect(
                        "worker: something went wrong when sending a message to parallel::branch_and_bound"
                    );
                } else {
                    // in case relaxed solution is infeasible,
                    // prepare to branch the subproblem
                    let mut relaxed_problems = Vec::new();
                    // for each subproblem, store it and its
                    // relaxed solution in relaxed_problems
                    for subproblem in problem.get_subproblems(&relaxed_solution) {
                        let relaxed_sol = subproblem.solve_relaxation();

                        relaxed_problems.push((*subproblem, relaxed_sol));
                    }
                    // send a message to the main thread
                    // containing new subproblems (and their
                    // relaxad solutions) to be distributed among
                    // worker threads
                    sender.send((relaxed_solution.get_cost(), SolvingInformation::Branched(relaxed_problems))).expect(
                        "worker: something went wrong when sending a message to parallel::branch_and_bound"
                    );
                }
            }
        }
    }
}
// this is a facility to manage the lower bound of the branch and
// bound process by counting the frequency of each active lower bound,
// which are lower bounds from problems being treated by worker
// threads
struct LowerBoundManager<T: BBProblem> {
    // associates each lower bound value to the number of active
    // subproblems that have it as relaxed solution cost
    lower_bounds: HashMap<SolCost<T>, u64>,
}
// constructor and methods for LowerBoundManager
impl<T: BBProblem> LowerBoundManager<T>
where
    SolCost<T>: Hash,
{
    // returns a new LowerBoundManager
    fn new() -> Self {
        let lower_bounds = HashMap::new();

        Self { lower_bounds }
    }
    // increases the count of lb
    fn register_lower_bound(&mut self, lb: SolCost<T>) {
        // if lb is not a key yet, treat its associated value as zero
        let count = *self.lower_bounds.get(&lb).unwrap_or(&0);
        // increases lb count
        self.lower_bounds.insert(lb, count + 1);
    }
    // decreases the count of lb
    fn discard_lower_bound(&mut self, lb: SolCost<T>) {
        // we need to clone the return of get, otherwise count becomes
        // a const ref forbidding us from call insert
        match self.lower_bounds.get(&lb).cloned() {
            None => (),
            Some(count) => {
                // if frequency of lb is one, remove it from the
                // mapping
                if count == 1 {
                    self.lower_bounds.remove(&lb);
                } else {
                    // otherwise just decrease its count
                    self.lower_bounds.insert(lb, count - 1);
                }
            }
        }
    }
    // returns a reference to the minimum lower bound currently in
    // storage. In case no lower bound is in storage, returns None
    fn min_lower_bound(&self) -> Option<&SolCost<T>> {
        self.lower_bounds.keys().min()
    }
}
/// parallel branch and bound implementation: given a BBProblem type
/// instance and the number of worker threads to be spawned, returns a
/// solution in case instance is feasible, otherwise returns None
pub fn branch_and_bound<T: 'static + BBProblem + Send>(
    problem: T,
    num_workers: usize,
) -> Option<T::Sol>
where
    Sol<T>: Send,
    SolCost<T>: Hash + Clone + Send,
{
    // this is to keep track of the lower bounds
    let mut lb_manager: LowerBoundManager<T> = LowerBoundManager::new();
    // number of subproblems currently being treated by worker threads
    let mut open_subproblems: u64 = 0;
    // status of branch and bound solving process
    let mut status: SolvingStatus<T> = SolvingStatus::new();
    // a channel for the worker threads send messages to this thread
    let (worker_sender, main_receiver): ChannelFromWorker<T> = mpsc::channel();
    // these are to store worker thread communiaction objects
    let mut main_sender = Vec::new();
    let mut worker_handler = Vec::new();
    main_sender.reserve(num_workers);
    worker_handler.reserve(num_workers);
    // for each worker thread
    for _ in 0..num_workers {
        // creates a channel for it
        let (sender, receiver): ChannelToWorker<T> = mpsc::channel();
        // clones worker_sender so it can send
        // messages to this thread
        let cloned_worker_sender = worker_sender.clone();
        // stores sender in order to send messages to it
        main_sender.push(sender);
        // finally spawns it, giving to it a sender and a receiver
        let handler = thread::spawn(move || {
            worker(cloned_worker_sender, receiver).unwrap();
        });
        // keeps its handler for a later join
        worker_handler.push(handler);
    }
    // an iterator cycling through each worker thread sender
    let mut main_sender_cycle = main_sender.iter().cycle();
    // this lambda sends a message to the next sender in main_sender_cycle
    let mut cyclic_send = |msg| {
        main_sender_cycle.next().unwrap().send(msg).expect(
            "parallel::branch_and_bound: something went wrong when sending a message to one of the worker threads"
        );
    };
    // relaxed solution of problem
    let root_relaxed_sol = problem.solve_relaxation();
    // registers its relaxed solution cost as our initial lower bound
    lb_manager.register_lower_bound(root_relaxed_sol.get_cost());
    status.set_lower_bound(root_relaxed_sol.get_cost()).unwrap();
    // sends problem and its relaxed solution to be treated by the
    // worker threads
    cyclic_send(Message::Treat(problem, root_relaxed_sol));
    // accounts for problem that has just been sent to the worker threads
    open_subproblems += 1;
    // while solving is not finished and there are subproblems to be
    // treated by the worker threads
    while !status.finished() && open_subproblems > 0 {
        // tries to receive a message from the working threads
        let (parent_relaxed_sol_cost, solving_information) = main_receiver.recv().expect(
            "parallel::branch_and_bound: something went wrong when receiving a message from a worker thread"
        );
        // accounsts from a subproblem that has been reported
        // back by the worker threads
        open_subproblems -= 1;
        // since this reported subproblem has been closed,
        // discard its relaxed solution cost
        lb_manager.discard_lower_bound(parent_relaxed_sol_cost);
        // treats solving_information
        match solving_information {
            // in case reported subproblem is feasible,
            // compares its solution with the best solution
            // found so far
            SolvingInformation::Feasible(solution) => {
                if let Some(best_sol) = status.best_solution() {
                    if solution.get_cost() < best_sol.get_cost() {
                        status.set_best_solution(solution).unwrap();
                    }
                } else {
                    status.set_best_solution(solution).unwrap();
                }
            }
            // in case reported problem is infeasible, treat
            // its subproblems
            SolvingInformation::Branched(relaxed_subproblems) => {
                // for each of its subprolems and the
                // corresponding relaxed solution
                for (problem, relaxed_sol) in relaxed_subproblems {
                    // takes the relaxed solution cost into
                    // account as a lower bound
                    lb_manager.register_lower_bound(relaxed_sol.get_cost());
                    // sends the subproblem and the
                    // corresponding relaxed solution to the
                    // worker threads
                    cyclic_send(Message::Treat(problem, relaxed_sol));
                    // accounts for a subproblem sent to the
                    // worker threads
                    open_subproblems += 1;
                }
            }
        }
        // updates lower bound to be the minimum active lower bound
        if let Some(min_lb) = lb_manager.min_lower_bound().cloned() {
            status.set_lower_bound(min_lb).unwrap();
        }
    }
    // once the solving process has ended, sends a finishing message
    // to each worker thread
    for _ in 0..num_workers {
        cyclic_send(Message::Finish);
    }
    // waits for each worker thread to finish
    for handler in worker_handler {
        handler.join().unwrap();
    }
    // returns best solution found during the process
    status.extract_best_solution()
}
