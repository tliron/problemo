use super::{
    super::{problem::*, result::*},
    common::*,
};

//
// MapIntoCommonProblemResult
//

/// Map [Err] into a common error problem.
pub trait MapIntoCommonProblemResult<OkT, ErrorT> {
    /// Map [Err] into a [MessageError] problem.
    fn into_message_problem(self) -> Result<OkT, Problem>
    where
        ErrorT: ToString;

    /// Map [Err] into a [ThreadError] problem.
    fn into_thread_problem(self) -> Result<OkT, Problem>
    where
        ErrorT: ToString;
}

impl<OkT, ErrorT> MapIntoCommonProblemResult<OkT, ErrorT> for Result<OkT, ErrorT> {
    fn into_message_problem(self) -> Result<OkT, Problem>
    where
        ErrorT: ToString,
    {
        self.map_into_problem(MessageError::new)
    }

    fn into_thread_problem(self) -> Result<OkT, Problem>
    where
        ErrorT: ToString,
    {
        self.map_into_problem(ThreadError::new)
    }
}
