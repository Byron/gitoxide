//! A crate to help setting the worktree to a particular state.
#![deny(missing_docs, rust_2018_idioms, unsafe_code)]

///
#[allow(clippy::empty_docs)]
pub mod checkout;
pub use checkout::function::checkout;
