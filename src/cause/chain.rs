use super::{
    super::{error::*, problem::*},
    cause::*,
    r#ref::*,
};

use std::error::*;

//
// CausationChain
//

/// Causation chain.
pub trait CausationChain<'own>
where
    Self: 'static,
    &'own Self: IntoIterator<Item = &'own Cause>,
{
    /// The problem that owns the causation chain.
    fn owning_problem(&self) -> &Problem;

    /// Errors in order of causation.
    ///
    /// Note that this will skip over [source](Error::source).
    fn errors(&'own self) -> impl Iterator<Item = &'own CapturedError> {
        self.into_iter().map(|cause| &cause.error)
    }

    /// Whether we have an error of a type in the causation chain.
    ///
    /// Will recurse into [source](Error::source).
    fn has_type<ErrorT>(&'own self) -> bool
    where
        ErrorT: 'static + Error,
    {
        self.into_iter()
            .filter(|cause| downcast_error_or_source::<ErrorT>(cause.error.as_ref()).is_some())
            .next()
            .is_some()
    }

    /// Causes with an error of a type.
    ///
    /// Will recurse into [source](Error::source).
    fn causes_of_type<ErrorT>(&'own self) -> impl Iterator<Item = CauseRef<'own, ErrorT>>
    where
        ErrorT: 'static + Error,
    {
        self.into_iter().enumerate().filter_map(|(depth, cause)| {
            downcast_error_or_source(cause.error.as_ref())
                .map(|error| CauseRef::new(self.owning_problem(), depth, error, &cause.attachments))
        })
    }

    /// The first cause with an error of a type.
    ///
    /// Will recurse into [source](Error::source).
    fn cause_of_type<ErrorT>(&'own self) -> Option<CauseRef<'own, ErrorT>>
    where
        ErrorT: 'static + Error,
    {
        self.causes_of_type().next()
    }

    /// Whether we have the error in the causation chain.
    ///
    /// Will recurse into [source](Error::source).
    fn has<ErrorT>(&'own self, error: &ErrorT) -> bool
    where
        ErrorT: 'static + Error + PartialEq,
    {
        self.into_iter()
            .filter_map(|cause| downcast_error_or_source(cause.error.as_ref()))
            .filter(|cause_error| *error == **cause_error)
            .next()
            .is_some()
    }

    /// Causes for the error.
    ///
    /// Will recurse into [source](Error::source).
    fn causes_for<ErrorT>(
        &'own self,
        error: &ErrorT,
    ) -> impl Iterator<Item = CauseRef<'own, ErrorT>>
    where
        ErrorT: 'static + Error + PartialEq,
    {
        self.into_iter().enumerate().filter_map(|(depth, cause)| {
            downcast_error_or_source(cause.error.as_ref())
                .filter(|cause_error| *error == **cause_error)
                .map(|error| CauseRef::new(self.owning_problem(), depth, error, &cause.attachments))
        })
    }

    /// The first cause for the error.
    ///
    /// Will recurse into [source](Error::source).
    fn cause_for<ErrorT>(&'own self, error: &ErrorT) -> Option<CauseRef<'own, ErrorT>>
    where
        ErrorT: 'static + Error + PartialEq,
    {
        self.causes_for(error).next()
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
