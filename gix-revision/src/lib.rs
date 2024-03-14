//! Interact with git revisions by parsing them from rev-specs and describing them in terms of reference names.
//!
//! ## Feature Flags
#![cfg_attr(
    all(doc, feature = "document-features"),
    doc = ::document_features::document_features!()
)]
#![cfg_attr(all(doc, feature = "document-features"), feature(doc_cfg, doc_auto_cfg))]
#![deny(missing_docs, rust_2018_idioms, unsafe_code)]

///
#[allow(clippy::empty_docs)]
#[cfg(feature = "describe")]
pub mod describe;
#[cfg(feature = "describe")]
pub use describe::function::describe;

///
#[allow(clippy::empty_docs)]
pub mod spec;
pub use gix_revwalk::{graph, Graph, PriorityQueue};
pub use spec::types::Spec;
