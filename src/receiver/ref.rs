use super::{super::problem::*, receiver::*};

use std::{cell::*, sync::*};

//
// ErrorReceiverRef
//

/// Common reference type for [ProblemReceiver].
pub type ProblemReceiverRef<'own> = Arc<RefCell<&'own mut dyn ProblemReceiver>>;

impl<'own> ProblemReceiver for ProblemReceiverRef<'own> {
    fn give(&mut self, problem: Problem) -> Result<(), Problem> {
        self.borrow_mut().give(problem)
    }
}

//
// ProblemReceiverAsRef
//

/// As problem receiver reference.
pub trait ProblemReceiverAsRef<'own, ProblemReceiverT> {
    /// As error receiver reference.
    fn as_ref(&'own mut self) -> ProblemReceiverRef<'own>;
}

impl<'own, ProblemReceiverT> ProblemReceiverAsRef<'own, ProblemReceiverT> for ProblemReceiverT
where
    ProblemReceiverT: ProblemReceiver,
{
    fn as_ref(&'own mut self) -> ProblemReceiverRef<'own> {
        ProblemReceiverRef::new(RefCell::new(self))
    }
}
