//! Interact with git credentials in various ways and launch helper programs.
//!
//! ## Feature Flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![forbid(unsafe_code)]
#![deny(missing_docs, rust_2018_idioms)]

///
pub mod helper;
pub use helper::action as helper;
