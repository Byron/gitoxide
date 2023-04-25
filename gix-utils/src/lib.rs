//! A crate with utilities that don't need feature toggles.
//!
//! If they would need feature toggles, they should be in `gix-features` instead.
#![deny(rust_2018_idioms, missing_docs)]
#![forbid(unsafe_code)]

///
pub mod backoff;
