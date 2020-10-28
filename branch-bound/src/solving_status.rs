use crate::Result;
use crate::traits::{BBProblem, Solution, SolutionCost};

type SolCost<T> = <<T as BBProblem>::Sol as Solution>::SolCost;

pub struct SolvingStatus<T: BBProblem> {
    lower_bound: SolCost<T>,
    upper_bound: SolCost<T>,

    valid_upper_bound: bool
}

// constructors
impl<T: BBProblem> SolvingStatus<T> {
    pub fn new() -> Self {
        let lower_bound = SolCost::<T>::min_value();
        let upper_bound = SolCost::<T>::max_value();
        let valid_upper_bound = false;

        Self {lower_bound, upper_bound, valid_upper_bound}
    }
}

// accessors
impl<T: BBProblem> SolvingStatus<T> {
    pub fn finished(&self) -> bool {
        self.valid_upper_bound && self.lower_bound == self.upper_bound
    }
}

// modifiers
impl<T: BBProblem> SolvingStatus<T> {
    pub fn set_lower_bound(&mut self, lb: SolCost<T>) -> Result {
        if lb <= self.upper_bound {
            self.lower_bound = lb;

            Ok(())
        }
        else {
            Err("SolvingStatus: attempt to set a lower bound greater than current upper bound")
        }
    }

    pub fn set_upper_bound(&mut self, ub: SolCost<T>) -> Result {
        if ub >= self.lower_bound {
            self.upper_bound = ub;

            Ok(())
        }
        else {
            Err("SolvingStatus: attempt to set an upper bound less than current lower bound")
        }
    }
}
