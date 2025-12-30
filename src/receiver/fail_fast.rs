use super::{super::problem::*, receiver::*};

//
// FailFast
//

/// [ProblemReceiver] that fails on the first given problem.
pub struct FailFast;

impl ProblemReceiver for FailFast {
    fn give(&mut self, problem: Problem) -> Result<(), Problem> {
        Err(problem)
    }
}
