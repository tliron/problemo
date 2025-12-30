use super::{captured::*, problem::*};

use std::error::*;

//
// Cause
//

/// A cause is a link in a [Problem]'s causation chain.
pub struct Cause {
    /// Error.
    pub error: CapturedError,

    /// Attachments.
    pub attachments: Vec<CapturedAttachment>,
}

impl<ErrorT> From<ErrorT> for Cause
where
    ErrorT: 'static + Error,
{
    fn from(error: ErrorT) -> Self {
        Self {
            error: Box::new(error),
            attachments: Default::default(),
        }
    }
}

//
// CauseRef
//

/// Reference to a [Cause].
pub struct CauseRef<'own, ErrorT> {
    /// Containing problem.
    pub problem: &'own Problem,

    /// Depth in causation chain.
    pub depth: usize,

    /// Error.
    ///
    /// This error could be either on the causation chain or nested in [source](Error::source).
    pub error: &'own ErrorT,

    /// Attachments.
    pub attachments: &'own Vec<CapturedAttachment>,
}

impl<'own, ErrorT> CauseRef<'own, ErrorT> {
    /// Next cause in the causation chain.
    ///
    /// It will be [None] if we are the root cause.
    ///
    /// Note that this will skip over [source](Error::source).
    pub fn next(&self) -> Option<CauseRef<'own, CapturedError>> {
        let depth = self.depth + 1;
        self.problem.causes.get(depth).map(|cause| CauseRef {
            problem: self.problem,
            depth,
            error: &cause.error,
            attachments: cause.attachments.as_ref(),
        })
    }

    /// Whether we are the top cause.
    pub fn is_top(&self) -> bool {
        self.depth == 0
    }

    /// Whether we are the root cause.
    pub fn is_root(&self) -> bool {
        self.depth == (self.problem.causes.len() - 1)
    }

    /// All attachments of a type.
    pub fn attachments_of<AttachmentT>(&self) -> impl Iterator<Item = &'own AttachmentT>
    where
        AttachmentT: 'static,
    {
        self.attachments
            .iter()
            .filter_map(|attachment| attachment.downcast_ref())
    }

    /// First attachment of a type.
    pub fn attachment_of<AttachmentT>(&self) -> Option<&'own AttachmentT>
    where
        AttachmentT: 'static,
    {
        self.attachments_of().next()
    }
}
