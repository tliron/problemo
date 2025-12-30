use super::{captured::*, problem::*, receiver::*};

use std::{any::*, collections::*, error::Error, fmt};

//
// Problems
//

/// Problems.
#[derive(Default)]
pub struct Problems {
    /// Problems.
    pub problems: Vec<Problem>,

    /// Critical error type IDs.
    pub critical_error_types: HashSet<TypeId>,
}

impl Problems {
    /// Constructor.
    pub fn with_capacity(capacity: usize) -> Self {
        Vec::with_capacity(capacity).into()
    }

    /// Marks a top error type as critical.
    pub fn handle_type_as_critical<ErrorT>(&mut self)
    where
        ErrorT: Any + Error,
    {
        self.critical_error_types.insert(TypeId::of::<ErrorT>());
    }

    /// True if the problem's top error is critical.
    pub fn is_critical(&self, problem: &Problem) -> bool {
        problem
            .top()
            .map(|cause| self.is_error_critical(&cause.error))
            .unwrap_or(false)
    }

    /// True if the error is critical.
    pub fn is_error_critical(&self, error: &CapturedError) -> bool {
        self.critical_error_types.contains(&error.type_id())
    }

    /// Add a problem.
    pub fn add<ProblemT>(&mut self, problem: ProblemT)
    where
        ProblemT: Into<Problem>,
    {
        self.problems.push(problem.into())
    }

    /// True if there are no problems.
    pub fn is_empty(&self) -> bool {
        self.problems.is_empty()
    }

    /// Fails with self if there are problems.
    pub fn check(self) -> Result<(), Self> {
        if self.is_empty() { Ok(()) } else { Err(self) }
    }
}

impl fmt::Debug for Problems {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let problems: Vec<_> = self
            .problems
            .iter()
            .map(|problem| format!("{:?}", problem))
            .collect();

        write!(formatter, "{}", problems.join("\n"))
    }
}

impl fmt::Display for Problems {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let problems: Vec<_> = self
            .problems
            .iter()
            .map(|problem| format!("{}", problem))
            .collect();

        write!(formatter, "{}", problems.join("\n"))
    }
}

impl Error for Problems {}

impl From<Vec<Problem>> for Problems {
    fn from(problems: Vec<Problem>) -> Self {
        Self {
            problems,
            critical_error_types: Default::default(),
        }
    }
}

impl FromIterator<Problem> for Problems {
    fn from_iter<IntoIteratorT>(iterator: IntoIteratorT) -> Self
    where
        IntoIteratorT: IntoIterator<Item = Problem>,
    {
        iterator.into_iter().collect::<Vec<_>>().into()
    }
}

impl<ErrorT> FromIterator<ErrorT> for Problems
where
    ErrorT: 'static + Error,
{
    fn from_iter<IntoIteratorT>(iterator: IntoIteratorT) -> Self
    where
        IntoIteratorT: IntoIterator<Item = ErrorT>,
    {
        Self::from_iter(iterator.into_iter().map(|error| Problem::from(error)))
    }
}

impl ProblemReceiver for Problems {
    fn give(&mut self, problem: Problem) -> Result<(), Problem> {
        // Fail fast if critical
        match self.is_critical(&problem) {
            // Fail fast if critical
            true => Err(problem),

            // Otherwise, swallow
            false => {
                self.problems.push(problem);
                Ok(())
            }
        }
    }
}
