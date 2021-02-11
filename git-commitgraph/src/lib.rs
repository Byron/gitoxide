//! Read, verify, and traverse git commit graphs.
//!
//! A [commit graph][Graph] is an index of commits in the git commit history.
//! The [Graph] stores commit data in a way that accelerates lookups considerably compared to
//! traversing the git history by usual means.
//!
//! As generating the full commit graph from scratch can take some time, git may write new commits
//! to separate [files][file::File] instead of overwriting the original file.
//! Eventually, git will merge these files together as the number of files grows.
#![forbid(unsafe_code)]
#![deny(rust_2018_idioms, missing_docs)]

pub mod file;
pub mod graph;

pub use graph::Graph;

/// The number of generations that are considered 'infinite' commit history.
pub const GENERATION_NUMBER_INFINITY: u32 = 0xffff_ffff;
/// The largest valid generation number.
///
/// If a commit's real generation number is larger than this, the commit graph will cap the value to
/// this number.
/// The largest distinct generation number is `GENERATION_NUMBER_MAX - 1`.
pub const GENERATION_NUMBER_MAX: u32 = 0x3fff_ffff;

/// The maximum number of commits that can be stored in a commit graph.
pub const MAX_COMMITS: u32 = (1 << 30) + (1 << 29) + (1 << 28) - 1;
