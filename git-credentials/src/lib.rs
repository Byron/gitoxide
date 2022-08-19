//! Interact with git credentials in various ways and launch helper programs.
//!
//! ## Feature Flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![deny(missing_docs, rust_2018_idioms)]
#![forbid(unsafe_code)]

///
pub mod helper;
pub use helper::action as helper;
