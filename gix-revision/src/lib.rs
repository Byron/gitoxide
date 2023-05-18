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
use crate::graph::FindFn;
pub use types::Spec;

/// A graph of commits which additionally allows to associate data with commits.
///
/// It starts empty, but each access may fill it with commit information.
/// Note that the traversal can be accelerated if a [commit-graph][gix_commitgraph::Graph] is also made available.
pub struct Graph<'find, T> {
    /// A way to resolve a commit from the object database.
    find: Box<FindFn<'find>>,
    /// A way to speedup commit access, essentially a multi-file commit database.
    cache: Option<gix_commitgraph::Graph>,
    /// The set of cached commits that we have seen once, along with data associated with them.
    set: gix_hashtable::HashMap<gix_hash::ObjectId, T>,
    /// A buffer for writing commit data into.
    buf: Vec<u8>,
    /// Another buffer we typically use to store parents.
    parent_buf: Vec<u8>,
}
///
pub mod graph;

/// A utility type implementing a queue which can be used to automatically sort data by its time in ascending order.
///
/// Note that the performance of this queue is very relevant to overall algorithm performance of many graph-walking algorithms,
/// and as it stands our implementation is about 6% slower in practice, probably also depending on the size of the stored data.
#[derive(Default)]
pub struct PriorityQueue<K: Ord, T>(std::collections::BinaryHeap<queue::Item<K, T>>);
mod queue;
