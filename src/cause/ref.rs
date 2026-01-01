use super::{
    super::{attachment::*, error::*, problem::*},
    iterator::*,
};

//
// CauseRef
//

/// Reference to a [Cause].
pub struct CauseRef<'problem, ErrorT> {
    /// Containing problem.
    pub problem: &'problem Problem,

    /// Depth in causation chain.
    pub depth: usize,

    /// Error.
    ///
    /// This error could be either on the causation chain or nested in [source](Error::source).
    pub error: &'problem ErrorT,

    /// Attachments.
    pub attachments: &'problem Vec<CapturedAttachment>,
}

impl<'problem, ErrorT> CauseRef<'problem, ErrorT> {
    /// Constructor.
    pub fn new(
        problem: &'problem Problem,
        depth: usize,
        error: &'problem ErrorT,
        attachments: &'problem Vec<CapturedAttachment>,
    ) -> Self {
        CauseRef {
            problem,
            depth,
            error,
            attachments,
        }
    }

    /// Iterate the causation chain starting from *under* this cause.
    ///
    /// Note that this will skip over [source](std::error::Error::source).
    pub fn iter_under(&self) -> CauseRefIterator<'problem> {
        CauseRefIterator::new(self.problem, self.depth + 1)
    }

    /// The cause under this one in the causation chain.
    ///
    /// It will be [None] if we are the root cause.
    ///
    /// Note that this will skip over [source](std::error::Error::source).
    pub fn under(&self) -> Option<CauseRef<'problem, CapturedError>> {
        self.iter_under().next()
    }

    /// Whether we are the top cause.
    pub fn is_top(&self) -> bool {
        self.depth == 0
    }

    /// Whether we are the root cause.
    pub fn is_root(&self) -> bool {
        self.depth == (self.problem.causes.len() - 1)
    }
}

impl<'problem, ErrorT> Attachments for CauseRef<'problem, ErrorT> {
    fn attachments(&self) -> impl Iterator<Item = &CapturedAttachment> {
        self.attachments.iter()
    }
}
