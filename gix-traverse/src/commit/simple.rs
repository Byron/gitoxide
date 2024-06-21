use gix_date::SecondsSinceUnixEpoch;
use gix_hash::ObjectId;
use gix_hashtable::HashSet;
use smallvec::SmallVec;
use std::collections::VecDeque;

/// Specify how to sort commits during a [simple](super::Simple) traversal.
///
/// ### Sample History
///
/// The following history will be referred to for explaining how the sort order works, with the number denoting the commit timestamp
/// (*their X-alignment doesn't matter*).
///
/// ```text
/// ---1----2----4----7 <- second parent of 8
///     \              \
///      3----5----6----8---
/// ```
#[derive(Default, Debug, Copy, Clone)]
pub enum Sorting {
    /// Commits are sorted as they are mentioned in the commit graph.
    ///
    /// In the *sample history* the order would be `8, 6, 7, 5, 4, 3, 2, 1`
    ///
    /// ### Note
    ///
    /// This is not to be confused with `git log/rev-list --topo-order`, which is notably different from
    /// as it avoids overlapping branches.
    #[default]
    BreadthFirst,
    /// Commits are sorted by their commit time in descending order, that is newest first.
    ///
    /// The sorting applies to all currently queued commit ids and thus is full.
    ///
    /// In the *sample history* the order would be `8, 7, 6, 5, 4, 3, 2, 1`
    ///
    /// # Performance
    ///
    /// This mode benefits greatly from having an object_cache in `find()`
    /// to avoid having to lookup each commit twice.
    ByCommitTimeNewestFirst,
    /// This sorting is similar to `ByCommitTimeNewestFirst`, but adds a cutoff to not return commits older than
    /// a given time, stopping the iteration once no younger commits is queued to be traversed.
    ///
    /// As the query is usually repeated with different cutoff dates, this search mode benefits greatly from an object cache.
    ///
    /// In the *sample history* and a cut-off date of 4, the returned list of commits would be `8, 7, 6, 4`
    ByCommitTimeNewestFirstCutoffOlderThan {
        /// The amount of seconds since unix epoch, the same value obtained by any `gix_date::Time` structure and the way git counts time.
        seconds: gix_date::SecondsSinceUnixEpoch,
    },
}

/// The error is part of the item returned by the [Ancestors](super::Simple) iterator.
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error(transparent)]
    Find(#[from] gix_object::find::existing_iter::Error),
    #[error(transparent)]
    ObjectDecode(#[from] gix_object::decode::Error),
}

/// The state used and potentially shared by multiple graph traversals.
#[derive(Clone)]
pub(super) struct State {
    next: VecDeque<ObjectId>,
    queue: gix_revwalk::PriorityQueue<SecondsSinceUnixEpoch, ObjectId>,
    buf: Vec<u8>,
    seen: HashSet<ObjectId>,
    parents_buf: Vec<u8>,
    parent_ids: SmallVec<[(ObjectId, SecondsSinceUnixEpoch); 2]>,
}

///
#[allow(clippy::empty_docs)]
mod init {
    use gix_date::SecondsSinceUnixEpoch;
    use gix_hash::{oid, ObjectId};
    use gix_object::{CommitRefIter, FindExt};

    use super::{
        super::{simple::Sorting, Either, Info, ParentIds, Parents, Simple},
        collect_parents, Error, State,
    };

    impl Default for State {
        fn default() -> Self {
            State {
                next: Default::default(),
                queue: gix_revwalk::PriorityQueue::new(),
                buf: vec![],
                seen: Default::default(),
                parents_buf: vec![],
                parent_ids: Default::default(),
            }
        }
    }

    impl State {
        fn clear(&mut self) {
            self.next.clear();
            self.queue.clear();
            self.buf.clear();
            self.seen.clear();
        }
    }

    /// Builder
    impl<Find, Predicate> Simple<Find, Predicate>
    where
        Find: gix_object::Find,
    {
        /// Set the `sorting` method.
        pub fn sorting(mut self, sorting: Sorting) -> Result<Self, Error> {
            self.sorting = sorting;
            match self.sorting {
                Sorting::BreadthFirst => {
                    self.queue_to_vecdeque();
                }
                Sorting::ByCommitTimeNewestFirst | Sorting::ByCommitTimeNewestFirstCutoffOlderThan { .. } => {
                    let cutoff_time = self.sorting.cutoff_time();
                    let state = &mut self.state;
                    for commit_id in state.next.drain(..) {
                        let commit_iter = self.objects.find_commit_iter(&commit_id, &mut state.buf)?;
                        let time = commit_iter.committer()?.time.seconds;
                        match cutoff_time {
                            Some(cutoff_time) if time >= cutoff_time => {
                                state.queue.insert(time, commit_id);
                            }
                            Some(_) => {}
                            None => {
                                state.queue.insert(time, commit_id);
                            }
                        }
                    }
                }
            }
            Ok(self)
        }

        /// Change our commit parent handling mode to the given one.
        pub fn parents(mut self, mode: Parents) -> Self {
            self.parents = mode;
            if matches!(self.parents, Parents::First) {
                self.queue_to_vecdeque();
            }
            self
        }

        /// Set the commitgraph as `cache` to greatly accelerate any traversal.
        ///
        /// The cache will be used if possible, but we will fall-back without error to using the object
        /// database for commit lookup. If the cache is corrupt, we will fall back to the object database as well.
        pub fn commit_graph(mut self, cache: Option<gix_commitgraph::Graph>) -> Self {
            self.cache = cache;
            self
        }

        fn queue_to_vecdeque(&mut self) {
            let state = &mut self.state;
            state.next.extend(
                std::mem::replace(&mut state.queue, gix_revwalk::PriorityQueue::new())
                    .into_iter_unordered()
                    .map(|(_time, id)| id),
            );
        }
    }

    /// Lifecycle
    impl<Find> Simple<Find, fn(&oid) -> bool>
    where
        Find: gix_object::Find,
    {
        /// Create a new instance.
        ///
        /// * `find` - a way to lookup new object data during traversal by their `ObjectId`, writing their data into buffer and returning
        ///    an iterator over commit tokens if the object is present and is a commit. Caching should be implemented within this function
        ///    as needed.
        /// * `tips`
        ///   * the starting points of the iteration, usually commits
        ///   * each commit they lead to will only be returned once, including the tip that started it
        pub fn new(tips: impl IntoIterator<Item = impl Into<ObjectId>>, find: Find) -> Self {
            Self::filtered(tips, find, |_| true)
        }
    }

    /// Lifecycle
    impl<Find, Predicate> Simple<Find, Predicate>
    where
        Find: gix_object::Find,
        Predicate: FnMut(&oid) -> bool,
    {
        /// Create a new instance with commit filtering enabled.
        ///
        /// * `find` - a way to lookup new object data during traversal by their `ObjectId`, writing their data into buffer and returning
        ///    an iterator over commit tokens if the object is present and is a commit. Caching should be implemented within this function
        ///    as needed.
        /// * `tips`
        ///   * the starting points of the iteration, usually commits
        ///   * each commit they lead to will only be returned once, including the tip that started it
        /// * `predicate` - indicate whether a given commit should be included in the result as well
        ///   as whether its parent commits should be traversed.
        pub fn filtered(
            tips: impl IntoIterator<Item = impl Into<ObjectId>>,
            find: Find,
            mut predicate: Predicate,
        ) -> Self {
            let tips = tips.into_iter();
            let mut state = State::default();
            {
                state.clear();
                state.next.reserve(tips.size_hint().0);
                for tip in tips.map(Into::into) {
                    let was_inserted = state.seen.insert(tip);
                    if was_inserted && predicate(&tip) {
                        state.next.push_back(tip);
                    }
                }
            }
            Self {
                objects: find,
                cache: None,
                predicate,
                state,
                parents: Default::default(),
                sorting: Default::default(),
            }
        }
    }

    /// Access
    impl<Find, Predicate> Simple<Find, Predicate> {
        /// Return an iterator for accessing data of the current commit, parsed lazily.
        pub fn commit_iter(&self) -> CommitRefIter<'_> {
            CommitRefIter::from_bytes(&self.state.buf)
        }

        /// Return the current commits' raw data, which can be parsed using [`gix_object::CommitRef::from_bytes()`].
        pub fn commit_data(&self) -> &[u8] {
            &self.state.buf
        }
    }

    impl<Find, Predicate> Iterator for Simple<Find, Predicate>
    where
        Find: gix_object::Find,
        Predicate: FnMut(&oid) -> bool,
    {
        type Item = Result<Info, Error>;

        fn next(&mut self) -> Option<Self::Item> {
            if matches!(self.parents, Parents::First) {
                self.next_by_topology()
            } else {
                match self.sorting {
                    Sorting::BreadthFirst => self.next_by_topology(),
                    Sorting::ByCommitTimeNewestFirst => self.next_by_commit_date(None),
                    Sorting::ByCommitTimeNewestFirstCutoffOlderThan { seconds } => {
                        self.next_by_commit_date(seconds.into())
                    }
                }
            }
        }
    }

    impl Sorting {
        /// If not topo sort, provide the cutoff date if present.
        fn cutoff_time(&self) -> Option<SecondsSinceUnixEpoch> {
            match self {
                Sorting::ByCommitTimeNewestFirstCutoffOlderThan { seconds } => Some(*seconds),
                _ => None,
            }
        }
    }

    /// Utilities
    impl<Find, Predicate> Simple<Find, Predicate>
    where
        Find: gix_object::Find,
        Predicate: FnMut(&oid) -> bool,
    {
        fn next_by_commit_date(
            &mut self,
            cutoff_older_than: Option<SecondsSinceUnixEpoch>,
        ) -> Option<Result<Info, Error>> {
            let state = &mut self.state;

            let (commit_time, oid) = state.queue.pop()?;
            let mut parents: ParentIds = Default::default();
            match super::super::find(self.cache.as_ref(), &self.objects, &oid, &mut state.buf) {
                Ok(Either::CachedCommit(commit)) => {
                    if !collect_parents(&mut state.parent_ids, self.cache.as_ref(), commit.iter_parents()) {
                        // drop corrupt caches and try again with ODB
                        self.cache = None;
                        return self.next_by_commit_date(cutoff_older_than);
                    }
                    for (id, parent_commit_time) in state.parent_ids.drain(..) {
                        parents.push(id);
                        let was_inserted = state.seen.insert(id);
                        if !(was_inserted && (self.predicate)(&id)) {
                            continue;
                        }

                        match cutoff_older_than {
                            Some(cutoff_older_than) if parent_commit_time < cutoff_older_than => continue,
                            Some(_) | None => state.queue.insert(parent_commit_time, id),
                        }
                    }
                }
                Ok(Either::CommitRefIter(commit_iter)) => {
                    for token in commit_iter {
                        match token {
                            Ok(gix_object::commit::ref_iter::Token::Tree { .. }) => continue,
                            Ok(gix_object::commit::ref_iter::Token::Parent { id }) => {
                                parents.push(id);
                                let was_inserted = state.seen.insert(id);
                                if !(was_inserted && (self.predicate)(&id)) {
                                    continue;
                                }

                                let parent = self.objects.find_commit_iter(id.as_ref(), &mut state.parents_buf).ok();
                                let parent_commit_time = parent
                                    .and_then(|parent| parent.committer().ok().map(|committer| committer.time.seconds))
                                    .unwrap_or_default();

                                match cutoff_older_than {
                                    Some(cutoff_older_than) if parent_commit_time < cutoff_older_than => continue,
                                    Some(_) | None => state.queue.insert(parent_commit_time, id),
                                }
                            }
                            Ok(_unused_token) => break,
                            Err(err) => return Some(Err(err.into())),
                        }
                    }
                }
                Err(err) => return Some(Err(err.into())),
            }
            Some(Ok(Info {
                id: oid,
                parent_ids: parents,
                commit_time: Some(commit_time),
            }))
        }
    }

    /// Utilities
    impl<Find, Predicate> Simple<Find, Predicate>
    where
        Find: gix_object::Find,
        Predicate: FnMut(&oid) -> bool,
    {
        fn next_by_topology(&mut self) -> Option<Result<Info, Error>> {
            let state = &mut self.state;
            let oid = state.next.pop_front()?;
            let mut parents: ParentIds = Default::default();
            match super::super::find(self.cache.as_ref(), &self.objects, &oid, &mut state.buf) {
                Ok(Either::CachedCommit(commit)) => {
                    if !collect_parents(&mut state.parent_ids, self.cache.as_ref(), commit.iter_parents()) {
                        // drop corrupt caches and try again with ODB
                        self.cache = None;
                        return self.next_by_topology();
                    }

                    for (id, _commit_time) in state.parent_ids.drain(..) {
                        parents.push(id);
                        let was_inserted = state.seen.insert(id);
                        if was_inserted && (self.predicate)(&id) {
                            state.next.push_back(id);
                        }
                        if matches!(self.parents, Parents::First) {
                            break;
                        }
                    }
                }
                Ok(Either::CommitRefIter(commit_iter)) => {
                    for token in commit_iter {
                        match token {
                            Ok(gix_object::commit::ref_iter::Token::Tree { .. }) => continue,
                            Ok(gix_object::commit::ref_iter::Token::Parent { id }) => {
                                parents.push(id);
                                let was_inserted = state.seen.insert(id);
                                if was_inserted && (self.predicate)(&id) {
                                    state.next.push_back(id);
                                }
                                if matches!(self.parents, Parents::First) {
                                    break;
                                }
                            }
                            Ok(_a_token_past_the_parents) => break,
                            Err(err) => return Some(Err(err.into())),
                        }
                    }
                }
                Err(err) => return Some(Err(err.into())),
            }
            Some(Ok(Info {
                id: oid,
                parent_ids: parents,
                commit_time: None,
            }))
        }
    }
}

fn collect_parents(
    dest: &mut SmallVec<[(gix_hash::ObjectId, gix_date::SecondsSinceUnixEpoch); 2]>,
    cache: Option<&gix_commitgraph::Graph>,
    parents: gix_commitgraph::file::commit::Parents<'_>,
) -> bool {
    dest.clear();
    let cache = cache.as_ref().expect("parents iter is available, backed by `cache`");
    for parent_id in parents {
        match parent_id {
            Ok(pos) => dest.push({
                let parent = cache.commit_at(pos);
                (
                    parent.id().to_owned(),
                    parent.committer_timestamp() as gix_date::SecondsSinceUnixEpoch, // we can't handle errors here and trying seems overkill
                )
            }),
            Err(_err) => return false,
        }
    }
    true
}
