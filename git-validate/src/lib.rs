//! Validation for various kinds of git related items.
#![forbid(unsafe_code)]
#![deny(missing_docs, rust_2018_idioms)]

///
pub mod reference;
pub use reference::name as refname;

///
pub mod tag;
pub use tag::name as tagname;
