use crate::Spec;

/// How to interpret a revision specification, or `revspec`.
#[derive(Default, Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Kind {
    /// Include commits reachable from this revision, the default when parsing revision `a` for example, i.e. `a` and its ancestors.
    /// Example: `a`.
    #[default]
    IncludeReachable,
    /// Exclude commits reachable from this revision, i.e. `a` and its ancestors. Example: `^a`.
    ExcludeReachable,
    /// Every commit that is reachable from `b` but not from `a`. Example: `a..b`.
    RangeBetween,
    /// Every commit reachable through either `a` or `b` but no commit that is reachable by both. Example: `a...b`.
    ReachableToMergeBase,
    /// Include every commit of all parents of `a`, but not `a` itself. Example: `a^@`.
    IncludeReachableFromParents,
    /// Exclude every commit of all parents of `a`, but not `a` itself. Example: `a^!`.
    ExcludeReachableFromParents,
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
            Spec::ExcludeParents { .. } => Kind::ExcludeReachableFromParents,
        }
    }
}

mod _impls {
    use std::fmt::{Display, Formatter};

    use crate::Spec;

    impl Display for Spec {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                Spec::Include(oid) => Display::fmt(oid, f),
                Spec::Exclude(oid) => write!(f, "^{oid}"),
                Spec::Range { from, to } => write!(f, "{from}..{to}"),
                Spec::Merge { theirs, ours } => write!(f, "{theirs}...{ours}"),
                Spec::IncludeOnlyParents(from_exclusive) => write!(f, "{from_exclusive}^@"),
                Spec::ExcludeParents(oid) => write!(f, "{oid}^!"),
            }
        }
    }
}

pub(crate) mod types {
    /// A revision specification without any bindings to a repository, useful for serialization or movement over thread boundaries.
    ///
    /// Note that all [object ids][gix_hash::ObjectId] should be a committish, but don't have to be.
    /// Unless the field name contains `_exclusive`, the respective objects are included in the set.
    #[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub enum Spec {
        /// Include commits reachable from this revision, i.e. `a` and its ancestors.
        ///
        /// The equivalent to [crate::spec::Kind::IncludeReachable], but with data.
        Include(gix_hash::ObjectId),
        /// Exclude commits reachable from this revision, i.e. `a` and its ancestors. Example: `^a`.
        ///
        /// The equivalent to [crate::spec::Kind::ExcludeReachable], but with data.
        Exclude(gix_hash::ObjectId),
        /// Every commit that is reachable from `from` to `to`, but not any ancestors of `from`. Example: `from..to`.
        ///
        /// The equivalent to [crate::spec::Kind::RangeBetween], but with data.
        Range {
            /// The starting point of the range, which is included in the set.
            from: gix_hash::ObjectId,
            /// The end point of the range, which is included in the set.
            to: gix_hash::ObjectId,
        },
        /// Every commit reachable through either `theirs` or `ours`, but no commit that is reachable by both. Example: `theirs...ours`.
        ///
        /// The equivalent to [crate::spec::Kind::ReachableToMergeBase], but with data.
        Merge {
            /// Their side of the merge, which is included in the set.
            theirs: gix_hash::ObjectId,
            /// Our side of the merge, which is included in the set.
            ours: gix_hash::ObjectId,
        },
        /// Include every commit of all parents of `a`, but not `a` itself. Example: `a^@`.
        ///
        /// The equivalent to [crate::spec::Kind::IncludeReachableFromParents], but with data.
        IncludeOnlyParents(
            /// Include only the parents of this object, but not the object itself.
            gix_hash::ObjectId,
        ),
        /// Exclude every commit of all parents of `a`, but not `a` itself. Example: `a^!`.
        ///
        /// The equivalent to [crate::spec::Kind::ExcludeReachableFromParents], but with data.
        ExcludeParents(
            /// Exclude the parents of this object, but not the object itself.
            gix_hash::ObjectId,
        ),
    }
}

///
pub mod parse;
pub use parse::function::parse;
