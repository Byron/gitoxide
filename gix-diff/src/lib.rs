//! Algorithms for diffing various git object types and for generating patches, highly optimized for performance.
//! ## Feature Flags
#![cfg_attr(
feature = "document-features",
cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![deny(missing_docs, rust_2018_idioms)]
#![forbid(unsafe_code)]

///
pub mod tree;

///
#[cfg(feature = "blob")]
pub mod blob;
