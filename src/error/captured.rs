use std::error::*;

//
// CapturedError
//

/// Captured [Error].
pub type CapturedError = Box<dyn Error + Send + Sync>;
