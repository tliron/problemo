use std::{any::*, error::*};

//
// CapturedError
//

/// Captured [Error].
pub type CapturedError = Box<dyn Error>;

//
// CapturedAttachment
//

/// Captured attachment.
pub type CapturedAttachment = Box<dyn Any>;
