/// A revision specification without any bindings to a repository, useful for serialization or movement over thread boundaries.
///
/// Note that all [object ids][gix_hash::ObjectId] should be a committish, but don't have to be.
/// Unless the field name contains `_exclusive`, the respective objects are included in the set.
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
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
