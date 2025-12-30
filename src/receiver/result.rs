use super::{
    super::{into::*, problem::*},
    receiver::*,
};

//
// ReportReceiverResult
//

/// Problem receiver result.
pub trait ReportReceiverResult<OkT, ProblemReceiverT>
where
    ProblemReceiverT: ProblemReceiver,
{
    /// Like [Result::ok] but gives [Err] to a [ProblemReceiver].
    fn give_ok(self, receiver: &mut ProblemReceiverT) -> Result<Option<OkT>, Problem>;

    /// Like [Result::unwrap_or] but gives [Err] to a [ProblemReceiver].
    fn give_unwrap_or(self, receiver: &mut ProblemReceiverT, default: OkT) -> Result<OkT, Problem>;

    /// Like [Result::unwrap_or_default] but gives [Err] to a [ProblemReceiver].
    fn give_unwrap_or_default(self, receiver: &mut ProblemReceiverT) -> Result<OkT, Problem>
    where
        OkT: Default;
}

impl<ResultT, OkT, ProblemReceiverT> ReportReceiverResult<OkT, ProblemReceiverT> for ResultT
where
    ProblemReceiverT: ProblemReceiver,
    ResultT: IntoProblemResult<OkT>,
{
    fn give_ok(self, receiver: &mut ProblemReceiverT) -> Result<Option<OkT>, Problem> {
        match self.into_problem() {
            Ok(ok) => Ok(Some(ok)),
            Err(problem) => {
                receiver.give(problem)?;
                Ok(None)
            }
        }
    }

    fn give_unwrap_or(self, receiver: &mut ProblemReceiverT, default: OkT) -> Result<OkT, Problem> {
        match self.into_problem() {
            Ok(ok) => Ok(ok),
            Err(problem) => {
                receiver.give(problem)?;
                Ok(default)
            }
        }
    }

    fn give_unwrap_or_default(self, receiver: &mut ProblemReceiverT) -> Result<OkT, Problem>
    where
        OkT: Default,
    {
        match self.into_problem() {
            Ok(ok) => Ok(ok),
            Err(problem) => {
                receiver.give(problem)?;
                Ok(OkT::default())
            }
        }
    }
}
