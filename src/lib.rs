// https://stackoverflow.com/a/61417700
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]

/*!
This library aims to improve the experience of working with Rust's [Error](std::error::Error) trait
by allowing for deep causation chains, arbitrary attachments, and error accumulation with the goal
of making it easy and rewarding to return richly-typed errors for callers to inspect and handle.

For more information and usage examples see the
[home page](https://github.com/tliron/problemo).
*/

mod attachment;
mod cause;
mod compatibility;
mod error;
mod into;
mod problem;
mod problems;
mod receiver;
mod result;

/// Common error and attachment types.
pub mod common;

#[allow(unused_imports)]
pub use {
    attachment::*, cause::*, compatibility::*, error::*, into::*, problem::*, problems::*,
    receiver::*, result::*,
};

#[cfg(feature = "backtrace")]
pub use backtrace;
