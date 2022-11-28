/// An iterator over the ancestors one or more starting commits
pub struct Ancestors<Find, Predicate, StateMut> {
    find: Find,
    predicate: Predicate,
    state: StateMut,
    parents: Parents,
    sorting: Sorting,
}

/// Specify how to handle commit parents during traversal.
#[derive(Copy, Clone)]
pub enum Parents {
    /// Traverse all parents, useful for traversing the entire ancestry.
    All,
    /// Only traverse along the first parent, which commonly ignores all branches.
    First,
}

impl Default for Parents {
    fn default() -> Self {
        Parents::All
    }
}

/// Specify how to sort commits during traversal.
#[derive(Copy, Clone)]
pub enum Sorting {
    /// Commits are sorted as they are mentioned in the commit graph.
    Topological,
    /// Commits are sorted by their commit time in descending order, that is newest first.
    ///
    /// The sorting applies to all currently queued commit ids and thus is full.
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
    ByCommitTimeNewestFirstCutoffOlderThan {
        /// The amount of seconds since unix epoch, the same value obtained by any `git_date::Time` structure and the way git counts time.
        time_in_seconds_since_epoch: u32,
    },
}

impl Default for Sorting {
    fn default() -> Self {
        Sorting::Topological
    }
}

///
pub mod ancestors {
    use std::{
        borrow::{Borrow, BorrowMut},
        collections::VecDeque,
        iter::FromIterator,
    };

    use git_hash::{oid, ObjectId};
    use git_hashtable::HashSet;
    use git_object::CommitRefIter;

    use crate::commit::{Ancestors, Parents, Sorting};

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
        ObjectDecode(#[from] git_object::decode::Error),
    }

    type TimeInSeconds = u32;

    /// The state used and potentially shared by multiple graph traversals.
    #[derive(Default, Clone)]
    pub struct State {
        next: VecDeque<(ObjectId, TimeInSeconds)>,
        buf: Vec<u8>,
        seen: HashSet<ObjectId>,
        parents_buf: Vec<u8>,
    }

    impl State {
        fn clear(&mut self) {
            self.next.clear();
            self.buf.clear();
            self.seen.clear();
        }
    }

    /// Builder
    impl<Find, Predicate, StateMut> Ancestors<Find, Predicate, StateMut> {
        /// Change our commit parent handling mode to the given one.
        pub fn parents(mut self, mode: Parents) -> Self {
            self.parents = mode;
            self
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
            if !matches!(self.sorting, Sorting::Topological) {
                let mut cutoff_time_storage = self.sorting.cutoff_time().map(|cot| (cot, Vec::new()));
                let state = self.state.borrow_mut();
                for (commit_id, commit_time) in state.next.iter_mut() {
                    let commit_iter = (self.find)(commit_id, &mut state.buf).map_err(|err| Error::FindExisting {
                        oid: *commit_id,
                        source: err.into(),
                    })?;
                    let time = commit_iter.committer()?.time.seconds_since_unix_epoch;
                    match &mut cutoff_time_storage {
                        Some((cutoff_time, storage)) if time >= *cutoff_time => {
                            storage.push((*commit_id, time));
                        }
                        Some(_) => {}
                        None => *commit_time = time,
                    }
                }
                let mut v = match cutoff_time_storage {
                    Some((_, storage)) => storage,
                    None => Vec::from_iter(std::mem::take(&mut state.next).into_iter()),
                };
                v.sort_by(|a, b| a.1.cmp(&b.1).reverse());
                state.next = v.into();
            }
            Ok(self)
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
        /// * `find` - a way to lookup new object data during traversal by their ObjectId, writing their data into buffer and returning
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
        /// * `find` - a way to lookup new object data during traversal by their ObjectId, writing their data into buffer and returning
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
                        state.next.push_back((tip, 0));
                    }
                }
            }
            Self {
                find,
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
    }

    impl<Find, Predicate, StateMut, E> Iterator for Ancestors<Find, Predicate, StateMut>
    where
        Find: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Result<CommitRefIter<'a>, E>,
        Predicate: FnMut(&oid) -> bool,
        StateMut: BorrowMut<State>,
        E: std::error::Error + Send + Sync + 'static,
    {
        type Item = Result<ObjectId, Error>;

        fn next(&mut self) -> Option<Self::Item> {
            if matches!(self.parents, Parents::First) {
                self.next_by_topology()
            } else {
                match self.sorting {
                    Sorting::Topological => self.next_by_topology(),
                    Sorting::ByCommitTimeNewestFirst => self.next_by_commit_date(None),
                    Sorting::ByCommitTimeNewestFirstCutoffOlderThan {
                        time_in_seconds_since_epoch,
                    } => self.next_by_commit_date(time_in_seconds_since_epoch.into()),
                }
            }
        }
    }

    impl Sorting {
        /// If not topo sort, provide the cutoff date if present.
        fn cutoff_time(&self) -> Option<u32> {
            match self {
                Sorting::ByCommitTimeNewestFirstCutoffOlderThan {
                    time_in_seconds_since_epoch,
                } => Some(*time_in_seconds_since_epoch),
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
        fn next_by_commit_date(&mut self, cutoff_older_than: Option<TimeInSeconds>) -> Option<Result<ObjectId, Error>> {
            let state = self.state.borrow_mut();

            let (oid, _commit_time) = state.next.pop_front()?;
            match (self.find)(&oid, &mut state.buf) {
                Ok(commit_iter) => {
                    let mut count = 0;
                    for token in commit_iter {
                        count += 1;
                        let is_first = count == 1;
                        match token {
                            Ok(git_object::commit::ref_iter::Token::Tree { .. }) => continue,
                            Ok(git_object::commit::ref_iter::Token::Parent { id }) => {
                                let was_inserted = state.seen.insert(id);
                                if !(was_inserted && (self.predicate)(&id)) {
                                    if is_first && matches!(self.parents, Parents::First) {
                                        break;
                                    } else {
                                        continue;
                                    }
                                }

                                let parent = (self.find)(id.as_ref(), &mut state.parents_buf).ok();
                                let parent_commit_time = parent
                                    .and_then(|parent| {
                                        parent
                                            .committer()
                                            .ok()
                                            .map(|committer| committer.time.seconds_since_unix_epoch)
                                    })
                                    .unwrap_or_default();

                                let pos = match state.next.binary_search_by(|c| c.1.cmp(&parent_commit_time).reverse())
                                {
                                    Ok(_) => None,
                                    Err(pos) => Some(pos),
                                };
                                match cutoff_older_than {
                                    Some(cutoff_older_than) if parent_commit_time < cutoff_older_than => continue,
                                    Some(_) | None => match pos {
                                        Some(pos) => state.next.insert(pos, (id, parent_commit_time)),
                                        None => state.next.push_back((id, parent_commit_time)),
                                    },
                                }

                                if is_first && matches!(self.parents, Parents::First) {
                                    break;
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
            Some(Ok(oid))
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
        fn next_by_topology(&mut self) -> Option<Result<ObjectId, Error>> {
            let state = self.state.borrow_mut();
            let (oid, _commit_time) = state.next.pop_front()?;
            match (self.find)(&oid, &mut state.buf) {
                Ok(commit_iter) => {
                    for token in commit_iter {
                        match token {
                            Ok(git_object::commit::ref_iter::Token::Tree { .. }) => continue,
                            Ok(git_object::commit::ref_iter::Token::Parent { id }) => {
                                let was_inserted = state.seen.insert(id);
                                if was_inserted && (self.predicate)(&id) {
                                    state.next.push_back((id, 0));
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
            Some(Ok(oid))
        }
    }
}
