//! Various functionality related to git references
#![forbid(unsafe_code)]
#![deny(rust_2018_idioms)]
#![allow(missing_docs)]

pub mod reference;
pub use reference::name as refname;

pub mod tag;
pub use tag::name as tagname;
