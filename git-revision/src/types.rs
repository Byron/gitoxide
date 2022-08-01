/// A revision specification without any bindings to a repository, useful for serialization or movement over thread boundaries.
///
/// Note that all [object ids][git_hash::ObjectId] should be a committish, but don't have to be.
/// Unless the field name contains `_exclusive`, the respective objects are included in the set.
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum Spec {
    /// The equivalent to [crate::spec::Kind::IncludeReachable], but with data.
    Include(git_hash::ObjectId),
    /// The equivalent to [crate::spec::Kind::ExcludeReachable], but with data.
    Exclude(git_hash::ObjectId),
    /// The equivalent to [crate::spec::Kind::RangeBetween], but with data.
    Range {
        /// The starting point of the range, which is included in the set.
        from: git_hash::ObjectId,
        /// The end point of the range, which is included in the set.
        to: git_hash::ObjectId,
    },

    /// The equivalent to [crate::spec::Kind::ReachableToMergeBase], but with data.
    Merge {
        /// Their side of the merge, which is included in the set.
        theirs: git_hash::ObjectId,
        /// Our side of the merge, which is included in the set.
        ours: git_hash::ObjectId,
    },

    /// The equivalent to [crate::spec::Kind::IncludeReachableFromParents], but with data.
    IncludeOnlyParents(
        /// Include only the parents of this object, but not the object itself.
        git_hash::ObjectId,
    ),
    /// The equivalent to [crate::spec::Kind::ExcludeReachableFromParents], but with data.
    ExcludeParents(
        /// Exclude the parents of this object, but not the object itself.
        git_hash::ObjectId,
    ),
}
