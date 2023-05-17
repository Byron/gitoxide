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

/// A utility type implementing a queue which can be used to automatically sort data by its time in ascending order.
///
/// Note that the performance of this queue is very relevant to overall algorithm performance of many graph-walking algorithms,
/// and as it stands our implementation is about 6% slower in practice, probably also depending on the size of the stored data.
#[derive(Default)]
pub struct PriorityQueue<K: Ord, T>(std::collections::BinaryHeap<queue::Item<K, T>>);
mod queue;
