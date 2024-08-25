bitflags::bitflags! {
    /// The flags used in the graph for finding [merge bases](crate::merge_base()).
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Flags: u8 {
        /// The commit belongs to the graph reachable by the first commit
        const COMMIT1 = 1 << 0;
        /// The commit belongs to the graph reachable by all other commits.
        const COMMIT2 = 1 << 1;

        /// Marks the commit as done, it's reachable by both COMMIT1 and COMMIT2.
        const STALE = 1 << 2;
        /// The commit was already put ontto the results list.
        const RESULT = 1 << 3;
    }
}

/// The error returned by the [`merge_base()`][function::describe()] function.
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("A commit could not be found")]
    FindExistingCommit(#[from] gix_object::find::existing_iter::Error),
    #[error("A commit could not be decoded during traversal")]
    Decode(#[from] gix_object::decode::Error),
}

pub(crate) mod function {
    use super::Error;
    use crate::{merge_base::Flags, Graph, PriorityQueue};
    use gix_hash::ObjectId;
    use gix_revwalk::graph::LazyCommit;
    use std::cmp::Ordering;

    /// Given a commit at `first` id, traverse the commit `graph` and return all possible merge-base between it and `others`,
    /// sorted from best to worst. Returns `None` if there is no merge-base as `first` and `others` don't share history.
    /// If `others` is empty, `Some(first)` is returned.
    ///
    /// Note that this function doesn't do any work if `first` is contained in `others`, which is when `first` will be returned
    /// as only merge-base right away. This is even the case if some commits of `others` are disjoint.
    pub fn merge_base<'name>(
        first: ObjectId,
        others: &[ObjectId],
        graph: &mut Graph<'_, Flags>,
    ) -> Result<Option<Vec<ObjectId>>, Error> {
        let _span = gix_trace::coarse!(
            "gix_revision::merge_base()",
            %first,
            %others,
        );
        if others.is_empty() || others.contains(&first) {
            return Ok(Some(vec![first]));
        }

        let mut queue = PriorityQueue::<GenThenTime, ObjectId>::new();
        graph.insert_data(first, |commit| -> Result<_, Error> {
            queue.insert(commit.try_into()?, first);
            Ok(Flags::COMMIT1)
        })?;

        for other in others {
            graph.insert_data(*other, |commit| -> Result<_, Error> {
                queue.insert(commit.try_into()?, *other);
                Ok(Flags::COMMIT2)
            })?;
        }
        Ok(None)
    }

    // TODO(ST): Should this type be used for `describe` as well?
    struct GenThenTime {
        /// Note that the special [`GENERATION_NUMBER_INFINITY`](gix_commitgraph::GENERATION_NUMBER_INFINITY) is used to indicate
        /// that no commitgraph is avaialble.
        generation: gix_revwalk::graph::Generation,
        time: gix_date::SecondsSinceUnixEpoch,
    }

    impl TryFrom<gix_revwalk::graph::LazyCommit<'_>> for GenThenTime {
        type Error = gix_object::decode::Error;

        fn try_from(commit: LazyCommit<'_>) -> Result<Self, Self::Error> {
            Ok(GenThenTime {
                generation: commit
                    .generation()
                    .unwrap_or(gix_commitgraph::GENERATION_NUMBER_INFINITY),
                time: commit.committer_timestamp()?,
            })
        }
    }

    impl Eq for GenThenTime {}

    impl PartialEq<Self> for GenThenTime {
        fn eq(&self, other: &Self) -> bool {
            self.cmp(other).is_eq()
        }
    }

    impl PartialOrd<Self> for GenThenTime {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.cmp(&other).into()
        }
    }

    impl Ord for GenThenTime {
        fn cmp(&self, other: &Self) -> Ordering {
            self.generation.cmp(&other.generation).then(self.time.cmp(&other.time))
        }
    }
}
