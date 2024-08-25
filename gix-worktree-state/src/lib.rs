//! A crate to help setting the worktree to a particular state.
#![deny(missing_docs, rust_2018_idioms, unsafe_code)]

///
pub mod checkout;
pub use checkout::function::checkout;
