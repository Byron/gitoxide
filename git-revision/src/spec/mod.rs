/// How to interpret a revision specification, or `revspec`.
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum Kind {
    /// Include commits reachable from this revision, the default when parsing revision `a` for example, i.e. `a` and its ancestors.
    Include,
    /// Include commits reachable from this revision, i.e. `a` and its ancestors.
    Exclude,
    /// Every commit that is reachable from `b` but not from `a`.
    Range,
    /// Every commit reachable through either `a` or `b` but no commit that is reachable by both.
    MergeBase,
}

impl Default for Kind {
    fn default() -> Self {
        Kind::Include
    }
}

///
pub mod parse;
pub use parse::function::parse;
