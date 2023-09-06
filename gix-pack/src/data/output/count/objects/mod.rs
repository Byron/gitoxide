use std::{cell::RefCell, sync::atomic::AtomicBool};

use gix_features::parallel;
use gix_hash::ObjectId;

use crate::data::output;

pub(in crate::data::output::count::objects_impl) mod reduce;
mod util;

mod types;
pub use types::{Error, ObjectExpansion, Options, Outcome};

mod tree;

/// Generate [`Count`][output::Count]s from input `objects` with object expansion based on [`options`][Options]
/// to learn which objects would would constitute a pack. This step is required to know exactly how many objects would
/// be in a pack while keeping data around to avoid minimize object database access.
///
/// A [`Count`][output::Count] object maintains enough state to greatly accelerate future access of packed objects.
///
/// * `db` - the object store to use for accessing objects.
/// * `objects_ids`
///   * A list of objects ids to add to the pack. Duplication checks are performed so no object is ever added to a pack twice.
///   * Objects may be expanded based on the provided [`options`][Options]
/// * `objects`
///   * count the amount of objects we encounter
/// * `should_interrupt`
///  * A flag that is set to true if the operation should stop
/// * `options`
///   * more configuration
pub fn objects<Find>(
    db: Find,
    objects_ids: Box<dyn Iterator<Item = Result<ObjectId, Box<dyn std::error::Error + Send + Sync + 'static>>> + Send>,
    objects: &dyn gix_features::progress::Count,
    should_interrupt: &AtomicBool,
    Options {
        thread_limit,
        input_object_expansion,
        chunk_size,
    }: Options,
) -> Result<(Vec<output::Count>, Outcome), Error>
where
    Find: crate::Find + Send + Clone,
{
    let lower_bound = objects_ids.size_hint().0;
    let (chunk_size, thread_limit, _) = parallel::optimize_chunk_size_and_thread_limit(
        chunk_size,
        if lower_bound == 0 { None } else { Some(lower_bound) },
        thread_limit,
        None,
    );
    let chunks = gix_features::iter::Chunks {
        inner: objects_ids,
        size: chunk_size,
    };
    let seen_objs = gix_hashtable::sync::ObjectIdMap::default();
    let objects = objects.counter();

    parallel::in_parallel(
        chunks,
        thread_limit,
        {
            move |_| {
                (
                    Vec::new(), // object data buffer
                    Vec::new(), // object data buffer 2 to hold two objects at a time
                    objects.clone(),
                )
            }
        },
        {
            let seen_objs = &seen_objs;
            move |oids: Vec<_>, (buf1, buf2, objects)| {
                expand::this(
                    &db,
                    input_object_expansion,
                    seen_objs,
                    &mut oids.into_iter(),
                    buf1,
                    buf2,
                    objects,
                    should_interrupt,
                    true, /*allow pack lookups*/
                )
            }
        },
        reduce::Statistics::new(),
    )
}

/// Like [`objects()`] but using a single thread only to mostly save on the otherwise required overhead.
pub fn objects_unthreaded(
    db: &dyn crate::Find,
    object_ids: &mut dyn Iterator<Item = Result<ObjectId, Box<dyn std::error::Error + Send + Sync + 'static>>>,
    objects: &dyn gix_features::progress::Count,
    should_interrupt: &AtomicBool,
    input_object_expansion: ObjectExpansion,
) -> Result<(Vec<output::Count>, Outcome), Error> {
    let seen_objs = RefCell::new(gix_hashtable::HashSet::default());

    let (mut buf1, mut buf2) = (Vec::new(), Vec::new());
    expand::this(
        db,
        input_object_expansion,
        &seen_objs,
        object_ids,
        &mut buf1,
        &mut buf2,
        &objects.counter(),
        should_interrupt,
        false, /*allow pack lookups*/
    )
}

mod expand {
    use std::sync::atomic::{AtomicBool, Ordering};

    use gix_hash::{oid, ObjectId};
    use gix_object::{CommitRefIter, TagRefIter};

    use super::{
        tree,
        types::{Error, ObjectExpansion, Outcome},
        util,
    };
    use crate::{
        data::{output, output::count::PackLocation},
        FindExt,
    };

    #[allow(clippy::too_many_arguments)]
    pub fn this(
        db: &dyn crate::Find,
        input_object_expansion: ObjectExpansion,
        seen_objs: &impl util::InsertImmutable,
        oids: &mut dyn Iterator<Item = Result<ObjectId, Box<dyn std::error::Error + Send + Sync + 'static>>>,
        buf1: &mut Vec<u8>,
        #[allow(clippy::ptr_arg)] buf2: &mut Vec<u8>,
        objects: &gix_features::progress::AtomicStep,
        should_interrupt: &AtomicBool,
        allow_pack_lookups: bool,
    ) -> Result<(Vec<output::Count>, Outcome), Error> {
        use ObjectExpansion::*;

        let mut out = Vec::new();
        let mut tree_traversal_state = gix_traverse::tree::breadthfirst::State::default();
        let mut tree_diff_state = gix_diff::tree::State::default();
        let mut parent_commit_ids = Vec::new();
        let mut traverse_delegate = tree::traverse::AllUnseen::new(seen_objs);
        let mut changes_delegate = tree::changes::AllNew::new(seen_objs);
        let mut outcome = Outcome::default();

        let stats = &mut outcome;
        for id in oids {
            if should_interrupt.load(Ordering::Relaxed) {
                return Err(Error::Interrupted);
            }

            let id = id.map_err(Error::InputIteration)?;
            let (obj, location) = db.find(&id, buf1)?;
            stats.input_objects += 1;
            match input_object_expansion {
                TreeAdditionsComparedToAncestor => {
                    use gix_object::Kind::*;
                    let mut obj = obj;
                    let mut location = location;
                    let mut id = id.to_owned();

                    loop {
                        push_obj_count_unique(&mut out, seen_objs, &id, location, objects, stats, false);
                        match obj.kind {
                            Tree | Blob => break,
                            Tag => {
                                id = TagRefIter::from_bytes(obj.data)
                                    .target_id()
                                    .expect("every tag has a target");
                                let tmp = db.find(&id, buf1)?;

                                obj = tmp.0;
                                location = tmp.1;

                                stats.expanded_objects += 1;
                                continue;
                            }
                            Commit => {
                                let current_tree_iter = {
                                    let mut commit_iter = CommitRefIter::from_bytes(obj.data);
                                    let tree_id = commit_iter.tree_id().expect("every commit has a tree");
                                    parent_commit_ids.clear();
                                    for token in commit_iter {
                                        match token {
                                            Ok(gix_object::commit::ref_iter::Token::Parent { id }) => {
                                                parent_commit_ids.push(id)
                                            }
                                            Ok(_) => break,
                                            Err(err) => return Err(Error::CommitDecode(err)),
                                        }
                                    }
                                    let (obj, location) = db.find(&tree_id, buf1)?;
                                    push_obj_count_unique(
                                        &mut out, seen_objs, &tree_id, location, objects, stats, true,
                                    );
                                    gix_object::TreeRefIter::from_bytes(obj.data)
                                };

                                let objects_ref = if parent_commit_ids.is_empty() {
                                    traverse_delegate.clear();
                                    gix_traverse::tree::breadthfirst(
                                        current_tree_iter,
                                        &mut tree_traversal_state,
                                        |oid, buf| {
                                            stats.decoded_objects += 1;
                                            match db.find(oid, buf).ok() {
                                                Some((obj, location)) => {
                                                    objects.fetch_add(1, Ordering::Relaxed);
                                                    stats.expanded_objects += 1;
                                                    out.push(output::Count::from_data(oid, location));
                                                    obj.try_into_tree_iter()
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
                                            let (parent_commit_obj, location) = db.find(commit_id, buf2)?;

                                            push_obj_count_unique(
                                                &mut out, seen_objs, commit_id, location, objects, stats, true,
                                            );
                                            CommitRefIter::from_bytes(parent_commit_obj.data)
                                                .tree_id()
                                                .expect("every commit has a tree")
                                        };
                                        let parent_tree = {
                                            let (parent_tree_obj, location) = db.find(&parent_tree_id, buf2)?;
                                            push_obj_count_unique(
                                                &mut out,
                                                seen_objs,
                                                &parent_tree_id,
                                                location,
                                                objects,
                                                stats,
                                                true,
                                            );
                                            gix_object::TreeRefIter::from_bytes(parent_tree_obj.data)
                                        };

                                        changes_delegate.clear();
                                        gix_diff::tree::Changes::from(Some(parent_tree))
                                            .needed_to_obtain(
                                                current_tree_iter.clone(),
                                                &mut tree_diff_state,
                                                |oid, buf| {
                                                    stats.decoded_objects += 1;
                                                    db.find_tree_iter(oid, buf).map(|t| t.0)
                                                },
                                                &mut changes_delegate,
                                            )
                                            .map_err(Error::TreeChanges)?;
                                    }
                                    &changes_delegate.objects
                                };
                                for id in objects_ref.iter() {
                                    out.push(id_to_count(db, buf2, id, objects, stats, allow_pack_lookups));
                                }
                                break;
                            }
                        }
                    }
                }
                TreeContents => {
                    use gix_object::Kind::*;
                    let mut id = id;
                    let mut obj = (obj, location);
                    loop {
                        push_obj_count_unique(&mut out, seen_objs, &id, obj.1.clone(), objects, stats, false);
                        match obj.0.kind {
                            Tree => {
                                traverse_delegate.clear();
                                gix_traverse::tree::breadthfirst(
                                    gix_object::TreeRefIter::from_bytes(obj.0.data),
                                    &mut tree_traversal_state,
                                    |oid, buf| {
                                        stats.decoded_objects += 1;
                                        match db.find(oid, buf).ok() {
                                            Some((obj, location)) => {
                                                objects.fetch_add(1, Ordering::Relaxed);
                                                stats.expanded_objects += 1;
                                                out.push(output::Count::from_data(oid, location));
                                                obj.try_into_tree_iter()
                                            }
                                            None => None,
                                        }
                                    },
                                    &mut traverse_delegate,
                                )
                                .map_err(Error::TreeTraverse)?;
                                for id in &traverse_delegate.non_trees {
                                    out.push(id_to_count(db, buf1, id, objects, stats, allow_pack_lookups));
                                }
                                break;
                            }
                            Commit => {
                                id = CommitRefIter::from_bytes(obj.0.data)
                                    .tree_id()
                                    .expect("every commit has a tree");
                                stats.expanded_objects += 1;
                                obj = db.find(&id, buf1)?;
                                continue;
                            }
                            Blob => break,
                            Tag => {
                                id = TagRefIter::from_bytes(obj.0.data)
                                    .target_id()
                                    .expect("every tag has a target");
                                stats.expanded_objects += 1;
                                obj = db.find(&id, buf1)?;
                                continue;
                            }
                        }
                    }
                }
                AsIs => push_obj_count_unique(&mut out, seen_objs, &id, location, objects, stats, false),
            }
        }
        outcome.total_objects = out.len();
        Ok((out, outcome))
    }

    #[inline]
    fn push_obj_count_unique(
        out: &mut Vec<output::Count>,
        all_seen: &impl util::InsertImmutable,
        id: &oid,
        location: Option<crate::data::entry::Location>,
        objects: &gix_features::progress::AtomicStep,
        statistics: &mut Outcome,
        count_expanded: bool,
    ) {
        let inserted = all_seen.insert(id.to_owned());
        if inserted {
            objects.fetch_add(1, Ordering::Relaxed);
            statistics.decoded_objects += 1;
            if count_expanded {
                statistics.expanded_objects += 1;
            }
            out.push(output::Count::from_data(id, location));
        }
    }

    #[inline]
    fn id_to_count(
        db: &dyn crate::Find,
        buf: &mut Vec<u8>,
        id: &oid,
        objects: &gix_features::progress::AtomicStep,
        statistics: &mut Outcome,
        allow_pack_lookups: bool,
    ) -> output::Count {
        objects.fetch_add(1, Ordering::Relaxed);
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
}
