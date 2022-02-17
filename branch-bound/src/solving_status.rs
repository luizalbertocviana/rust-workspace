// BBProblem and Solution traits
use crate::traits::{BBProblem, Solution};
// Result type alias
use crate::Result;
// Aliases for getting Sol and SolCost out of BBProblem T
type Sol<T> = <T as BBProblem>::Sol;
type SolCost<T> = <Sol<T> as Solution>::SolCost;
/// SolvingStatus: keeps track of current lower bound and best known
/// solution
pub struct SolvingStatus<T: BBProblem> {
    lower_bound: Option<SolCost<T>>,
    upper_bound: Option<SolCost<T>>,

    best_solution: Option<Sol<T>>,
}

// constructors
impl<T: BBProblem> SolvingStatus<T> {
    /// returns empty SolvingStatus object
    pub fn new() -> Self {
        let lower_bound = None;
        let upper_bound = None;
        let best_solution = None;

        Self {
            lower_bound,
            upper_bound,
            best_solution,
        }
    }
}

// accessors
impl<T: BBProblem> SolvingStatus<T> {
    /// determines whether SolvingStatus indicates a finished solving
    /// process
    pub fn finished(&self) -> bool {
        match (&self.lower_bound, &self.upper_bound) {
            (None, _) => false,
            (_, None) => false,
            (Some(lb), Some(ub)) => *lb == *ub,
        }
    }
    /// gets reference to Option possibly containing best solution
    pub fn best_solution(&self) -> &Option<Sol<T>> {
        &self.best_solution
    }
}

// modifiers
impl<T: BBProblem> SolvingStatus<T> {
    /// sets lower bound. Returns error in case lb is greater than
    /// upper bound
    pub fn set_lower_bound(&mut self, lb: SolCost<T>) -> Result {
        if let Some(ub) = &self.upper_bound {
            if lb > *ub {
                Err("SolvingStatus: attempt to set a lower bound greater than current upper bound")
            } else {
                self.lower_bound.replace(lb);

                Ok(())
            }
        } else {
            self.lower_bound.replace(lb);

            Ok(())
        }
    }
    /// sets best solution. Returns error in case sol has cost less
    /// than lower bound
    pub fn set_best_solution(&mut self, sol: Sol<T>) -> Result {
        let sol_cost = sol.get_cost();

        if let Some(ub) = &self.upper_bound {
            if *ub > sol_cost {
                self.best_solution.replace(sol);
                self.upper_bound.replace(sol_cost);

                Ok(())
            } else {
                Err("SolvingStatus: attempt to set a solution worse than current best solution")
            }
        } else {
            self.best_solution.replace(sol);
            self.upper_bound.replace(sol_cost);

            Ok(())
        }
    }
    /// moves best solution out of SolvingStatus. Intended usage is
    /// after solving process is finished
    pub fn extract_best_solution(self) -> Option<Sol<T>> {
        self.best_solution
    }
}
