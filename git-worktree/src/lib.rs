//! ## Feature Flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![deny(missing_docs, rust_2018_idioms, unsafe_code)]

/// file system related utilities
pub mod fs;

///
pub mod index;

pub(crate) mod os;
