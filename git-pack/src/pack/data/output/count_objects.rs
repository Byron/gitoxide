use crate::{find, pack, pack::data::output, FindExt};
use dashmap::DashSet;
use git_features::{parallel, progress::Progress};
use git_hash::{oid, ObjectId};
use git_object::immutable;
use std::sync::Arc;

/// Generate [`Entries`][output::Count] from input `objects` into `out` without attempting to apply any delta compression.
/// TODO: update based on new counting mechanism
/// TODO: Don't decode traversal all objects, provide info only if you have it. Massive duplication right now.
///
/// * `objects`
///   * A list of objects to add to the pack. Duplication checks are performed so no object is ever added to a pack twice.
/// * `progress`
///   * a way to obtain progress information
/// * `options`
///   * more configuration
pub fn count_objects_iter<Find, Iter, Oid, Cache>(
    db: Find,
    make_cache: impl Fn() -> Cache + Send + Clone + Sync + 'static,
    objects: Iter,
    progress: impl Progress,
    Options {
        thread_limit,
        input_object_expansion,
        chunk_size,
    }: Options,
) -> impl Iterator<Item = Result<Vec<output::Count>, Error<find::existing::Error<Find::Error>>>>
       + parallel::reduce::Finalize<
    Reduce = parallel::reduce::IdentityWithResult<Vec<output::Count>, Error<find::existing::Error<Find::Error>>>,
>
where
    Find: crate::Find + Clone + Send + Sync + 'static,
    <Find as crate::Find>::Error: Send,
    Iter: Iterator<Item = Oid> + Send + 'static,
    Oid: AsRef<oid> + Send + 'static,
    Cache: pack::cache::DecodeEntry,
{
    let lower_bound = objects.size_hint().0;
    let (chunk_size, thread_limit, _) = parallel::optimize_chunk_size_and_thread_limit(
        chunk_size,
        if lower_bound == 0 { None } else { Some(lower_bound) },
        thread_limit,
        None,
    );
    let chunks = util::Chunks {
        iter: objects,
        size: chunk_size,
    };
    let seen_objs = Arc::new(dashmap::DashSet::<ObjectId>::new());
    let progress = Arc::new(parking_lot::Mutex::new(progress));

    parallel::reduce::Stepwise::new(
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
            let seen_objs = Arc::clone(&seen_objs);
            move |oids: Vec<Oid>, (buf1, buf2, cache, progress)| {
                use ObjectExpansion::*;
                let mut out = Vec::new();
                let mut tree_traversal_state = git_traverse::tree::breadthfirst::State::default();
                let mut tree_diff_state = git_diff::tree::State::default();
                let mut parent_commit_ids = Vec::new();
                let seen_objs = seen_objs.as_ref();
                let mut traverse_delegate = tree::traverse::AllUnseen::new(seen_objs);
                let mut changes_delegate = tree::changes::AllNew::new(seen_objs);

                for id in oids.into_iter() {
                    let id = id.as_ref();
                    let obj = db.find_existing(id, buf1, cache)?;
                    match input_object_expansion {
                        TreeAdditionsComparedToAncestor => {
                            use git_object::Kind::*;
                            let mut obj = obj;
                            let mut id = id.to_owned();

                            loop {
                                push_obj_count_unique(&mut out, seen_objs, &id, &obj, progress);
                                match obj.kind {
                                    Tree | Blob => break,
                                    Tag => {
                                        id = immutable::TagIter::from_bytes(obj.data)
                                            .target_id()
                                            .expect("every tag has a target");
                                        obj = db.find_existing(id, buf1, cache)?;
                                        continue;
                                    }
                                    Commit => {
                                        let current_tree_iter = {
                                            let mut commit_iter = immutable::CommitIter::from_bytes(obj.data);
                                            let tree_id = commit_iter.tree_id().expect("every commit has a tree");
                                            parent_commit_ids.clear();
                                            for token in commit_iter {
                                                match token {
                                                    Ok(immutable::commit::iter::Token::Parent { id }) => {
                                                        parent_commit_ids.push(id)
                                                    }
                                                    Ok(_) => break,
                                                    Err(err) => return Err(Error::CommitDecode(err)),
                                                }
                                            }
                                            let obj = db.find_existing(tree_id, buf1, cache)?;
                                            push_obj_count_unique(&mut out, seen_objs, &tree_id, &obj, progress);
                                            immutable::TreeIter::from_bytes(obj.data)
                                        };

                                        let objects = if parent_commit_ids.is_empty() {
                                            traverse_delegate.clear();
                                            git_traverse::tree::breadthfirst(
                                                current_tree_iter,
                                                &mut tree_traversal_state,
                                                |oid, buf| db.find_existing_tree_iter(oid, buf, cache).ok(),
                                                &mut traverse_delegate,
                                            )
                                            .map_err(Error::TreeTraverse)?;
                                            &traverse_delegate.objects
                                        } else {
                                            changes_delegate.clear();
                                            for commit_id in &parent_commit_ids {
                                                let parent_tree_id = {
                                                    let parent_commit_obj = db.find_existing(commit_id, buf2, cache)?;

                                                    push_obj_count_unique(
                                                        &mut out,
                                                        seen_objs,
                                                        &commit_id,
                                                        &parent_commit_obj,
                                                        progress,
                                                    );
                                                    immutable::CommitIter::from_bytes(parent_commit_obj.data)
                                                        .tree_id()
                                                        .expect("every commit has a tree")
                                                };
                                                let parent_tree = {
                                                    let parent_tree_obj =
                                                        db.find_existing(parent_tree_id, buf2, cache)?;
                                                    push_obj_count_unique(
                                                        &mut out,
                                                        seen_objs,
                                                        &parent_tree_id,
                                                        &parent_tree_obj,
                                                        progress,
                                                    );
                                                    immutable::TreeIter::from_bytes(parent_tree_obj.data)
                                                };

                                                git_diff::tree::Changes::from(Some(parent_tree))
                                                    .needed_to_obtain(
                                                        current_tree_iter.clone(),
                                                        &mut tree_diff_state,
                                                        |oid, buf| db.find_existing_tree_iter(oid, buf, cache).ok(),
                                                        &mut changes_delegate,
                                                    )
                                                    .map_err(Error::TreeChanges)?;
                                            }
                                            &changes_delegate.objects
                                        };
                                        for id in objects.iter() {
                                            out.push(id_to_count(&db, buf2, id, progress));
                                        }
                                        break;
                                    }
                                }
                            }
                        }
                        TreeContents => {
                            use git_object::Kind::*;
                            let mut id: ObjectId = id.into();
                            let mut obj = obj;
                            loop {
                                push_obj_count_unique(&mut out, seen_objs, &id, &obj, progress);
                                match obj.kind {
                                    Tree => {
                                        traverse_delegate.clear();
                                        git_traverse::tree::breadthfirst(
                                            git_object::immutable::TreeIter::from_bytes(obj.data),
                                            &mut tree_traversal_state,
                                            |oid, buf| db.find_existing_tree_iter(oid, buf, cache).ok(),
                                            &mut traverse_delegate,
                                        )
                                        .map_err(Error::TreeTraverse)?;
                                        for id in traverse_delegate.objects.iter() {
                                            out.push(id_to_count(&db, buf1, id, progress));
                                        }
                                        break;
                                    }
                                    Commit => {
                                        id = immutable::CommitIter::from_bytes(obj.data)
                                            .tree_id()
                                            .expect("every commit has a tree");
                                        obj = db.find_existing(id, buf1, cache)?;
                                        continue;
                                    }
                                    Blob => break,
                                    Tag => {
                                        id = immutable::TagIter::from_bytes(obj.data)
                                            .target_id()
                                            .expect("every tag has a target");
                                        obj = db.find_existing(id, buf1, cache)?;
                                        continue;
                                    }
                                }
                            }
                        }
                        AsIs => push_obj_count_unique(&mut out, seen_objs, id, &obj, progress),
                    }
                }
                Ok(out)
            }
        },
        parallel::reduce::IdentityWithResult::default(),
    )
}

mod tree {
    pub mod changes {
        use dashmap::DashSet;
        use git_diff::tree::{
            visit::{Action, Change},
            Visit,
        };
        use git_hash::{bstr::BStr, ObjectId};
        use std::collections::HashSet;

        pub struct AllNew<'a> {
            pub objects: HashSet<ObjectId>,
            all_seen: &'a DashSet<ObjectId>,
        }

        impl<'a> AllNew<'a> {
            pub fn new(all_seen: &'a DashSet<ObjectId>) -> Self {
                AllNew {
                    objects: Default::default(),
                    all_seen,
                }
            }
            pub fn clear(&mut self) {
                self.objects.clear();
            }
        }

        impl<'a> Visit for AllNew<'a> {
            fn pop_front_tracked_path_and_set_current(&mut self) {}

            fn push_back_tracked_path_component(&mut self, _component: &BStr) {}

            fn push_path_component(&mut self, _component: &BStr) {}

            fn pop_path_component(&mut self) {}

            fn visit(&mut self, change: Change) -> Action {
                match change {
                    Change::Addition { oid, .. } | Change::Modification { oid, .. } => {
                        let inserted = self.all_seen.insert(oid);
                        if inserted {
                            self.objects.insert(oid);
                        }
                    }
                    Change::Deletion { .. } => {}
                };
                Action::Continue
            }
        }
    }

    pub mod traverse {
        use dashmap::DashSet;
        use git_hash::{bstr::BStr, ObjectId};
        use git_object::immutable::tree::Entry;
        use git_traverse::tree::visit::{Action, Visit};
        use std::collections::HashSet;

        pub struct AllUnseen<'a> {
            pub objects: HashSet<ObjectId>,
            all_seen: &'a DashSet<ObjectId>,
        }

        impl<'a> AllUnseen<'a> {
            pub fn new(all_seen: &'a DashSet<ObjectId>) -> Self {
                AllUnseen {
                    objects: Default::default(),
                    all_seen,
                }
            }
            pub fn clear(&mut self) {
                self.objects.clear();
            }
        }

        impl<'a> Visit for AllUnseen<'a> {
            fn pop_front_tracked_path_and_set_current(&mut self) {}

            fn push_back_tracked_path_component(&mut self, _component: &BStr) {}

            fn push_path_component(&mut self, _component: &BStr) {}

            fn pop_path_component(&mut self) {}

            fn visit_tree(&mut self, entry: &Entry<'_>) -> Action {
                let inserted = self.all_seen.insert(entry.oid.to_owned());
                if inserted {
                    self.objects.insert(entry.oid.to_owned());
                    Action::Continue
                } else {
                    Action::Skip
                }
            }

            fn visit_nontree(&mut self, entry: &Entry<'_>) -> Action {
                let inserted = self.all_seen.insert(entry.oid.to_owned());
                if inserted {
                    self.objects.insert(entry.oid.to_owned());
                }
                Action::Continue
            }
        }
    }
}

fn push_obj_count_unique(
    out: &mut Vec<output::Count>,
    all_seen: &DashSet<ObjectId>,
    id: &oid,
    obj: &crate::data::Object<'_>,
    progress: &mut impl Progress,
) {
    let inserted = all_seen.insert(id.to_owned());
    if inserted {
        progress.inc();
        out.push(output::Count::from_data(id, &obj));
    }
}

fn id_to_count<Find: crate::Find>(
    db: &Find,
    buf: &mut Vec<u8>,
    id: &oid,
    progress: &mut impl Progress,
) -> output::Count {
    progress.inc();
    output::Count {
        id: id.to_owned(),
        entry_pack_location: db.location_by_id(id, buf),
    }
}

mod util {
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
            while let Some(item) = self.iter.next() {
                res.push(item);
                items_left -= 1;
                if items_left == 0 {
                    break;
                }
            }
            if res.is_empty() {
                None
            } else {
                Some(res)
            }
        }
    }
}

mod types {
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

    /// Configuration options for the pack generation functions provied in [this module][crate::pack::data::output].
    #[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
    #[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
    pub struct Options {
        /// The amount of threads to use at most when resolving the pack. If `None`, all logical cores are used.
        pub thread_limit: Option<usize>,
        /// The amount of objects per chunk or unit of work to be sent to threads for processing
        /// TODO: could this become the window size?
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

    /// The error returned by the pack generation function [`to_entry_iter()`][crate::pack::data::output::objects_to_entries_iter()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error<FindErr>
    where
        FindErr: std::error::Error + 'static,
    {
        #[error(transparent)]
        CommitDecode(git_object::immutable::object::decode::Error),
        #[error(transparent)]
        FindExisting(#[from] FindErr),
        #[error(transparent)]
        TreeTraverse(git_traverse::tree::breadthfirst::Error),
        #[error(transparent)]
        TreeChanges(git_diff::tree::changes::Error),
    }
}
pub use types::{Error, ObjectExpansion, Options};
