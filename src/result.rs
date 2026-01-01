use super::problem::*;

use std::{any::*, error::Error, io};

//
// IntoProblemResult
//

/// Maps [Err] into a [Problem].
pub trait IntoProblemResult<OkT> {
    /// Maps [Err] into a [Problem].
    fn into_problem(self) -> Result<OkT, Problem>;
}

impl<OkT> IntoProblemResult<OkT> for Result<OkT, Problem> {
    fn into_problem(self) -> Result<OkT, Problem> {
        self
    }
}

impl<OkT, ErrorT> IntoProblemResult<OkT> for Result<OkT, ErrorT>
where
    ErrorT: 'static + Error + Send + Sync,
{
    fn into_problem(self) -> Result<OkT, Problem> {
        self.map_err(Problem::from)
    }
}

//
// ProblemResult
//

/// Problemo extensions for [Result].
pub trait ProblemResult<OkT> {
    /// Adds the error to the top of the causation chain.
    fn via<ErrorT>(self, error: ErrorT) -> Result<OkT, Problem>
    where
        ErrorT: 'static + Error + Send + Sync;

    /// Adds the error to the top of the causation chain.
    fn map_via<ErrorT, FromT>(self, from: FromT) -> Result<OkT, Problem>
    where
        ErrorT: 'static + Error + Send + Sync,
        FromT: FnOnce() -> ErrorT;

    /// Attach to the top cause.
    fn with<AttachmentT>(self, attachment: AttachmentT) -> Result<OkT, Problem>
    where
        AttachmentT: Any + Send + Sync;

    /// Attach to the top cause.
    fn map_with<AttachmentT, FromT>(self, from: FromT) -> Result<OkT, Problem>
    where
        AttachmentT: Any + Send + Sync,
        FromT: FnOnce() -> AttachmentT;

    /// Attach to the top cause if [Some].
    fn maybe_with<AttachmentT>(self, attachment: Option<AttachmentT>) -> Result<OkT, Problem>
    where
        AttachmentT: Any + Send + Sync;

    /// Attach to the top cause.
    fn maybe_map_with<AttachmentT, FromT>(self, from: FromT) -> Result<OkT, Problem>
    where
        AttachmentT: Any + Send + Sync,
        FromT: FnOnce() -> Option<AttachmentT>;

    /// Attach backtrace.
    #[cfg(feature = "backtrace")]
    fn with_backtrace(self) -> Result<OkT, Problem>;

    /// Into [io::Error] with [ErrorKind::Other](io::ErrorKind::Other).
    fn into_io_error(self) -> io::Result<OkT>;
}

impl<ResultT, OkT> ProblemResult<OkT> for ResultT
where
    ResultT: IntoProblemResult<OkT>,
{
    fn via<ViaErrorT>(self, error: ViaErrorT) -> Result<OkT, Problem>
    where
        ViaErrorT: 'static + Error + Send + Sync,
    {
        self.into_problem().map_err(|problem| problem.via(error))
    }

    fn map_via<ErrorT, FromT>(self, from: FromT) -> Result<OkT, Problem>
    where
        ErrorT: 'static + Error + Send + Sync,
        FromT: FnOnce() -> ErrorT,
    {
        self.into_problem().map_err(|problem| problem.via(from()))
    }

    fn with<AttachmentT>(self, attachment: AttachmentT) -> Result<OkT, Problem>
    where
        AttachmentT: Any + Send + Sync,
    {
        self.into_problem()
            .map_err(|problem| problem.with(attachment))
    }

    fn map_with<AttachmentT, FromT>(self, from: FromT) -> Result<OkT, Problem>
    where
        AttachmentT: Any + Send + Sync,
        FromT: FnOnce() -> AttachmentT,
    {
        self.into_problem().map_err(|problem| problem.with(from()))
    }

    fn maybe_with<AttachmentT>(self, attachment: Option<AttachmentT>) -> Result<OkT, Problem>
    where
        AttachmentT: Any + Send + Sync,
    {
        self.into_problem()
            .map_err(|problem| problem.maybe_with(attachment))
    }

    fn maybe_map_with<AttachmentT, FromT>(self, from: FromT) -> Result<OkT, Problem>
    where
        AttachmentT: Any + Send + Sync,
        FromT: FnOnce() -> Option<AttachmentT>,
    {
        self.into_problem()
            .map_err(|problem| problem.maybe_with(from()))
    }

    #[cfg(feature = "backtrace")]
    fn with_backtrace(self) -> Result<OkT, Problem> {
        self.into_problem()
            .map_err(|problem| problem.with_backtrace())
    }

    fn into_io_error(self) -> io::Result<OkT> {
        self.into_problem().map_err(|problem| problem.into())
    }
}

//
// MapIntoProblemResult
//

/// Map [Err] into a problem.
pub trait MapIntoProblemResult<OkT, ErrorT> {
    /// Map [Err] into a problem.
    fn map_into_problem<MappedErrorT, MapT>(self, map: MapT) -> Result<OkT, Problem>
    where
        MappedErrorT: 'static + Error + Send + Sync,
        MapT: FnOnce(ErrorT) -> MappedErrorT;
}

impl<OkT, ErrorT> MapIntoProblemResult<OkT, ErrorT> for Result<OkT, ErrorT> {
    fn map_into_problem<MappedErrorT, MapT>(self, map: MapT) -> Result<OkT, Problem>
    where
        MappedErrorT: 'static + Error + Send + Sync,
        MapT: FnOnce(ErrorT) -> MappedErrorT,
    {
        self.map_err(map).into_problem()
    }
}
