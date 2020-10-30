type Var<T> = <T as Solution>::Var;
type ValType<T> = <Var<T> as Variable>::ValueType;

type Prob<T> = <T as RelaxedProblemPool>::Prob;
type Sol<T> = <Prob<T> as BBProblem>::Sol;

pub trait Variable {
    type ValueType;
}

pub trait SolutionCost: Ord {
    fn max_value() -> Self;
    fn min_value() -> Self;
}

pub trait Solution {
    type Var: Variable;
    type SolCost: SolutionCost;

    fn is_feasible(&self) -> bool;

    fn get_cost(&self) -> Self::SolCost;
    fn get_value(var: &Self::Var) -> ValType<Self>;
}

pub trait BBProblem {
    type Sol: Solution;
    type SubproblemIterator: Iterator<Item = Box<Self>>;

    fn solve_relaxation(&self) -> Self::Sol;

    fn get_subproblems(&self) -> Self::SubproblemIterator;
}

pub trait RelaxedProblemPool {
    type Prob: BBProblem;

    fn new() -> Self;
    fn add(&mut self, p: Self::Prob, s: Sol<Self>);
    fn empty(&self) -> bool;
    fn extract(&mut self) -> Option<(Self::Prob, Sol<Self>)>;
    fn min_relaxed_solution(&self) -> Option<&Sol<Self>>;
}
