use super::{into::*, problem::*};

use std::{any::*, error::Error};

//
// ProblemResult
//

/// Problem result.
pub trait ProblemResult<OkT> {
    /// Adds the error to the front.
    fn via<ErrorT>(self, error: ErrorT) -> Result<OkT, Problem>
    where
        ErrorT: 'static + Error;

    /// Attach to the top cause.
    fn with<AttachmentT>(self, attachment: AttachmentT) -> Result<OkT, Problem>
    where
        AttachmentT: Any + Send + Sync;

    /// Attach to the top cause if [Some].
    fn maybe_with<AttachmentT>(self, attachment: Option<AttachmentT>) -> Result<OkT, Problem>
    where
        AttachmentT: Any + Send + Sync;

    /// Attach backtrace.
    fn with_backtrace(self) -> Result<OkT, Problem>;
}

impl<ResultT, OkT> ProblemResult<OkT> for ResultT
where
    ResultT: IntoProblemResult<OkT>,
{
    fn via<ViaErrorT>(self, error: ViaErrorT) -> Result<OkT, Problem>
    where
        ViaErrorT: 'static + Error,
    {
        self.into_problem().map_err(|e| e.via(error))
    }

    fn with<AttachmentT>(self, attachment: AttachmentT) -> Result<OkT, Problem>
    where
        AttachmentT: Any + Send + Sync,
    {
        self.into_problem().map_err(|error| error.with(attachment))
    }

    fn maybe_with<AttachmentT>(self, attachment: Option<AttachmentT>) -> Result<OkT, Problem>
    where
        AttachmentT: Any + Send + Sync,
    {
        self.into_problem()
            .map_err(|error| error.maybe_with(attachment))
    }

    fn with_backtrace(self) -> Result<OkT, Problem> {
        self.into_problem().map_err(|error| error.with_backtrace())
    }
}
