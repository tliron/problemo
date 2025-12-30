// https://stackoverflow.com/a/61417700
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]

/*!
This library aims to improve the experience of working with Rust's [Error](std::error::Error) trait
by allowing for deep causation chains and arbitrary attachments with the goal of making it easy and
rewarding to return richly-typed errors for callers to inspect and handle.

For more information and usage examples see the
[home page](https://github.com/tliron/problemo).
*/

mod as_error;
mod captured;
mod cause;
mod errors;
mod into;
mod problem;
mod problems;
mod receiver;
mod result;

/// Common error and attachment types.
pub mod common;

#[allow(unused_imports)]
pub use {
    as_error::*, backtrace, captured::*, cause::*, errors::*, into::*, problem::*, problems::*,
    receiver::*, result::*,
};
