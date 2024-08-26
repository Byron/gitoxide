bitflags::bitflags! {
    /// The flags used in the graph for finding [merge bases](crate::merge_base()).
    #[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
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
    #[error(transparent)]
    IterParents(#[from] gix_revwalk::graph::commit::iter_parents::Error),
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
    pub fn merge_base(
        first: ObjectId,
        others: &[ObjectId],
        graph: &mut Graph<'_, '_, Flags>,
    ) -> Result<Option<Vec<ObjectId>>, Error> {
        let _span = gix_trace::coarse!("gix_revision::merge_base()", ?first, ?others,);
        if others.is_empty() || others.contains(&first) {
            return Ok(Some(vec![first]));
        }

        let bases = paint_down_to_common(first, others, graph)?;
        graph.clear();

        let bases = remove_redundant(&bases, graph)?;
        Ok((!bases.is_empty()).then_some(bases))
    }

    /// Remove all those commits from `commits` if they are in the history of another commit in `commits`.
    /// That way, we return only the topologically most recent commits in `commits`.
    fn remove_redundant(
        commits: &[(ObjectId, GenThenTime)],
        graph: &mut Graph<'_, '_, Flags>,
    ) -> Result<Vec<ObjectId>, Error> {
        if commits.is_empty() {
            return Ok(Vec::new());
        }
        let sorted_commits = {
            let mut v = commits.to_vec();
            v.sort_by(|a, b| a.1.cmp(&b.1));
            v
        };
        let mut min_gen_pos = 0;
        let mut min_gen = sorted_commits[min_gen_pos].1.generation;

        let mut walk_start = Vec::with_capacity(commits.len());
        for (id, _) in commits {
            graph.insert(*id, Flags::RESULT);
            graph.insert_parents_with_lookup(id, &mut |parent_id, parent_data, maybe_flags| -> Result<_, Error> {
                if maybe_flags.is_none() {
                    walk_start.push((parent_id, GenThenTime::try_from(parent_data)?));
                }
                Ok(Flags::empty())
            })?;
        }
        walk_start.sort_by(|a, b| a.0.cmp(&b.0));
        let mut count_still_independent = commits.len();

        let mut stack = Vec::new();
        while let Some((commit_id, commit_info)) = walk_start.pop().filter(|_| count_still_independent > 1) {
            stack.clear();
            graph.insert(commit_id, Flags::STALE);
            stack.push((commit_id, commit_info));

            while let Some((commit_id, commit_info)) = stack.last().copied() {
                let flags = graph.get_mut(&commit_id).expect("all commits have been added");
                if flags.contains(Flags::RESULT) {
                    flags.remove(Flags::RESULT);
                    count_still_independent -= 1;
                    if count_still_independent <= 1 {
                        break;
                    }
                    if commit_id == sorted_commits[min_gen_pos].0 {
                        while min_gen_pos < commits.len() - 1
                            && graph
                                .get(&sorted_commits[min_gen_pos].0)
                                .expect("already added")
                                .contains(Flags::STALE)
                        {
                            min_gen_pos += 1;
                        }
                        min_gen = sorted_commits[min_gen_pos].1.generation;
                    }
                }

                if commit_info.generation < min_gen {
                    stack.pop();
                    continue;
                }

                let mut pushed_one_parent = false;
                graph.insert_parents_with_lookup(&commit_id, &mut |parent_id,
                                                                    parent_data,
                                                                    maybe_flags|
                 -> Result<_, Error> {
                    let is_new_parent = !pushed_one_parent
                        && maybe_flags.map_or(true, |flags| {
                            let res = !flags.contains(Flags::STALE);
                            *flags |= Flags::STALE;
                            res
                        });
                    if is_new_parent {
                        stack.push((parent_id, GenThenTime::try_from(parent_data)?));
                        pushed_one_parent = true;
                    }
                    Ok(Flags::STALE)
                })?;

                if !pushed_one_parent {
                    stack.pop();
                }
            }
        }

        Ok(commits
            .iter()
            .filter_map(|(id, _info)| graph.get(id).filter(|flags| !flags.contains(Flags::STALE)).map(|_| *id))
            .collect())
    }

    fn paint_down_to_common(
        first: ObjectId,
        others: &[ObjectId],
        graph: &mut Graph<'_, '_, Flags>,
    ) -> Result<Vec<(ObjectId, GenThenTime)>, Error> {
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

        let mut out = Vec::new();
        while queue
            .iter_unordered()
            .any(|id| graph.get(id).map_or(false, |data| !data.contains(Flags::STALE)))
        {
            let (info, commit_id) = queue.pop().expect("we have non-stale");
            let flags_mut = graph.get_mut(&commit_id).expect("everything queued is in graph");
            let mut flags_without_result = *flags_mut & (Flags::COMMIT1 | Flags::COMMIT2 | Flags::STALE);
            if flags_without_result == (Flags::COMMIT1 | Flags::COMMIT2) {
                if !flags_mut.contains(Flags::RESULT) {
                    *flags_mut |= Flags::RESULT;
                    out.push((commit_id, info));
                }
                flags_without_result |= Flags::STALE;
            }

            graph.insert_parents_with_lookup(&commit_id, &mut |parent_id, parent, ex_flags| -> Result<_, Error> {
                let queue_info = match ex_flags {
                    Some(ex_flags) => {
                        if (*ex_flags & flags_without_result) != flags_without_result {
                            *ex_flags |= flags_without_result;
                            Some(GenThenTime::try_from(parent)?)
                        } else {
                            None
                        }
                    }
                    None => Some(GenThenTime::try_from(parent)?),
                };
                if let Some(info) = queue_info {
                    queue.insert(info, parent_id);
                }
                Ok(flags_without_result)
            })?;
        }

        Ok(out)
    }

    // TODO(ST): Should this type be used for `describe` as well?
    #[derive(Debug, Clone, Copy)]
    struct GenThenTime {
        /// Note that the special [`GENERATION_NUMBER_INFINITY`](gix_commitgraph::GENERATION_NUMBER_INFINITY) is used to indicate
        /// that no commitgraph is avaialble.
        generation: gix_revwalk::graph::Generation,
        time: gix_date::SecondsSinceUnixEpoch,
    }

    impl TryFrom<gix_revwalk::graph::LazyCommit<'_, '_>> for GenThenTime {
        type Error = gix_object::decode::Error;

        fn try_from(commit: LazyCommit<'_, '_>) -> Result<Self, Self::Error> {
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
            Some(self.cmp(other))
        }
    }

    impl Ord for GenThenTime {
        fn cmp(&self, other: &Self) -> Ordering {
            self.generation.cmp(&other.generation).then(self.time.cmp(&other.time))
        }
    }
}
