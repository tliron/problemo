use super::super::errors::*;

tag_error!(LowLevelError, "low-level");
tag_error!(OverflowError, "overflow");

message_error!(MessageError);
message_error!(ConcurrencyError, "concurrency");
