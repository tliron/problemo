use super::{as_error::*, captured::*, cause::*};

use {
    backtrace::*,
    std::{any::*, collections::*, error::*, fmt},
};

//
// Problem
//

/// Problem.
///
/// Note that this type does not itself implement [Error](Error) directly, but you can use
/// [into_error](Problem::into_error).
#[derive(Default)]
pub struct Problem {
    /// Causes in order of causation, from top to root.
    pub causes: VecDeque<Cause>,
}

impl Problem {
    /// Into error.
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

    /// Errors in order of causation, from top to root.
    ///
    /// Note that this will skip over [source](Error::source).
    pub fn errors(&self) -> impl Iterator<Item = &CapturedError> {
        self.causes.iter().map(|cause| &cause.error)
    }

    /// The first cause with an error of a type.
    ///
    /// Will recurse into [source](Error::source).
    pub fn get<'own, ErrorT>(&'own self) -> Option<CauseRef<'own, ErrorT>>
    where
        ErrorT: 'static + Error,
    {
        for (depth, cause) in self.causes.iter().enumerate() {
            if let Some(error) = downcast_error_or_source(cause.error.as_ref()) {
                return Some(CauseRef {
                    problem: self,
                    depth,
                    error,
                    attachments: &cause.attachments,
                });
            }
        }
        None
    }

    /// Whether we have an error in the causation chain.
    ///
    /// Will recurse into [source](Error::source).
    pub fn has<ErrorT>(&self, error: ErrorT) -> bool
    where
        ErrorT: 'static + Error + PartialEq,
    {
        self.get()
            .map(|cause| error == *cause.error)
            .unwrap_or(false)
    }

    /// Whether we have an error of a type in the causation chain.
    ///
    /// Will recurse into [source](Error::source).
    pub fn has_type<ErrorT>(&self) -> bool
    where
        ErrorT: 'static + Error,
    {
        for cause in &self.causes {
            if downcast_error_or_source::<ErrorT>(cause.error.as_ref()).is_some() {
                return true;
            }
        }
        false
    }

    /// Adds the error to the top of the causation chain.
    pub fn via<ErrorT>(mut self, error: ErrorT) -> Self
    where
        ErrorT: 'static + Error,
    {
        self.causes.push_front(error.into());
        self
    }

    /// Inserts our causation chain behind that of the given problem.
    pub fn behind(mut self, mut problem: Problem) -> Self {
        self.causes.append(&mut problem.causes);
        problem.causes = self.causes;
        problem
    }

    /// All attachments.
    pub fn attachments(&self) -> impl Iterator<Item = &CapturedAttachment> {
        self.causes
            .iter()
            .flat_map(|cause| cause.attachments.iter())
    }

    /// All attachments of a type.
    pub fn attachments_of<'own, AttachmentT>(&'own self) -> impl Iterator<Item = &'own AttachmentT>
    where
        AttachmentT: 'static,
    {
        self.attachments()
            .filter_map(|attachment| attachment.downcast_ref())
    }

    /// First attachment of a type.
    pub fn attachment_of<'own, AttachmentT>(&'own self) -> Option<&'own AttachmentT>
    where
        AttachmentT: 'static,
    {
        self.attachments_of().next()
    }

    /// Attach to the top cause.
    pub fn with<AttachmentT>(mut self, attachment: AttachmentT) -> Self
    where
        AttachmentT: Any + Send + Sync,
    {
        if let Some(cause) = self.top_mut() {
            cause.attachments.push(Box::new(attachment));
        }
        self
    }

    /// Attach to the top cause if [Some].
    pub fn maybe_with<AttachmentT>(self, attachment: Option<AttachmentT>) -> Self
    where
        AttachmentT: Any + Send + Sync,
    {
        match attachment {
            Some(attachment) => self.with(attachment),
            None => self,
        }
    }

    /// Attach backtrace.
    pub fn with_backtrace(self) -> Self {
        self.with(Backtrace::new())
    }
}

impl fmt::Debug for Problem {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let errors: Vec<_> = self
            .causes
            .iter()
            .map(|cause| format!("{:?}", cause.error))
            .collect();

        write!(formatter, "{}", errors.join("\n"))
    }
}

impl fmt::Display for Problem {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let errors: Vec<_> = self
            .causes
            .iter()
            .map(|cause| format!("{}", cause.error))
            .collect();

        write!(formatter, "{}", errors.join(": "))
    }
}

impl<ErrorT> From<ErrorT> for Problem
where
    ErrorT: 'static + Error,
{
    fn from(error: ErrorT) -> Self {
        Self {
            causes: [error.into()].into(),
        }
        .with_backtrace()
    }
}

// Utils

fn downcast_error_or_source<'own, ErrorT>(
    error: &'own (dyn 'static + Error),
) -> Option<&'own ErrorT>
where
    ErrorT: 'static + Error,
{
    // Recursive!
    error
        .downcast_ref()
        .or_else(|| error.source().and_then(downcast_error_or_source))
}
