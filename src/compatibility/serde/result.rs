use super::{
    super::super::{common::*, into::*, problem::*},
    problem::*,
};

use std::error::Error;

//
// IntoSerdeProblemResult
//

/// Maps [Err] into a [SerdeProblem].
pub trait IntoSerdeProblemResult<OkT> {
    /// Maps [Err] into a [SerdeProblem] via [SerializeError].
    ///
    /// You probably want to call [from_serde_problem](FromSerdeProblemResult::from_serde_problem)
    /// to eventually map it back and avoid re-wrapping it and thus losing the causation chain.
    fn into_serde_serialize_problem(self) -> Result<OkT, SerdeProblem>;

    /// Maps [Err] into a [SerdeProblem] via [DeserializeError].
    ///
    /// You probably want to call [from_serde_problem](FromSerdeProblemResult::from_serde_problem)
    /// to eventually map it back and avoid re-wrapping it and thus losing the causation chain.
    fn into_serde_deserialize_problem(self) -> Result<OkT, SerdeProblem>;
}

impl<OkT, ErrorT> IntoSerdeProblemResult<OkT> for Result<OkT, ErrorT>
where
    ErrorT: 'static + Error + Send + Sync,
{
    fn into_serde_serialize_problem(self) -> Result<OkT, SerdeProblem> {
        self.map_err(|error| {
            error
                .into_problem()
                .via(SerializeError::new("serde"))
                .into()
        })
    }

    fn into_serde_deserialize_problem(self) -> Result<OkT, SerdeProblem> {
        self.map_err(|error| {
            error
                .into_problem()
                .via(DeserializeError::new("serde"))
                .into()
        })
    }
}

//
// FromSerdeProblemResult
//

/// Maps a [SerdeProblem] [Err] into a [Problem].
pub trait FromSerdeProblemResult<OkT> {
    /// Maps a [SerdeProblem] [Err] into a [Problem].
    fn from_serde_problem(self) -> Result<OkT, Problem>;
}

impl<OkT> FromSerdeProblemResult<OkT> for Result<OkT, SerdeProblem> {
    fn from_serde_problem(self) -> Result<OkT, Problem> {
        self.map_err(|serde_problem| serde_problem.problem)
    }
}
