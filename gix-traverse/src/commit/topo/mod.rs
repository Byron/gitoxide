//! Topological commit traversal, similar to `git log --topo-order`.

use gix_hash::ObjectId;
use gix_revwalk::{graph::IdMap, PriorityQueue};

use bitflags::bitflags;

use super::Parents;

/// A commit walker that walks in topographical order, like `git rev-list
/// --topo-order` or `--date-order` depending on the chosen [`Sorting`].
pub struct Walk<Find, Predicate> {
    commit_graph: Option<gix_commitgraph::Graph>,
    find: Find,
    predicate: Predicate,
    indegrees: IdMap<i32>,
    states: IdMap<WalkFlags>,
    explore_queue: PriorityQueue<iter::GenAndCommitTime, ObjectId>,
    indegree_queue: PriorityQueue<iter::GenAndCommitTime, ObjectId>,
    topo_queue: iter::Queue,
    parents: Parents,
    min_gen: u32,
    buf: Vec<u8>,
}

/// The errors that can occur during creation and iteration.
#[derive(thiserror::Error, Debug)]
#[allow(missing_docs)]
pub enum Error {
    #[error("Indegree information is missing")]
    MissingIndegreeUnexpected,
    #[error("Internal state (bitflags) not found")]
    MissingStateUnexpected,
    #[error(transparent)]
    CommitGraphFile(#[from] gix_commitgraph::file::commit::Error),
    #[error(transparent)]
    ObjectDecode(#[from] gix_object::decode::Error),
    #[error(transparent)]
    Find(#[from] gix_object::find::existing_iter::Error),
}

bitflags! {
    /// Set of flags to describe the state of a particular commit while iterating.
    // NOTE: The names correspond to the names of the flags in revision.h
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct WalkFlags: u32 {
        /// Commit has been seen
        const Seen = 0b000001;
        /// Commit has been processed by the Explore walk
        const Explored = 0b000010;
        /// Commit has been processed by the Indegree walk
        const InDegree = 0b000100;
        /// Commit is deemed uninteresting for whatever reason
        const Uninteresting = 0b001000;
        /// Commit marks the end of a walk, like `foo` in `git rev-list foo..bar`
        const Bottom = 0b010000;
        /// Parents have been processed
        const Added = 0b100000;
    }
}

/// Sorting to use for the topological walk
#[derive(Clone, Copy, Debug, Default)]
pub enum Sorting {
    /// Show no parents before all of its children are shown, but otherwise show
    /// commits in the commit timestamp order.
    #[default]
    DateOrder,

    /// Show no parents before all of its children are shown, and avoid
    /// showing commits on multiple lines of history intermixed.
    TopoOrder,
}

mod init;
pub use init::Builder;

mod iter;
