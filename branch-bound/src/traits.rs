pub trait Variable {
    type ValueType;
}

pub trait Solution {
    type Var: Variable;
    type SolutionCost: PartialOrd;

    fn is_feasible(&self) -> bool;

    fn get_cost(&self) -> Self::SolutionCost;
    fn get_value(var: &Self::Var) -> <<Self as Solution>::Var as Variable>::ValueType;
}

pub trait BBProblem {
    type Sol: Solution;
    type SubproblemIterator: Iterator;
    
    fn solve_relaxation(&self) -> Self::Sol;

    fn get_subproblems() -> Self::SubproblemIterator;
}
