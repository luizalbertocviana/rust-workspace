// solving_status module
mod solving_status;
// traits module
mod traits;
// serial branch and bound module
mod serial;
// parallel branch and bound
mod parallel;
// some exported traits
pub use crate::traits::{BBProblem, RelaxedProblemPool, Solution, SolutionCost, Variable};
// alias to return type of functions that can return an error
type Result<'a> = std::result::Result<(), &'a str>;

/// serial bracnh and bound
pub use serial::branch_and_bound as serial;
/// parallel branch and bound
pub use parallel::branch_and_bound as parallel;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
