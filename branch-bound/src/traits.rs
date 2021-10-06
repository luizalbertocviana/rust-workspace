// type aliases to extract Var and ValueType out of Solution T
type Var<T> = <T as Solution>::Var;
type ValType<T> = <Var<T> as Variable>::ValueType;
// type aliases to extract Prob and Sol out of RelaxedProblemPool T
type Prob<T> = <T as RelaxedProblemPool>::Prob;
type Sol<T> = <Prob<T> as BBProblem>::Sol;
/// trait for types representing variables
pub trait Variable {
    /// type of a variable value
    type ValueType;
}
/// trait for types representing solution costs. Those types should
/// also implement Ord trait
pub trait SolutionCost: Ord {}
/// trait for types representing a solution
pub trait Solution {
    /// variable type
    type Var: Variable;
    /// solution cost type
    type SolCost: SolutionCost;
    /// determines whether solution is feasible
    fn is_feasible(&self) -> bool;
    /// returns cost of solution
    fn get_cost(&self) -> Self::SolCost;
    /// returns value assigned to var
    fn get_value(&self, var: &Self::Var) -> ValType<Self>;
}
/// trait for types representing problem instances suitable for branch
/// and bound
pub trait BBProblem {
    /// solution type: needs to implement Send trait in order to be
    /// used by parallel branch and bound
    type Sol: Solution;
    /// subproblem iterator type
    type SubproblemIterator: Iterator<Item = Box<Self>>;
    /// solves problem relaxation, returning its relaxed solution
    fn solve_relaxation(&self) -> Self::Sol;
    /// returns iterator to subproblems
    fn get_subproblems(&self, solution: &Self::Sol) -> Self::SubproblemIterator;
}
/// trait for types representing data structures capable of managing
/// subproblems for a branch and bound procedure
pub trait RelaxedProblemPool {
    /// problem type
    type Prob: BBProblem;
    /// creates an empty problem pool
    fn new() -> Self;
    /// adds problem and its relaxed solution to the pool
    fn add(&mut self, p: Self::Prob, s: Sol<Self>);
    /// determines whether pool is empty
    fn empty(&self) -> bool;
    /// extract problem and its relaxed solution from the pool
    fn extract(&mut self) -> Option<(Self::Prob, Sol<Self>)>;
    /// gets a reference to the relaxed solution with minimum cost
    fn min_relaxed_solution(&self) -> Option<&Sol<Self>>;
}
