use super::{attachment::*, cause::*, error::*};

use std::{any::*, collections::*, error::*, fmt, io};

//
// Problem
//

/// Problem.
///
/// Note that this type does not itself implement [Error](Error) directly, but you can use
/// [into_error](Problem::into_error).
#[derive(Default)]
pub struct Problem {
    /// Causes in order of causation from top to root.
    pub causes: VecDeque<Cause>,
}

impl Problem {
    /// Add support for [Error].
    ///
    /// Take care to avoid adding it into a [Problem]'s causation chain.
    pub fn into_error(self) -> ProblemAsError {
        self.into()
    }

    /// The top of the causation chain.
    pub fn top(&self) -> Option<&Cause> {
        self.causes.front()
    }

    /// The top of the causation chain.
    pub fn top_mut(&mut self) -> Option<&mut Cause> {
        self.causes.front_mut()
    }

    /// The root of the causation chain.
    pub fn root(&self) -> Option<&Cause> {
        self.causes.back()
    }

    /// The root of the causation chain.
    pub fn root_mut(&mut self) -> Option<&mut Cause> {
        self.causes.back_mut()
    }

    /// Inserts our causation chain under that of the given problem.
    pub fn under(mut self, mut problem: Problem) -> Self {
        self.causes.append(&mut problem.causes);
        problem.causes = self.causes;
        problem
    }

    /// Appends our causation chain above that of the given problem.
    pub fn above(mut self, mut problem: Problem) -> Self {
        problem.causes.append(&mut self.causes);
        problem
    }

    /// Adds the error to the top of the causation chain.
    pub fn via<ErrorT>(mut self, error: ErrorT) -> Self
    where
        ErrorT: 'static + Error + Send + Sync,
    {
        self.causes.push_front(error.into());
        self
    }

    /// Attach to the top cause.
    pub fn with<AttachmentT>(mut self, attachment: AttachmentT) -> Self
    where
        AttachmentT: Any + Send + Sync,
    {
        if let Some(cause) = self.top_mut() {
            cause.attach(attachment);
        }
        self
    }

    /// Attach to the top cause if [Some].
    pub fn maybe_with<AttachmentT>(mut self, attachment: Option<AttachmentT>) -> Self
    where
        AttachmentT: Any + Send + Sync,
    {
        if let Some(attachment) = attachment
            && let Some(cause) = self.top_mut()
        {
            cause.attach(attachment);
        }
        self
    }

    /// Attach a backtrace if we don't already have one.
    #[cfg(feature = "backtrace")]
    pub fn with_backtrace(mut self) -> Self {
        if self.attachment_of_type::<backtrace::Backtrace>().is_none()
            && let Some(cause) = self.top_mut()
        {
            cause.attach_backtrace();
        }
        self
    }
}

impl CausationChain<'_> for Problem {
    fn owning_problem(&self) -> &Problem {
        self
    }
}

impl Attachments for Problem {
    fn attachments(&self) -> impl Iterator<Item = &CapturedAttachment> {
        self.into_iter().flat_map(|cause| cause.attachments.iter())
    }
}

impl fmt::Debug for Problem {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut iterator = self.into_iter().peekable();
        while let Some(cause) = iterator.next() {
            write!(formatter, "{:?}", cause.error)?;
            if iterator.peek().is_some() {
                writeln!(formatter)?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for Problem {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut iterator = self.into_iter().peekable();
        while let Some(cause) = iterator.next() {
            write!(formatter, "{}", cause.error)?;
            if iterator.peek().is_some() {
                write!(formatter, ": ")?;
            }
        }
        Ok(())
    }
}

#[cfg(feature = "backtrace")]
impl<ErrorT> From<ErrorT> for Problem
where
    ErrorT: 'static + Error + Send + Sync,
{
    fn from(error: ErrorT) -> Self {
        Self {
            causes: [error.into()].into(),
        }
        .with_backtrace()
    }
}

#[cfg(not(feature = "backtrace"))]
impl<ErrorT> From<ErrorT> for Problem
where
    ErrorT: 'static + Error + Send + Sync,
{
    fn from(error: ErrorT) -> Self {
        Self {
            causes: [error.into()].into(),
        }
    }
}

impl Into<io::Error> for Problem {
    fn into(self) -> io::Error {
        io::Error::new(io::ErrorKind::Other, self.into_error())
    }
}

impl IntoIterator for Problem {
    type Item = Cause;
    type IntoIter = vec_deque::IntoIter<Cause>;

    fn into_iter(self) -> Self::IntoIter {
        self.causes.into_iter()
    }
}

impl<'own> IntoIterator for &'own Problem {
    type Item = &'own Cause;
    type IntoIter = vec_deque::Iter<'own, Cause>;

    fn into_iter(self) -> Self::IntoIter {
        self.causes.iter()
    }
}
