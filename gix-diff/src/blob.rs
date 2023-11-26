//! For using text diffs, please have a look at the [`imara-diff` documentation](https://docs.rs/imara-diff),
//! maintained by [Pascal Kuthe](https://github.com/pascalkuthe).
//!
//!
/// Information about the diff performed to detect similarity.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub struct DiffLineStats {
    /// The amount of lines to remove from the source to get to the destination.
    pub removals: u32,
    /// The amount of lines to add to the source to get to the destination.
    pub insertions: u32,
    /// The amount of lines of the previous state, in the source.
    pub before: u32,
    /// The amount of lines of the new state, in the destination.
    pub after: u32,
}

pub use imara_diff::*;
