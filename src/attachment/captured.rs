use std::any::*;

//
// CapturedAttachment
//

/// Captured attachment.
pub type CapturedAttachment = Box<dyn Any + Send + Sync>;
