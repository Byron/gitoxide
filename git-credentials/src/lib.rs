#![forbid(unsafe_code)]
#![deny(missing_docs, rust_2018_idioms)]
//! Interact with git credentials in various ways and launch helper programs.

///
pub mod helper;
pub use helper::action as helper;
