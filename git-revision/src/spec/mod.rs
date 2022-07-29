use crate::Spec;

/// How to interpret a revision specification, or `revspec`.
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum Kind {
    /// Include commits reachable from this revision, the default when parsing revision `a` for example, i.e. `a` and its ancestors.
    /// Created by `a`.
    IncludeReachable,
    /// Exclude commits reachable from this revision, i.e. `a` and its ancestors. Created by `^a`.
    ExcludeReachable,
    /// Every commit that is reachable from `b` but not from `a`. Created by `a..b`.
    RangeBetween,
    /// Every commit reachable through either `a` or `b` but no commit that is reachable by both. Created by `a...b`.
    ReachableToMergeBase,
    /// Include every commit of all parents of `a`, but not `a` itself. Created by `a^@`.
    IncludeReachableFromParents,
    /// Exclude every commit of all parents of `a`, but not `a` itself. Created by `a^!`.
    ExcludeReachableFromParents,
}

impl Default for Kind {
    fn default() -> Self {
        Kind::IncludeReachable
    }
}

impl Spec {
    /// Return the kind of this specification.
    pub fn kind(&self) -> Kind {
        match self {
            Spec::Include(_) => Kind::IncludeReachable,
            Spec::Exclude(_) => Kind::ExcludeReachable,
            Spec::Range { .. } => Kind::RangeBetween,
            Spec::Merge { .. } => Kind::ReachableToMergeBase,
            Spec::IncludeOnlyParents { .. } => Kind::IncludeReachableFromParents,
            Spec::ExcludeFromParents { .. } => Kind::ExcludeReachableFromParents,
        }
    }
}

///
pub mod parse;
pub use parse::function::parse;
