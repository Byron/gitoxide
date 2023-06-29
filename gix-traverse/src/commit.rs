use smallvec::SmallVec;

/// An iterator over the ancestors one or more starting commits
pub struct Ancestors<Find, Predicate, StateMut> {
    find: Find,
    cache: Option<gix_commitgraph::Graph>,
    predicate: Predicate,
    state: StateMut,
    parents: Parents,
    sorting: Sorting,
}

/// Specify how to handle commit parents during traversal.
#[derive(Default, Copy, Clone)]
pub enum Parents {
    /// Traverse all parents, useful for traversing the entire ancestry.
    #[default]
    All,
    /// Only traverse along the first parent, which commonly ignores all branches.
    First,
}

/// Specify how to sort commits during traversal.
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

/// The collection of parent ids we saw as part of the iteration.
///
/// Note that this list is truncated if [`Parents::First`] was used.
pub type ParentIds = SmallVec<[gix_hash::ObjectId; 1]>;

/// Information about a commit that we obtained naturally as part of the iteration.
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Info {
    /// The id of the commit.
    pub id: gix_hash::ObjectId,
    /// All parent ids we have encountered. Note that these will be at most one if [`Parents::First`] is enabled.
    pub parent_ids: ParentIds,
    /// The time at which the commit was created. It's only `Some(_)` if sorting is not [`Sorting::BreadthFirst`], as the walk
    /// needs to require the commit-date.
    pub commit_time: Option<gix_date::SecondsSinceUnixEpoch>,
}

///
pub mod ancestors {
    use std::{
        borrow::{Borrow, BorrowMut},
        collections::VecDeque,
    };

    use gix_date::SecondsSinceUnixEpoch;
    use gix_hash::{oid, ObjectId};
    use gix_hashtable::HashSet;
    use gix_object::CommitRefIter;
    use smallvec::SmallVec;

    use crate::commit::{collect_parents, Ancestors, Either, Info, ParentIds, Parents, Sorting};

    /// The error is part of the item returned by the [Ancestors] iterator.
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("The commit {oid} could not be found")]
        FindExisting {
            oid: ObjectId,
            source: Box<dyn std::error::Error + Send + Sync + 'static>,
        },
        #[error(transparent)]
        ObjectDecode(#[from] gix_object::decode::Error),
    }

    /// The state used and potentially shared by multiple graph traversals.
    #[derive(Clone)]
    pub struct State {
        next: VecDeque<ObjectId>,
        queue: gix_revwalk::PriorityQueue<SecondsSinceUnixEpoch, ObjectId>,
        buf: Vec<u8>,
        seen: HashSet<ObjectId>,
        parents_buf: Vec<u8>,
        parent_ids: SmallVec<[(ObjectId, SecondsSinceUnixEpoch); 2]>,
    }

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
    impl<Find, Predicate, StateMut, E> Ancestors<Find, Predicate, StateMut>
    where
        Find: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Result<CommitRefIter<'a>, E>,
        StateMut: BorrowMut<State>,
        E: std::error::Error + Send + Sync + 'static,
    {
        /// Set the sorting method, either topological or by author date
        pub fn sorting(mut self, sorting: Sorting) -> Result<Self, Error> {
            self.sorting = sorting;
            match self.sorting {
                Sorting::BreadthFirst => {
                    self.queue_to_vecdeque();
                }
                Sorting::ByCommitTimeNewestFirst | Sorting::ByCommitTimeNewestFirstCutoffOlderThan { .. } => {
                    let cutoff_time = self.sorting.cutoff_time();
                    let state = self.state.borrow_mut();
                    for commit_id in state.next.drain(..) {
                        let commit_iter =
                            (self.find)(&commit_id, &mut state.buf).map_err(|err| Error::FindExisting {
                                oid: commit_id,
                                source: err.into(),
                            })?;
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
            let state = self.state.borrow_mut();
            state.next.extend(
                std::mem::replace(&mut state.queue, gix_revwalk::PriorityQueue::new())
                    .into_iter_unordered()
                    .map(|(_time, id)| id),
            );
        }
    }

    /// Initialization
    impl<Find, StateMut, E> Ancestors<Find, fn(&oid) -> bool, StateMut>
    where
        Find: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Result<CommitRefIter<'a>, E>,
        StateMut: BorrowMut<State>,
        E: std::error::Error + Send + Sync + 'static,
    {
        /// Create a new instance.
        ///
        /// * `find` - a way to lookup new object data during traversal by their `ObjectId`, writing their data into buffer and returning
        ///    an iterator over commit tokens if the object is present and is a commit. Caching should be implemented within this function
        ///    as needed.
        /// * `state` - all state used for the traversal. If multiple traversals are performed, allocations can be minimized by reusing
        ///   this state.
        /// * `tips`
        ///   * the starting points of the iteration, usually commits
        ///   * each commit they lead to will only be returned once, including the tip that started it
        pub fn new(tips: impl IntoIterator<Item = impl Into<ObjectId>>, state: StateMut, find: Find) -> Self {
            Self::filtered(tips, state, find, |_| true)
        }
    }

    /// Initialization
    impl<Find, Predicate, StateMut, E> Ancestors<Find, Predicate, StateMut>
    where
        Find: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Result<CommitRefIter<'a>, E>,
        Predicate: FnMut(&oid) -> bool,
        StateMut: BorrowMut<State>,
        E: std::error::Error + Send + Sync + 'static,
    {
        /// Create a new instance with commit filtering enabled.
        ///
        /// * `find` - a way to lookup new object data during traversal by their `ObjectId`, writing their data into buffer and returning
        ///    an iterator over commit tokens if the object is present and is a commit. Caching should be implemented within this function
        ///    as needed.
        /// * `state` - all state used for the traversal. If multiple traversals are performed, allocations can be minimized by reusing
        ///   this state.
        /// * `tips`
        ///   * the starting points of the iteration, usually commits
        ///   * each commit they lead to will only be returned once, including the tip that started it
        /// * `predicate` - indicate whether a given commit should be included in the result as well
        ///   as whether its parent commits should be traversed.
        pub fn filtered(
            tips: impl IntoIterator<Item = impl Into<ObjectId>>,
            mut state: StateMut,
            find: Find,
            mut predicate: Predicate,
        ) -> Self {
            let tips = tips.into_iter();
            {
                let state = state.borrow_mut();
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
                find,
                cache: None,
                predicate,
                state,
                parents: Default::default(),
                sorting: Default::default(),
            }
        }
    }
    /// Access
    impl<Find, Predicate, StateMut> Ancestors<Find, Predicate, StateMut>
    where
        StateMut: Borrow<State>,
    {
        /// Return an iterator for accessing more of the current commits data.
        pub fn commit_iter(&self) -> CommitRefIter<'_> {
            CommitRefIter::from_bytes(&self.state.borrow().buf)
        }

        /// Return the current commits data.
        pub fn commit_data(&self) -> &[u8] {
            &self.state.borrow().buf
        }
    }

    impl<Find, Predicate, StateMut, E> Iterator for Ancestors<Find, Predicate, StateMut>
    where
        Find: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Result<CommitRefIter<'a>, E>,
        Predicate: FnMut(&oid) -> bool,
        StateMut: BorrowMut<State>,
        E: std::error::Error + Send + Sync + 'static,
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
    impl<Find, Predicate, StateMut, E> Ancestors<Find, Predicate, StateMut>
    where
        Find: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Result<CommitRefIter<'a>, E>,
        Predicate: FnMut(&oid) -> bool,
        StateMut: BorrowMut<State>,
        E: std::error::Error + Send + Sync + 'static,
    {
        fn next_by_commit_date(
            &mut self,
            cutoff_older_than: Option<SecondsSinceUnixEpoch>,
        ) -> Option<Result<Info, Error>> {
            let state = self.state.borrow_mut();

            let (commit_time, oid) = state.queue.pop()?;
            let mut parents: ParentIds = Default::default();
            match super::find(self.cache.as_ref(), &mut self.find, &oid, &mut state.buf) {
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

                                let parent = (self.find)(id.as_ref(), &mut state.parents_buf).ok();
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
                Err(err) => {
                    return Some(Err(Error::FindExisting {
                        oid,
                        source: err.into(),
                    }))
                }
            }
            Some(Ok(Info {
                id: oid,
                parent_ids: parents,
                commit_time: Some(commit_time),
            }))
        }
    }

    /// Utilities
    impl<Find, Predicate, StateMut, E> Ancestors<Find, Predicate, StateMut>
    where
        Find: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Result<CommitRefIter<'a>, E>,
        Predicate: FnMut(&oid) -> bool,
        StateMut: BorrowMut<State>,
        E: std::error::Error + Send + Sync + 'static,
    {
        fn next_by_topology(&mut self) -> Option<Result<Info, Error>> {
            let state = self.state.borrow_mut();
            let oid = state.next.pop_front()?;
            let mut parents: ParentIds = Default::default();
            match super::find(self.cache.as_ref(), &mut self.find, &oid, &mut state.buf) {
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
                Err(err) => {
                    return Some(Err(Error::FindExisting {
                        oid,
                        source: err.into(),
                    }))
                }
            }
            Some(Ok(Info {
                id: oid,
                parent_ids: parents,
                commit_time: None,
            }))
        }
    }
}

enum Either<'buf, 'cache> {
    CommitRefIter(gix_object::CommitRefIter<'buf>),
    CachedCommit(gix_commitgraph::file::Commit<'cache>),
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

fn find<'cache, 'buf, Find, E>(
    cache: Option<&'cache gix_commitgraph::Graph>,
    mut find: Find,
    id: &gix_hash::oid,
    buf: &'buf mut Vec<u8>,
) -> Result<Either<'buf, 'cache>, E>
where
    Find: for<'a> FnMut(&gix_hash::oid, &'a mut Vec<u8>) -> Result<gix_object::CommitRefIter<'a>, E>,
    E: std::error::Error + Send + Sync + 'static,
{
    match cache.and_then(|cache| cache.commit_by_id(id).map(Either::CachedCommit)) {
        Some(c) => Ok(c),
        None => find(id, buf).map(Either::CommitRefIter),
    }
}
