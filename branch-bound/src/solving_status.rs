use crate::traits::{BBProblem, Solution};
use crate::Result;

type Sol<T> = <T as BBProblem>::Sol;
type SolCost<T> = <Sol<T> as Solution>::SolCost;

pub struct SolvingStatus<T: BBProblem> {
    lower_bound: Option<SolCost<T>>,

    best_solution: Option<Sol<T>>,
}

// constructors
impl<T: BBProblem> SolvingStatus<T> {
    pub fn new() -> Self {
        let lower_bound = None;
        let best_solution = None;

        Self {
            lower_bound,
            best_solution,
        }
    }
}

// accessors
impl<T: BBProblem> SolvingStatus<T> {
    pub fn finished(&self) -> bool {
        match (&self.lower_bound, &self.best_solution) {
            (None, _) => false,
            (_, None) => false,
            (Some(lb), Some(sol)) => *lb == sol.get_cost(),
        }
    }

    pub fn lower_bound(&self) -> &Option<SolCost<T>> {
        &self.lower_bound
    }

    pub fn best_solution(&self) -> &Option<Sol<T>> {
        &self.best_solution
    }
}

// modifiers
impl<T: BBProblem> SolvingStatus<T> {
    pub fn set_lower_bound(&mut self, lb: SolCost<T>) -> Result {
        if let Some(sol) = &self.best_solution {
            if lb > sol.get_cost() {
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

    pub fn set_best_solution(&mut self, sol: Sol<T>) -> Result {
        if let Some(best_sol) = &self.best_solution {
            if best_sol.get_cost() > sol.get_cost() {
                self.best_solution.replace(sol);

                Ok(())
            } else {
                Err("SolvingStatus: attempt to set a solution worse than current best solution")
            }
        } else {
            self.best_solution.replace(sol);

            Ok(())
        }
    }

    pub fn extract_best_solution(self) -> Option<Sol<T>> {
        self.best_solution
    }
}
