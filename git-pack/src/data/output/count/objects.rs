use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use git_features::{parallel, progress::Progress};
use git_hash::{oid, ObjectId};
use git_object::immutable;

use crate::{data::output, find, FindExt};

/// The return type used by [`objects()`].
pub type Result<E1, E2> = std::result::Result<(Vec<output::Count>, Outcome), Error<E1, E2>>;

/// Generate [`Count`][output::Count]s from input `objects` with object expansion based on [`options`][Options]
/// to learn which objects would would constitute a pack. This step is required to know exactly how many objects would
/// be in a pack while keeping data around to avoid minimize object database access.
///
/// A [`Count`][output::Count] object maintains enough state to greatly accelerate future access of packed objects.
///
/// * `db` - the object store to use for accessing objects.
/// * `make_cache` - a function to create thread-local pack caches
/// * `objects_ids`
///   * A list of objects ids to add to the pack. Duplication checks are performed so no object is ever added to a pack twice.
///   * Objects may be expanded based on the provided [`options`][Options]
/// * `progress`
///   * a way to obtain progress information
/// * `should_interrupt`
///  * A flag that is set to true if the operation should stop
/// * `options`
///   * more configuration
pub fn objects<Find, Iter, IterErr, Oid, Cache>(
    db: Find,
    make_cache: impl Fn() -> Cache + Send + Sync,
    objects_ids: Iter,
    progress: impl Progress,
    should_interrupt: &AtomicBool,
    Options {
        thread_limit,
        input_object_expansion,
        chunk_size,
    }: Options,
) -> Result<find::existing::Error<Find::Error>, IterErr>
where
    Find: crate::Find + Send + Sync,
    <Find as crate::Find>::Error: Send,
    Iter: Iterator<Item = std::result::Result<Oid, IterErr>> + Send,
    Oid: Into<ObjectId> + Send,
    IterErr: std::error::Error + Send,
    Cache: crate::cache::DecodeEntry,
{
    let lower_bound = objects_ids.size_hint().0;
    let (chunk_size, thread_limit, _) = parallel::optimize_chunk_size_and_thread_limit(
        chunk_size,
        if lower_bound == 0 { None } else { Some(lower_bound) },
        thread_limit,
        None,
    );
    let chunks = util::Chunks {
        iter: objects_ids,
        size: chunk_size,
    };
    let seen_objs = dashmap::DashSet::<ObjectId>::new();
    let progress = Arc::new(parking_lot::Mutex::new(progress));

    parallel::in_parallel(
        chunks,
        thread_limit,
        {
            let progress = Arc::clone(&progress);
            move |n| {
                (
                    Vec::new(),   // object data buffer
                    Vec::new(),   // object data buffer 2 to hold two objects at a time
                    make_cache(), // cache to speed up pack operations
                    {
                        let mut p = progress.lock().add_child(format!("thread {}", n));
                        p.init(None, git_features::progress::count("objects"));
                        p
                    },
                )
            }
        },
        {
            move |oids: Vec<std::result::Result<Oid, IterErr>>, (buf1, buf2, cache, progress)| {
                expand_inner(
                    &db,
                    input_object_expansion,
                    &seen_objs,
                    oids,
                    buf1,
                    buf2,
                    cache,
                    progress,
                    should_interrupt,
                    true,
                )
            }
        },
        reduce::Statistics::new(progress),
    )
}

/// Like [`objects()`] but using a single thread only to mostly save on the otherwise required overhead.
pub fn objects_unthreaded<Find, IterErr, Oid>(
    db: Find,
    pack_cache: &mut impl crate::cache::DecodeEntry,
    object_ids: impl Iterator<Item = std::result::Result<Oid, IterErr>>,
    mut progress: impl Progress,
    should_interrupt: &AtomicBool,
    input_object_expansion: ObjectExpansion,
) -> Result<find::existing::Error<Find::Error>, IterErr>
where
    Find: crate::Find + Send + Sync,
    Oid: Into<ObjectId> + Send,
    IterErr: std::error::Error + Send,
{
    let seen_objs = RefCell::new(HashSet::<ObjectId>::new());

    let (mut buf1, mut buf2) = (Vec::new(), Vec::new());
    expand_inner(
        &db,
        input_object_expansion,
        &seen_objs,
        object_ids,
        &mut buf1,
        &mut buf2,
        pack_cache,
        &mut progress,
        should_interrupt,
        false,
    )
}

#[allow(clippy::too_many_arguments)]
fn expand_inner<Find, IterErr, Oid>(
    db: &Find,
    input_object_expansion: ObjectExpansion,
    seen_objs: &impl util::InsertImmutable<ObjectId>,
    oids: impl IntoIterator<Item = std::result::Result<Oid, IterErr>>,
    buf1: &mut Vec<u8>,
    buf2: &mut Vec<u8>,
    cache: &mut impl crate::cache::DecodeEntry,
    progress: &mut impl Progress,
    should_interrupt: &AtomicBool,
    allow_pack_lookups: bool,
) -> Result<find::existing::Error<Find::Error>, IterErr>
where
    Find: crate::Find + Send + Sync,
    Oid: Into<ObjectId> + Send,
    IterErr: std::error::Error + Send,
{
    use ObjectExpansion::*;

    let mut out = Vec::new();
    let mut tree_traversal_state = git_traverse::tree::breadthfirst::State::default();
    let mut tree_diff_state = git_diff::tree::State::default();
    let mut parent_commit_ids = Vec::new();
    let mut traverse_delegate = tree::traverse::AllUnseen::new(seen_objs);
    let mut changes_delegate = tree::changes::AllNew::new(seen_objs);
    let mut outcome = Outcome::default();
    let stats = &mut outcome;
    for id in oids.into_iter() {
        if should_interrupt.load(Ordering::Relaxed) {
            return Err(Error::Interrupted);
        }

        let id = id.map(|oid| oid.into()).map_err(Error::InputIteration)?;
        let obj = db.find_existing(id, buf1, cache)?;
        stats.input_objects += 1;
        match input_object_expansion {
            TreeAdditionsComparedToAncestor => {
                use git_object::Kind::*;
                let mut obj = obj;
                let mut id = id.to_owned();

                loop {
                    push_obj_count_unique(&mut out, seen_objs, &id, &obj, progress, stats, false);
                    match obj.kind {
                        Tree | Blob => break,
                        Tag => {
                            id = immutable::TagRefIter::from_bytes(obj.data)
                                .target_id()
                                .expect("every tag has a target");
                            obj = db.find_existing(id, buf1, cache)?;
                            stats.expanded_objects += 1;
                            continue;
                        }
                        Commit => {
                            let current_tree_iter = {
                                let mut commit_iter = immutable::CommitRefIter::from_bytes(obj.data);
                                let tree_id = commit_iter.tree_id().expect("every commit has a tree");
                                parent_commit_ids.clear();
                                for token in commit_iter {
                                    match token {
                                        Ok(immutable::commit::iter::Token::Parent { id }) => parent_commit_ids.push(id),
                                        Ok(_) => break,
                                        Err(err) => return Err(Error::CommitDecode(err)),
                                    }
                                }
                                let obj = db.find_existing(tree_id, buf1, cache)?;
                                push_obj_count_unique(&mut out, seen_objs, &tree_id, &obj, progress, stats, true);
                                immutable::TreeIter::from_bytes(obj.data)
                            };

                            let objects = if parent_commit_ids.is_empty() {
                                traverse_delegate.clear();
                                git_traverse::tree::breadthfirst(
                                    current_tree_iter,
                                    &mut tree_traversal_state,
                                    |oid, buf| {
                                        stats.decoded_objects += 1;
                                        match db.find_existing(oid, buf, cache).ok() {
                                            Some(obj) => {
                                                progress.inc();
                                                stats.expanded_objects += 1;
                                                out.push(output::Count::from_data(oid, &obj));
                                                obj.into_tree_iter()
                                            }
                                            None => None,
                                        }
                                    },
                                    &mut traverse_delegate,
                                )
                                .map_err(Error::TreeTraverse)?;
                                &traverse_delegate.non_trees
                            } else {
                                for commit_id in &parent_commit_ids {
                                    let parent_tree_id = {
                                        let parent_commit_obj = db.find_existing(commit_id, buf2, cache)?;

                                        push_obj_count_unique(
                                            &mut out,
                                            seen_objs,
                                            commit_id,
                                            &parent_commit_obj,
                                            progress,
                                            stats,
                                            true,
                                        );
                                        immutable::CommitRefIter::from_bytes(parent_commit_obj.data)
                                            .tree_id()
                                            .expect("every commit has a tree")
                                    };
                                    let parent_tree = {
                                        let parent_tree_obj = db.find_existing(parent_tree_id, buf2, cache)?;
                                        push_obj_count_unique(
                                            &mut out,
                                            seen_objs,
                                            &parent_tree_id,
                                            &parent_tree_obj,
                                            progress,
                                            stats,
                                            true,
                                        );
                                        immutable::TreeIter::from_bytes(parent_tree_obj.data)
                                    };

                                    changes_delegate.clear();
                                    git_diff::tree::Changes::from(Some(parent_tree))
                                        .needed_to_obtain(
                                            current_tree_iter.clone(),
                                            &mut tree_diff_state,
                                            |oid, buf| {
                                                stats.decoded_objects += 1;
                                                db.find_existing_tree_iter(oid, buf, cache).ok()
                                            },
                                            &mut changes_delegate,
                                        )
                                        .map_err(Error::TreeChanges)?;
                                }
                                &changes_delegate.objects
                            };
                            for id in objects.iter() {
                                out.push(id_to_count(db, buf2, id, progress, stats, allow_pack_lookups));
                            }
                            break;
                        }
                    }
                }
            }
            TreeContents => {
                use git_object::Kind::*;
                let mut id = id;
                let mut obj = obj;
                loop {
                    push_obj_count_unique(&mut out, seen_objs, &id, &obj, progress, stats, false);
                    match obj.kind {
                        Tree => {
                            traverse_delegate.clear();
                            git_traverse::tree::breadthfirst(
                                git_object::immutable::TreeIter::from_bytes(obj.data),
                                &mut tree_traversal_state,
                                |oid, buf| {
                                    stats.decoded_objects += 1;
                                    match db.find_existing(oid, buf, cache).ok() {
                                        Some(obj) => {
                                            progress.inc();
                                            stats.expanded_objects += 1;
                                            out.push(output::Count::from_data(oid, &obj));
                                            obj.into_tree_iter()
                                        }
                                        None => None,
                                    }
                                },
                                &mut traverse_delegate,
                            )
                            .map_err(Error::TreeTraverse)?;
                            for id in traverse_delegate.non_trees.iter() {
                                out.push(id_to_count(db, buf1, id, progress, stats, allow_pack_lookups));
                            }
                            break;
                        }
                        Commit => {
                            id = immutable::CommitRefIter::from_bytes(obj.data)
                                .tree_id()
                                .expect("every commit has a tree");
                            stats.expanded_objects += 1;
                            obj = db.find_existing(id, buf1, cache)?;
                            continue;
                        }
                        Blob => break,
                        Tag => {
                            id = immutable::TagRefIter::from_bytes(obj.data)
                                .target_id()
                                .expect("every tag has a target");
                            stats.expanded_objects += 1;
                            obj = db.find_existing(id, buf1, cache)?;
                            continue;
                        }
                    }
                }
            }
            AsIs => push_obj_count_unique(&mut out, seen_objs, &id, &obj, progress, stats, false),
        }
    }
    Ok((out, outcome))
}

mod tree {
    pub mod changes {
        use git_diff::tree::{
            visit::{Action, Change},
            Visit,
        };
        use git_hash::ObjectId;
        use git_object::bstr::BStr;

        use crate::data::output::count::objects::util::InsertImmutable;

        pub struct AllNew<'a, H> {
            pub objects: Vec<ObjectId>,
            all_seen: &'a H,
        }

        impl<'a, H> AllNew<'a, H>
        where
            H: InsertImmutable<ObjectId>,
        {
            pub fn new(all_seen: &'a H) -> Self {
                AllNew {
                    objects: Default::default(),
                    all_seen,
                }
            }
            pub fn clear(&mut self) {
                self.objects.clear();
            }
        }

        impl<'a, H> Visit for AllNew<'a, H>
        where
            H: InsertImmutable<ObjectId>,
        {
            fn pop_front_tracked_path_and_set_current(&mut self) {}

            fn push_back_tracked_path_component(&mut self, _component: &BStr) {}

            fn push_path_component(&mut self, _component: &BStr) {}

            fn pop_path_component(&mut self) {}

            fn visit(&mut self, change: Change) -> Action {
                match change {
                    Change::Addition { oid, .. } | Change::Modification { oid, .. } => {
                        let inserted = self.all_seen.insert(oid);
                        if inserted {
                            self.objects.push(oid);
                        }
                    }
                    Change::Deletion { .. } => {}
                };
                Action::Continue
            }
        }
    }

    pub mod traverse {
        use git_hash::ObjectId;
        use git_object::{bstr::BStr, immutable::tree::Entry};
        use git_traverse::tree::visit::{Action, Visit};

        use crate::data::output::count::objects::util::InsertImmutable;

        pub struct AllUnseen<'a, H> {
            pub non_trees: Vec<ObjectId>,
            all_seen: &'a H,
        }

        impl<'a, H> AllUnseen<'a, H>
        where
            H: InsertImmutable<ObjectId>,
        {
            pub fn new(all_seen: &'a H) -> Self {
                AllUnseen {
                    non_trees: Default::default(),
                    all_seen,
                }
            }
            pub fn clear(&mut self) {
                self.non_trees.clear();
            }
        }

        impl<'a, H> Visit for AllUnseen<'a, H>
        where
            H: InsertImmutable<ObjectId>,
        {
            fn pop_front_tracked_path_and_set_current(&mut self) {}

            fn push_back_tracked_path_component(&mut self, _component: &BStr) {}

            fn push_path_component(&mut self, _component: &BStr) {}

            fn pop_path_component(&mut self) {}

            fn visit_tree(&mut self, entry: &Entry<'_>) -> Action {
                let inserted = self.all_seen.insert(entry.oid.to_owned());
                if inserted {
                    Action::Continue
                } else {
                    Action::Skip
                }
            }

            fn visit_nontree(&mut self, entry: &Entry<'_>) -> Action {
                let inserted = self.all_seen.insert(entry.oid.to_owned());
                if inserted {
                    self.non_trees.push(entry.oid.to_owned());
                }
                Action::Continue
            }
        }
    }
}

#[inline]
fn push_obj_count_unique(
    out: &mut Vec<output::Count>,
    all_seen: &impl util::InsertImmutable<ObjectId>,
    id: &oid,
    obj: &crate::data::Object<'_>,
    progress: &mut impl Progress,
    statistics: &mut Outcome,
    count_expanded: bool,
) {
    let inserted = all_seen.insert(id.to_owned());
    if inserted {
        progress.inc();
        statistics.decoded_objects += 1;
        if count_expanded {
            statistics.expanded_objects += 1;
        }
        out.push(output::Count::from_data(id, obj));
    }
}

#[inline]
fn id_to_count<Find: crate::Find>(
    db: &Find,
    buf: &mut Vec<u8>,
    id: &oid,
    progress: &mut impl Progress,
    statistics: &mut Outcome,
    allow_pack_lookups: bool,
) -> output::Count {
    progress.inc();
    statistics.expanded_objects += 1;
    output::Count {
        id: id.to_owned(),
        entry_pack_location: if allow_pack_lookups {
            PackLocation::LookedUp(db.location_by_oid(id, buf))
        } else {
            PackLocation::NotLookedUp
        },
    }
}

mod util {
    pub trait InsertImmutable<Item: Eq + std::hash::Hash> {
        fn insert(&self, item: Item) -> bool;
    }

    mod trait_impls {
        use std::{cell::RefCell, collections::HashSet, hash::Hash};

        use dashmap::DashSet;

        use super::InsertImmutable;

        impl<T: Eq + Hash> InsertImmutable<T> for DashSet<T> {
            fn insert(&self, item: T) -> bool {
                self.insert(item)
            }
        }

        impl<T: Eq + Hash> InsertImmutable<T> for RefCell<HashSet<T>> {
            fn insert(&self, item: T) -> bool {
                self.borrow_mut().insert(item)
            }
        }
    }

    pub struct Chunks<I> {
        pub size: usize,
        pub iter: I,
    }

    impl<I, Item> Iterator for Chunks<I>
    where
        I: Iterator<Item = Item>,
    {
        type Item = Vec<Item>;

        fn next(&mut self) -> Option<Self::Item> {
            let mut res = Vec::with_capacity(self.size);
            let mut items_left = self.size;
            for item in &mut self.iter {
                res.push(item);
                items_left -= 1;
                if items_left == 0 {
                    break;
                }
            }
            (!res.is_empty()).then(|| res)
        }
    }
}

mod types {
    /// Information gathered during the run of [`iter_from_objects()`][super::objects()].
    #[derive(Default, PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
    #[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
    pub struct Outcome {
        /// The amount of objects provided to start the iteration.
        pub input_objects: usize,
        /// The amount of objects that have been expanded from the input source.
        /// It's desirable to do that as expansion happens on multiple threads, allowing the amount of input objects to be small.
        /// `expanded_objects - decoded_objects` is the 'cheap' object we found without decoding the object itself.
        pub expanded_objects: usize,
        /// The amount of fully decoded objects. These are the most expensive as they are fully decoded
        pub decoded_objects: usize,
        /// The total amount of encountered objects. Should be `expanded_objects + input_objects`.
        pub total_objects: usize,
    }

    impl Outcome {
        pub(in crate::data::output::count) fn aggregate(
            &mut self,
            Outcome {
                input_objects,
                decoded_objects,
                expanded_objects,
                total_objects,
            }: Self,
        ) {
            self.input_objects += input_objects;
            self.decoded_objects += decoded_objects;
            self.expanded_objects += expanded_objects;
            self.total_objects += total_objects;
        }
    }

    /// The way input objects are handled
    #[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
    #[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
    pub enum ObjectExpansion {
        /// Don't do anything with the input objects except for transforming them into pack entries
        AsIs,
        /// If the input object is a Commit then turn it into a pack entry. Additionally obtain its tree, turn it into a pack entry
        /// along with all of its contents, that is nested trees, and any other objects reachable from it.
        /// Otherwise, the same as [`AsIs`][ObjectExpansion::AsIs].
        ///
        /// This mode is useful if all reachable objects should be added, as in cloning a repository.
        TreeContents,
        /// If the input is a commit, obtain its ancestors and turn them into pack entries. Obtain the ancestor trees along with the commits
        /// tree and turn them into pack entries. Finally obtain the added/changed objects when comparing the ancestor trees with the
        /// current tree and turn them into entries as well.
        /// Otherwise, the same as [`AsIs`][ObjectExpansion::AsIs].
        ///
        /// This mode is useful to build a pack containing only new objects compared to a previous state.
        TreeAdditionsComparedToAncestor,
    }

    impl Default for ObjectExpansion {
        fn default() -> Self {
            ObjectExpansion::AsIs
        }
    }

    /// Configuration options for the pack generation functions provied in [this module][crate::data::output].
    #[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
    #[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
    pub struct Options {
        /// The amount of threads to use at most when resolving the pack. If `None`, all logical cores are used.
        /// If more than one thread is used, the order of returned [counts][crate::data::output::Count] is not deterministic anymore
        /// especially when tree traversal is involved. Thus deterministic ordering requires `Some(1)` to be set.
        pub thread_limit: Option<usize>,
        /// The amount of objects per chunk or unit of work to be sent to threads for processing
        pub chunk_size: usize,
        /// The way input objects are handled
        pub input_object_expansion: ObjectExpansion,
    }

    impl Default for Options {
        fn default() -> Self {
            Options {
                thread_limit: None,
                chunk_size: 10,
                input_object_expansion: Default::default(),
            }
        }
    }

    /// The error returned by the pack generation iterator [bytes::FromEntriesIter][crate::data::output::bytes::FromEntriesIter].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error<FindErr, IterErr>
    where
        FindErr: std::error::Error + 'static,
        IterErr: std::error::Error + 'static,
    {
        #[error(transparent)]
        CommitDecode(git_object::immutable::object::decode::Error),
        #[error(transparent)]
        FindExisting(#[from] FindErr),
        #[error(transparent)]
        InputIteration(IterErr),
        #[error(transparent)]
        TreeTraverse(git_traverse::tree::breadthfirst::Error),
        #[error(transparent)]
        TreeChanges(git_diff::tree::changes::Error),
        #[error("Operation interrupted")]
        Interrupted,
    }
}
use std::{cell::RefCell, collections::HashSet};

pub use types::{Error, ObjectExpansion, Options, Outcome};

use crate::data::output::count::PackLocation;

mod reduce {
    use std::{marker::PhantomData, sync::Arc};

    use git_features::{parallel, progress::Progress};

    use super::Outcome;
    use crate::data::output;

    pub struct Statistics<E, P> {
        total: Outcome,
        counts: Vec<output::Count>,
        progress: Arc<parking_lot::Mutex<P>>,
        _err: PhantomData<E>,
    }

    impl<E, P> Statistics<E, P>
    where
        P: Progress,
    {
        pub fn new(progress: Arc<parking_lot::Mutex<P>>) -> Self {
            Statistics {
                total: Default::default(),
                counts: Default::default(),
                progress,
                _err: PhantomData::default(),
            }
        }
    }

    impl<E, P> parallel::Reduce for Statistics<E, P>
    where
        P: Progress,
    {
        type Input = Result<(Vec<output::Count>, Outcome), E>;
        type FeedProduce = ();
        type Output = (Vec<output::Count>, Outcome);
        type Error = E;

        fn feed(&mut self, item: Self::Input) -> Result<Self::FeedProduce, Self::Error> {
            let (counts, mut stats) = item?;
            stats.total_objects = counts.len();
            self.total.aggregate(stats);
            self.progress.lock().inc_by(counts.len());
            self.counts.extend(counts);
            Ok(())
        }

        fn finalize(self) -> Result<Self::Output, Self::Error> {
            Ok((self.counts, self.total))
        }
    }
}
