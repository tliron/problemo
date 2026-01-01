use super::super::{cause::*, problem::*};

//
// AnyhowIntoProblem
//

/// Into problem.
pub trait AnyhowIntoProblem {
    /// Into problem.
    fn into_problem(self) -> Problem;
}

impl AnyhowIntoProblem for anyhow::Error {
    fn into_problem(self) -> Problem {
        let mut problem = Problem::default();
        problem
            .causes
            .push_back(Cause::new(self.into_boxed_dyn_error()));
        problem
    }
}

//
// AnyhowIntoProblemResult
//

/// Into [Result]\<_, [Problem]\>.
pub trait AnyhowIntoProblemResult<OkT> {
    /// Into [Result]\<_, [Problem]\>.
    fn into_problem(self) -> Result<OkT, Problem>;
}

impl<OkT> AnyhowIntoProblemResult<OkT> for anyhow::Result<OkT> {
    fn into_problem(self) -> Result<OkT, Problem> {
        self.map_err(|error| error.into_problem())
    }
}
