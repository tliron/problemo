use super::super::problem::*;

use std::{error::*, fmt};

//
// ProblemAsError
//

/// Straightforward wrapper for [Problem] that adds an implementation of [Error].
///
/// Take care to avoid adding it into a [Problem]'s causation chain.
pub struct ProblemAsError {
    /// Problem.
    pub problem: Problem,
}

impl fmt::Debug for ProblemAsError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.problem, formatter)
    }
}

impl fmt::Display for ProblemAsError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.problem, formatter)
    }
}

impl Error for ProblemAsError {}

impl From<Problem> for ProblemAsError {
    fn from(problem: Problem) -> Self {
        Self { problem }
    }
}
