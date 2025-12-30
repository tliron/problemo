use super::{common::*, problem::*};

use std::error::Error;

//
// IntoProblem
//

/// Into problem.
pub trait IntoProblem {
    /// Into problem.
    fn into_problem(self) -> Problem;
}

impl<ToStringT> IntoProblem for ToStringT
where
    ToStringT: ToString,
{
    fn into_problem(self) -> Problem {
        MessageError::new(self).into()
    }
}

//
// IntoProblemResult
//

/// Into [Result]\<_, [Problem]\>.
pub trait IntoProblemResult<OkT> {
    /// Into [Result]\<_, [Problem]\>.
    fn into_problem(self) -> Result<OkT, Problem>;
}

impl<OkT> IntoProblemResult<OkT> for Result<OkT, Problem> {
    fn into_problem(self) -> Result<OkT, Problem> {
        self
    }
}

impl<OkT, ErrorT> IntoProblemResult<OkT> for Result<OkT, ErrorT>
where
    ErrorT: 'static + Error,
{
    fn into_problem(self) -> Result<OkT, Problem> {
        self.map_err(Problem::from)
    }
}

//
// MapIntoProblemResult
//

/// Map [Err] into problem.
pub trait MapIntoProblemResult<OkT, ErrorT> {
    /// Map [Err] into problem.
    fn map_into_problem<MappedErrorT, MapT>(self, map: MapT) -> Result<OkT, Problem>
    where
        MappedErrorT: 'static + Error,
        MapT: FnOnce(ErrorT) -> MappedErrorT;

    /// Map [Err] into a [MessageError] problem.
    fn into_message_problem(self) -> Result<OkT, Problem>
    where
        ErrorT: ToString;

    /// Map [Err] into a [ConcurrencyError] problem.
    fn into_concurrency_problem(self) -> Result<OkT, Problem>
    where
        ErrorT: ToString;
}

impl<OkT, ErrorT> MapIntoProblemResult<OkT, ErrorT> for Result<OkT, ErrorT> {
    fn map_into_problem<MappedErrorT, ConvertT>(self, map: ConvertT) -> Result<OkT, Problem>
    where
        MappedErrorT: 'static + Error,
        ConvertT: FnOnce(ErrorT) -> MappedErrorT,
    {
        self.map_err(map).into_problem()
    }

    fn into_message_problem(self) -> Result<OkT, Problem>
    where
        ErrorT: ToString,
    {
        self.map_into_problem(MessageError::new)
    }

    fn into_concurrency_problem(self) -> Result<OkT, Problem>
    where
        ErrorT: ToString,
    {
        self.map_into_problem(ConcurrencyError::new)
    }
}
