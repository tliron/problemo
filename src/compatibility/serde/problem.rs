use super::super::super::{common::*, into::*, problem::*};

use {
    serde::*,
    std::{error::Error, fmt},
};

//
// SerdeProblem
//

/// [Problem] with support for [ser::Error] and [de::Error].
///
/// Note that unlike [Problem] it also supports [Error], so take care to avoid adding it to a
/// [Problem]'s causation chain.
pub struct SerdeProblem {
    /// Problem.
    pub problem: Problem,
}

impl fmt::Debug for SerdeProblem {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.problem, formatter)
    }
}

impl fmt::Display for SerdeProblem {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.problem, formatter)
    }
}

impl Error for SerdeProblem {}

impl ser::Error for SerdeProblem {
    fn custom<DisplayT>(custom: DisplayT) -> Self
    where
        DisplayT: fmt::Display,
    {
        SerializeError::new(format!("serde: {}", custom))
            .into_problem()
            .into()
    }
}

impl de::Error for SerdeProblem {
    fn custom<DisplayT>(custom: DisplayT) -> Self
    where
        DisplayT: fmt::Display,
    {
        DeserializeError::new(format!("serde: {}", custom))
            .into_problem()
            .into()
    }
}

impl From<Problem> for SerdeProblem {
    fn from(problem: Problem) -> Self {
        Self { problem }
    }
}
