use super::{super::problem::*, common::*};

//
// IntoCommonProblem
//

/// Into a common [Problem].
pub trait IntoCommonProblem {
    /// Into a [MessageError] problem.
    fn into_message_problem(self) -> Problem;

    /// Into a [ThreadError] problem.
    fn into_thread_problem(self) -> Problem;
}

impl<ToStringT> IntoCommonProblem for ToStringT
where
    ToStringT: ToString,
{
    fn into_message_problem(self) -> Problem {
        MessageError::new(self).into()
    }

    fn into_thread_problem(self) -> Problem {
        ThreadError::new(self).into()
    }
}
