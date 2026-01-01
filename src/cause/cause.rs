use super::{
    super::{attachment::*, error::*, problem::*},
    r#ref::*,
};

use std::{any::*, error::*};

//
// Cause
//

/// A link in a [Problem]'s causation chain.
pub struct Cause {
    /// Error.
    pub error: CapturedError,

    /// Attachments.
    pub attachments: Vec<CapturedAttachment>,
}

impl Cause {
    /// Constructor.
    pub fn new(error: CapturedError) -> Self {
        Self {
            error,
            attachments: Default::default(),
        }
    }

    /// To a [CauseRef].
    pub fn to_ref<'problem>(
        &'problem self,
        problem: &'problem Problem,
        depth: usize,
    ) -> CauseRef<'problem, CapturedError> {
        CauseRef::new(problem, depth, &self.error, self.attachments.as_ref())
    }

    /// Attach.
    pub fn attach<AttachmentT>(&mut self, attachment: AttachmentT)
    where
        AttachmentT: Any + Send + Sync,
    {
        self.attachments.push(Box::new(attachment));
    }

    /// Attach if [Some].
    pub fn maybe_attach<AttachmentT>(&mut self, attachment: Option<AttachmentT>)
    where
        AttachmentT: Any + Send + Sync,
    {
        if let Some(attachment) = attachment {
            self.attach(attachment)
        }
    }

    /// Attach a backtrace if we don't already have one.
    #[cfg(feature = "backtrace")]
    pub fn attach_backtrace(&mut self) {
        if self.attachment_of_type::<backtrace::Backtrace>().is_none() {
            self.attach(backtrace::Backtrace::new())
        }
    }
}

impl Attachments for Cause {
    fn attachments(&self) -> impl Iterator<Item = &CapturedAttachment> {
        self.attachments.iter()
    }
}

impl<ErrorT> From<ErrorT> for Cause
where
    ErrorT: 'static + Error + Send + Sync,
{
    fn from(error: ErrorT) -> Self {
        Self::new(error.into())
    }
}
