//! Read, verify, and traverse git commit graphs.
//!
//! A [commit graph][Graph] acts as a cache is a set of [graph files][file::File] each storing
//! various commits in a way that accelerates lookups considerably compared to traversing the git history by usual means.
//!
//! A [`Graph`] is generated as each [`File`][file::File] only contains a subset of all commits in the git commit
//! history to avoid having to recreate the entire cache each time the cache should be updated, thus avoiding to traverse
//! the git history that is already present in the [`Graph`].
#![forbid(unsafe_code)]
#![deny(rust_2018_idioms, missing_docs)]

pub mod file;
pub mod graph;

pub use graph::Graph;

/// The number of generations that are considered 'infinite' commit history.
pub const GENERATION_NUMBER_INFINITY: u32 = 0xffff_ffff;
/// The biggest possible amount of commits in the history of a repository.
pub const GENERATION_NUMBER_MAX: u32 = 0x3fff_ffff;

/// The maximum number of commits that can be stored in a commit graph.
pub const MAX_COMMITS: u32 = (1 << 30) + (1 << 29) + (1 << 28) - 1;
