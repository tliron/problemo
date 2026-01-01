use super::problem::*;

use std::error::*;

//
// IntoProblem
//

/// Into a [Problem].
pub trait IntoProblem {
    /// Into a [Problem].
    fn into_problem(self) -> Problem;
}

impl<ErrorT> IntoProblem for ErrorT
where
    ErrorT: 'static + Error + Send + Sync,
{
    fn into_problem(self) -> Problem {
        self.into()
    }
}
