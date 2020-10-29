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
    fn get_value(var: &Self::Var) -> <<Self as Solution>::Var as Variable>::ValueType;
}

pub trait BBProblem {
    type Sol: Solution;
    type SubproblemIterator: Iterator;
    
    fn solve_relaxation(&self) -> Self::Sol;

    fn get_subproblems(&self) -> Self::SubproblemIterator;
}
