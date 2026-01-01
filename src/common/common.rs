use super::super::error::*;

message_error!(MessageError);

// General

tag_error!(LowLevelError, "low-level");
message_error!(UnsupportedError, "unsupported");
message_error!(IncompatibleError, "incompatible");
message_error!(UnreachableError, "unreachable");
message_error!(NotFoundError, "not found");

// Data validation

message_error!(MissingError, "missing");
message_error!(InvalidError, "invalid");
message_error!(MalformedError, "malformed");
message_error!(NoneError, "none");
message_error!(NoMoreItemsError, "no more items");

// Data conversion

message_error!(SerializeError, "serialize");
message_error!(DeserializeError, "deserialize");
message_error!(OverflowError, "overflow");

// Threading

message_error!(ThreadError, "thread");
