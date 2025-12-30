use super::super::{into::*, problem::*};

use std::{error::Error, fmt, process::*};

//
// ExitCodeAttachment
//

/// Exit code [Problem] attachment.
///
/// Although it can be attached to any problem you can use [ExitError] for simple messages.
#[derive(Clone, Debug)]
pub struct ExitCodeAttachment {
    /// Exit code.
    pub exit_code: ExitCode,
}

impl ExitCodeAttachment {
    /// Failure.
    pub fn failure() -> Self {
        ExitCode::FAILURE.into()
    }

    /// Success.
    pub fn success() -> Self {
        ExitCode::SUCCESS.into()
    }
}

impl<ExitCodeT> From<ExitCodeT> for ExitCodeAttachment
where
    ExitCodeT: Into<ExitCode>,
{
    fn from(exit_code: ExitCodeT) -> Self {
        Self {
            exit_code: exit_code.into(),
        }
    }
}

//
// WithExitCode
//

/// With exit code.
pub trait WithExitCode {
    /// With [ExitCodeAttachment].
    fn with_exit_code<ExitCodeT>(self, exit_code: ExitCodeT) -> Self
    where
        ExitCodeT: Into<ExitCode>;

    /// With failure [ExitCodeAttachment].
    fn with_failure_exit_code(self) -> Self;

    /// With success [ExitCodeAttachment].
    fn with_success_exit_code(self) -> Self;
}

impl WithExitCode for Problem {
    fn with_exit_code<ExitCodeT>(self, exit_code: ExitCodeT) -> Self
    where
        ExitCodeT: Into<ExitCode>,
    {
        self.with(ExitCodeAttachment::from(exit_code))
    }

    fn with_failure_exit_code(self) -> Self {
        self.with(ExitCodeAttachment::failure())
    }

    fn with_success_exit_code(self) -> Self {
        self.with(ExitCodeAttachment::success())
    }
}

//
// WithExitCodeResult
//

/// With exit code.
pub trait WithExitCodeResult<OkT> {
    /// With [ExitCodeAttachment].
    fn with_exit_code<ExitCodeT>(self, exit_code: ExitCodeT) -> Result<OkT, Problem>
    where
        ExitCodeT: Into<ExitCode>;

    /// With failure [ExitCodeAttachment].
    fn with_failure_exit_code(self) -> Result<OkT, Problem>;

    /// With success [ExitCodeAttachment].
    fn with_success_exit_code(self) -> Result<OkT, Problem>;
}

impl<ResultT, OkT> WithExitCodeResult<OkT> for ResultT
where
    ResultT: IntoProblemResult<OkT>,
{
    fn with_exit_code<ExitCodeT>(self, exit_code: ExitCodeT) -> Result<OkT, Problem>
    where
        ExitCodeT: Into<ExitCode>,
    {
        self.into_problem()
            .map_err(|problem| problem.with_exit_code(exit_code))
    }

    fn with_failure_exit_code(self) -> Result<OkT, Problem> {
        self.into_problem()
            .map_err(|problem| problem.with_failure_exit_code())
    }

    fn with_success_exit_code(self) -> Result<OkT, Problem> {
        self.into_problem()
            .map_err(|problem| problem.with_success_exit_code())
    }
}

//
// ExitError
//

/// Message error with an [ExitCodeAttachment].
///
/// Note that you can attach [ExitCodeAttachment] to any problem. This type is merely a convenience
/// for simple message errors.
#[derive(Clone, Debug)]
pub struct ExitError(pub Option<String>);

impl ExitError {
    /// Problem for [ExitError] with an [ExitCodeAttachment].
    pub fn problem<ToStringT, ExitCodeT>(message: ToStringT, exit_code: ExitCodeT) -> Problem
    where
        ToStringT: ToString,
        ExitCodeT: Into<ExitCode>,
    {
        Self(Some(message.to_string()))
            .into_problem()
            .with_exit_code(exit_code)
    }

    /// Problem for [ExitError] with a failure [ExitCodeAttachment].
    pub fn failure<ToStringT, ExitCodeT>(message: ToStringT) -> Problem
    where
        ToStringT: ToString,
    {
        Self(Some(message.to_string()))
            .into_problem()
            .with_failure_exit_code()
    }

    /// Problem for [ExitError] with a success [ExitCodeAttachment].
    pub fn success() -> Problem {
        Self(None).into_problem().with_success_exit_code()
    }
}

impl fmt::Display for ExitError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Some(message) => write!(formatter, "exit: {}", message),
            None => write!(formatter, "exit"),
        }
    }
}

impl Error for ExitError {}

impl PartialEq for ExitError {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for ExitError {}
