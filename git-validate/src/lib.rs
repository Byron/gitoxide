//! Validation for various kinds of git related items.
#![deny(missing_docs, rust_2018_idioms)]
#![forbid(unsafe_code)]

///
pub mod reference;
pub use reference::name as refname;

///
pub mod tag;
pub use tag::name as tagname;
