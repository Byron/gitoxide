//! Interact with git revisions by parsing them from rev-specs and describing them in terms of reference names.
//!
//! ## Feature Flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![deny(missing_docs, rust_2018_idioms, unsafe_code)]

///
pub mod describe;
pub use describe::function::describe;

///
pub mod spec;

mod types;
pub use types::Spec;
