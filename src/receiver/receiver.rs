use super::super::problem::*;

//
// ProblemReceiver
//

/// A problem receiver.
///
/// Example of usage:
///
/// ```
/// fn divide<ProblemReceiverT>(a: f64, b: f64, problems: &mut ProblemReceiverT) -> Result<Option<f64>, Problem>
/// where
///     ProblemReceiverT: ProblemReceiver,
/// {
///     Ok(if b == 0.0 {
///         problems.give("division by zero".into_problem())?;
///         None
///     } else {
///         Some(a / b)
///     })
/// }
/// ```
pub trait ProblemReceiver {
    /// Gives a problem to the receiver.
    ///
    /// Implementations may swallow the problem (e.g. to accumulate it) or return it (fail-fast).
    fn give(&mut self, problem: Problem) -> Result<(), Problem>;
}
