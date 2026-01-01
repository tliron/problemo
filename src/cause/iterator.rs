use super::{
    super::{error::*, problem::*},
    r#ref::*,
};

//
// CauseRefIterator
//

/// [CauseRef] iterator.
///
/// Note that this will skip over [source](std::error::Error::source).
pub struct CauseRefIterator<'problem> {
    /// Problem.
    pub problem: &'problem Problem,

    /// Current depth.
    pub depth: usize,
}

impl<'problem> CauseRefIterator<'problem> {
    /// Constructor.
    pub fn new(problem: &'problem Problem, depth: usize) -> Self {
        CauseRefIterator { problem, depth }
    }
}

impl<'problem> Iterator for CauseRefIterator<'problem> {
    type Item = CauseRef<'problem, CapturedError>;

    fn next(&mut self) -> Option<Self::Item> {
        let depth = self.depth;
        self.depth += 1;
        self.problem
            .causes
            .get(depth)
            .map(|cause| cause.to_ref(self.problem, depth))
    }
}
