//! ### Deviation
//!
//! Note that the algorithm implemented here is in many ways different from what `git` does.
//!
//! - it's less sophisticated and doesn't use any ranking of candidates. Instead, it picks the first possible match.
//! - the set used for copy-detection is probably smaller by default.
use std::ops::Range;

use bstr::BStr;
use gix_object::tree::{EntryKind, EntryMode};

use crate::{
    blob::{platform::prepare_diff::Operation, DiffLineStats, ResourceKind},
    rewrites::{CopySource, Outcome, Tracker},
    Rewrites,
};

/// The kind of a change.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, PartialEq, Eq)]
pub enum ChangeKind {
    /// The change represents the *deletion* of an item.
    Deletion,
    /// The change represents the *modification* of an item.
    Modification,
    /// The change represents the *addition* of an item.
    Addition,
}

/// A trait providing all functionality to abstract over the concept of a change, as seen by the [`Tracker`].
pub trait Change: Clone {
    /// Return the hash of this change for identification.
    ///
    /// Note that this is the id of the object as stored in `git`, i.e. it must have gone through workspace
    /// conversions.
    fn id(&self) -> &gix_hash::oid;
    /// Return the kind of this change.
    fn kind(&self) -> ChangeKind;
    /// Return more information about the kind of entry affected by this change.
    fn entry_mode(&self) -> EntryMode;
    /// Return the id of the change along with its mode.
    fn id_and_entry_mode(&self) -> (&gix_hash::oid, EntryMode);
}

/// A set of tracked items allows to figure out their relations by figuring out their similarity.
pub(crate) struct Item<T> {
    /// The underlying raw change
    change: T,
    /// That slice into the backing for paths.
    path: Range<usize>,
    /// If true, this item was already emitted, i.e. seen by the caller.
    emitted: bool,
}

impl<T: Change> Item<T> {
    fn location<'a>(&self, backing: &'a [u8]) -> &'a BStr {
        backing[self.path.clone()].as_ref()
    }
    fn entry_mode_compatible(&self, mode: EntryMode) -> bool {
        use EntryKind::*;
        matches!(
            (mode.kind(), self.change.entry_mode().kind()),
            (Blob | BlobExecutable, Blob | BlobExecutable) | (Link, Link)
        )
    }

    fn is_source_for_destination_of(&self, kind: visit::SourceKind, dest_item_mode: EntryMode) -> bool {
        self.entry_mode_compatible(dest_item_mode)
            && match kind {
                visit::SourceKind::Rename => !self.emitted && matches!(self.change.kind(), ChangeKind::Deletion),
                visit::SourceKind::Copy => {
                    matches!(self.change.kind(), ChangeKind::Modification)
                }
            }
    }
}

/// A module with types used in the user-callback in [Tracker::emit()](crate::rewrites::Tracker::emit()).
pub mod visit {
    use bstr::BStr;
    use gix_object::tree::EntryMode;

    use crate::blob::DiffLineStats;

    /// The source of a rewrite, rename or copy.
    #[derive(Debug, Clone, PartialEq, PartialOrd)]
    pub struct Source<'a> {
        /// The kind of entry.
        pub entry_mode: EntryMode,
        /// The hash of the state of the source as seen in the object database.
        pub id: gix_hash::ObjectId,
        /// Further specify what kind of source this is.
        pub kind: SourceKind,
        /// The repository-relative location of this entry.
        pub location: &'a BStr,
        /// If this is a rewrite, indicate how many lines would need to change to turn this source into the destination.
        pub diff: Option<DiffLineStats>,
    }

    /// Further identify the kind of [Source].
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
    pub enum SourceKind {
        /// This is the source of an entry that was renamed, as `source` was renamed to `destination`.
        Rename,
        /// This is the source of a copy, as `source` was copied into `destination`.
        Copy,
    }

    /// A change along with a location.
    #[derive(Clone)]
    pub struct Destination<'a, T: Clone> {
        /// The change at the given `location`.
        pub change: T,
        /// The repository-relative location of this destination.
        pub location: &'a BStr,
    }
}

///
pub mod emit {
    /// The error returned by [Tracker::emit()](super::Tracker::emit()).
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("Could not find blob for similarity checking")]
        FindExistingBlob(#[from] gix_object::find::existing_object::Error),
        #[error("Could not obtain exhaustive item set to use as possible sources for copy detection")]
        GetItemsForExhaustiveCopyDetection(#[source] Box<dyn std::error::Error + Send + Sync>),
        #[error(transparent)]
        SetResource(#[from] crate::blob::platform::set_resource::Error),
        #[error(transparent)]
        PrepareDiff(#[from] crate::blob::platform::prepare_diff::Error),
    }
}

/// Lifecycle
impl<T: Change> Tracker<T> {
    /// Create a new instance with `rewrites` configuration.
    pub fn new(rewrites: Rewrites) -> Self {
        Tracker {
            items: vec![],
            path_backing: vec![],
            rewrites,
        }
    }
}

/// build state and find matches.
impl<T: Change> Tracker<T> {
    /// We may refuse the push if that information isn't needed for what we have to track.
    pub fn try_push_change(&mut self, change: T, location: &BStr) -> Option<T> {
        if !change.entry_mode().is_blob_or_symlink() {
            return Some(change);
        }
        let keep = match (self.rewrites.copies, change.kind()) {
            (Some(_find_copies), _) => true,
            (None, ChangeKind::Modification { .. }) => false,
            (None, _) => true,
        };

        if !keep {
            return Some(change);
        }

        let start = self.path_backing.len();
        self.path_backing.extend_from_slice(location);
        self.items.push(Item {
            path: start..self.path_backing.len(),
            change,
            emitted: false,
        });
        None
    }

    /// Can only be called once effectively as it alters its own state to assure each item is only emitted once.
    ///
    /// `cb(destination, source)` is called for each item, either with `Some(source)` if it's
    /// the destination of a copy or rename, or with `None` for source if no relation to other
    /// items in the tracked set exist, which is like saying 'no rename or rewrite or copy' happened.
    ///
    /// `objects` is used to access blob data for similarity checks if required and is taken directly from the object database.
    /// Worktree filters and text conversions will be applied afterwards automatically. Note that object-caching *should not*
    /// be enabled as caching is implemented by `diff_cache`, after all, the blob that's actually diffed is going
    /// through conversion steps.
    ///
    /// `diff_cache` is a way to retain a cache of resources that are prepared for rapid diffing, and it also controls
    /// the diff-algorithm (provided no user-algorithm is set).
    /// Note that we control a few options of `diff_cache` to assure it will ignore external commands.
    /// Note that we do not control how the `diff_cache` converts resources, it's left to the caller to decide
    /// if it should look at what's stored in `git`, or in the working tree, along with all diff-specific conversions.
    ///
    /// `push_source_tree(push_fn: push(change, location))` is a function that is called when the entire tree of the source
    /// should be added as modifications by calling `push` repeatedly to use for perfect copy tracking. Note that `push`
    /// will panic if `change` is not a modification, and it's valid to not call `push` at all.
    pub fn emit<PushSourceTreeFn, E>(
        &mut self,
        mut cb: impl FnMut(visit::Destination<'_, T>, Option<visit::Source<'_>>) -> crate::tree::visit::Action,
        diff_cache: &mut crate::blob::Platform,
        objects: &impl gix_object::FindObjectOrHeader,
        mut push_source_tree: PushSourceTreeFn,
    ) -> Result<Outcome, emit::Error>
    where
        PushSourceTreeFn: FnMut(&mut dyn FnMut(T, &BStr)) -> Result<(), E>,
        E: std::error::Error + Send + Sync + 'static,
    {
        diff_cache.options.skip_internal_diff_if_external_is_configured = false;

        fn by_id_and_location<T: Change>(a: &Item<T>, b: &Item<T>) -> std::cmp::Ordering {
            a.change
                .id()
                .cmp(b.change.id())
                .then_with(|| a.path.start.cmp(&b.path.start).then(a.path.end.cmp(&b.path.end)))
        }
        self.items.sort_by(by_id_and_location);

        let mut out = Outcome {
            options: self.rewrites,
            ..Default::default()
        };
        self.match_pairs_of_kind(
            visit::SourceKind::Rename,
            &mut cb,
            self.rewrites.percentage,
            &mut out,
            diff_cache,
            objects,
        )?;

        if let Some(copies) = self.rewrites.copies {
            self.match_pairs_of_kind(
                visit::SourceKind::Copy,
                &mut cb,
                copies.percentage,
                &mut out,
                diff_cache,
                objects,
            )?;

            match copies.source {
                CopySource::FromSetOfModifiedFiles => {}
                CopySource::FromSetOfModifiedFilesAndAllSources => {
                    push_source_tree(&mut |change, location| {
                        assert!(
                            self.try_push_change(change, location).is_none(),
                            "we must accept every change"
                        );
                        // make sure these aren't viable to be emitted anymore.
                        self.items.last_mut().expect("just pushed").emitted = true;
                    })
                    .map_err(|err| emit::Error::GetItemsForExhaustiveCopyDetection(Box::new(err)))?;
                    self.items.sort_by(by_id_and_location);

                    self.match_pairs_of_kind(
                        visit::SourceKind::Copy,
                        &mut cb,
                        copies.percentage,
                        &mut out,
                        diff_cache,
                        objects,
                    )?;
                }
            }
        }

        self.items
            .sort_by(|a, b| a.location(&self.path_backing).cmp(b.location(&self.path_backing)));
        for item in self.items.drain(..).filter(|item| !item.emitted) {
            if cb(
                visit::Destination {
                    location: item.location(&self.path_backing),
                    change: item.change,
                },
                None,
            ) == crate::tree::visit::Action::Cancel
            {
                break;
            }
        }
        Ok(out)
    }
}

impl<T: Change> Tracker<T> {
    fn match_pairs_of_kind(
        &mut self,
        kind: visit::SourceKind,
        cb: &mut impl FnMut(visit::Destination<'_, T>, Option<visit::Source<'_>>) -> crate::tree::visit::Action,
        percentage: Option<f32>,
        out: &mut Outcome,
        diff_cache: &mut crate::blob::Platform,
        objects: &impl gix_object::FindObjectOrHeader,
    ) -> Result<(), emit::Error> {
        // we try to cheaply reduce the set of possibilities first, before possibly looking more exhaustively.
        let needs_second_pass = !needs_exact_match(percentage);
        if self.match_pairs(cb, None /* by identity */, kind, out, diff_cache, objects)?
            == crate::tree::visit::Action::Cancel
        {
            return Ok(());
        }
        if needs_second_pass {
            let is_limited = if self.rewrites.limit == 0 {
                false
            } else {
                let (num_src, num_dst) =
                    estimate_involved_items(self.items.iter().map(|item| (item.emitted, item.change.kind())), kind);
                let permutations = num_src * num_dst;
                if permutations > self.rewrites.limit {
                    match kind {
                        visit::SourceKind::Rename => {
                            out.num_similarity_checks_skipped_for_rename_tracking_due_to_limit = permutations;
                        }
                        visit::SourceKind::Copy => {
                            out.num_similarity_checks_skipped_for_copy_tracking_due_to_limit = permutations;
                        }
                    }
                    true
                } else {
                    false
                }
            };
            if !is_limited {
                self.match_pairs(cb, percentage, kind, out, diff_cache, objects)?;
            }
        }
        Ok(())
    }

    fn match_pairs(
        &mut self,
        cb: &mut impl FnMut(visit::Destination<'_, T>, Option<visit::Source<'_>>) -> crate::tree::visit::Action,
        percentage: Option<f32>,
        kind: visit::SourceKind,
        stats: &mut Outcome,
        diff_cache: &mut crate::blob::Platform,
        objects: &impl gix_object::FindObjectOrHeader,
    ) -> Result<crate::tree::visit::Action, emit::Error> {
        let mut dest_ofs = 0;
        while let Some((mut dest_idx, dest)) = self.items[dest_ofs..].iter().enumerate().find_map(|(idx, item)| {
            (!item.emitted && matches!(item.change.kind(), ChangeKind::Addition)).then_some((idx, item))
        }) {
            dest_idx += dest_ofs;
            dest_ofs = dest_idx + 1;
            let src = find_match(
                &self.items,
                dest,
                dest_idx,
                percentage,
                kind,
                stats,
                objects,
                diff_cache,
                &self.path_backing,
            )?
            .map(|(src_idx, src, diff)| {
                let (id, entry_mode) = src.change.id_and_entry_mode();
                let id = id.to_owned();
                let location = src.location(&self.path_backing);
                (
                    visit::Source {
                        entry_mode,
                        id,
                        kind,
                        location,
                        diff,
                    },
                    src_idx,
                )
            });
            if src.is_none() {
                continue;
            }
            let location = dest.location(&self.path_backing);
            let change = dest.change.clone();
            let dest = visit::Destination { change, location };
            self.items[dest_idx].emitted = true;
            if let Some(src_idx) = src.as_ref().map(|t| t.1) {
                self.items[src_idx].emitted = true;
            }
            if cb(dest, src.map(|t| t.0)) == crate::tree::visit::Action::Cancel {
                return Ok(crate::tree::visit::Action::Cancel);
            }
        }
        Ok(crate::tree::visit::Action::Continue)
    }
}

/// Returns the amount of viable sources and destinations for `items` as eligible for the given `kind` of operation.
fn estimate_involved_items(
    items: impl IntoIterator<Item = (bool, ChangeKind)>,
    kind: visit::SourceKind,
) -> (usize, usize) {
    items
        .into_iter()
        .filter(|(emitted, _)| match kind {
            visit::SourceKind::Rename => !*emitted,
            visit::SourceKind::Copy => true,
        })
        .fold((0, 0), |(mut src, mut dest), (emitted, change_kind)| {
            match change_kind {
                ChangeKind::Addition => {
                    if kind == visit::SourceKind::Rename || !emitted {
                        dest += 1;
                    }
                }
                ChangeKind::Deletion => {
                    if kind == visit::SourceKind::Rename {
                        src += 1
                    }
                }
                ChangeKind::Modification => {
                    if kind == visit::SourceKind::Copy {
                        src += 1
                    }
                }
            }
            (src, dest)
        })
}

fn needs_exact_match(percentage: Option<f32>) -> bool {
    percentage.map_or(true, |p| p >= 1.0)
}

/// <`src_idx`, src, possibly diff stat>
type SourceTuple<'a, T> = (usize, &'a Item<T>, Option<DiffLineStats>);

/// Find `item` in our set of items ignoring `item_idx` to avoid finding ourselves, by similarity indicated by `percentage`.
/// The latter can be `None` or `Some(x)` where `x>=1` for identity, and anything else for similarity.
/// We also ignore emitted items entirely.
/// Use `kind` to indicate what kind of match we are looking for, which might be deletions matching an `item` addition, or
/// any non-deletion otherwise.
/// Note that we always try to find by identity first even if a percentage is given as it's much faster and may reduce the set
/// of items to be searched.
#[allow(clippy::too_many_arguments)]
fn find_match<'a, T: Change>(
    items: &'a [Item<T>],
    item: &Item<T>,
    item_idx: usize,
    percentage: Option<f32>,
    kind: visit::SourceKind,
    stats: &mut Outcome,
    objects: &impl gix_object::FindObjectOrHeader,
    diff_cache: &mut crate::blob::Platform,
    path_backing: &[u8],
) -> Result<Option<SourceTuple<'a, T>>, emit::Error> {
    let (item_id, item_mode) = item.change.id_and_entry_mode();
    if needs_exact_match(percentage) || item_mode.is_link() {
        let first_idx = items.partition_point(|a| a.change.id() < item_id);
        let range = match items.get(first_idx..).map(|items| {
            let end = items
                .iter()
                .position(|a| a.change.id() != item_id)
                .map_or(items.len(), |idx| first_idx + idx);
            first_idx..end
        }) {
            Some(range) => range,
            None => return Ok(None),
        };
        if range.is_empty() {
            return Ok(None);
        }
        let res = items[range.clone()].iter().enumerate().find_map(|(mut src_idx, src)| {
            src_idx += range.start;
            (src_idx != item_idx && src.is_source_for_destination_of(kind, item_mode)).then_some((src_idx, src, None))
        });
        if let Some(src) = res {
            return Ok(Some(src));
        }
    } else {
        let mut has_new = false;
        let percentage = percentage.expect("it's set to something below 1.0 and we assured this");
        debug_assert_eq!(
            item.change.entry_mode().kind(),
            EntryKind::Blob,
            "symlinks are matched exactly, and trees aren't used here"
        );

        for (can_idx, src) in items
            .iter()
            .enumerate()
            .filter(|(src_idx, src)| *src_idx != item_idx && src.is_source_for_destination_of(kind, item_mode))
        {
            if !has_new {
                diff_cache.set_resource(
                    item_id.to_owned(),
                    item_mode.kind(),
                    item.location(path_backing),
                    ResourceKind::NewOrDestination,
                    objects,
                )?;
                has_new = true;
            }
            let (src_id, src_mode) = src.change.id_and_entry_mode();
            diff_cache.set_resource(
                src_id.to_owned(),
                src_mode.kind(),
                src.location(path_backing),
                ResourceKind::OldOrSource,
                objects,
            )?;
            let prep = diff_cache.prepare_diff()?;
            stats.num_similarity_checks += 1;
            match prep.operation {
                Operation::InternalDiff { algorithm } => {
                    let tokens =
                        crate::blob::intern::InternedInput::new(prep.old.intern_source(), prep.new.intern_source());
                    let counts = crate::blob::diff(
                        algorithm,
                        &tokens,
                        crate::blob::sink::Counter::new(diff::Statistics {
                            removed_bytes: 0,
                            input: &tokens,
                        }),
                    );
                    let old_data_len = prep.old.data.as_slice().unwrap_or_default().len();
                    let new_data_len = prep.new.data.as_slice().unwrap_or_default().len();
                    let similarity = (old_data_len - counts.wrapped) as f32 / old_data_len.max(new_data_len) as f32;
                    if similarity >= percentage {
                        return Ok(Some((
                            can_idx,
                            src,
                            DiffLineStats {
                                removals: counts.removals,
                                insertions: counts.insertions,
                                before: tokens.before.len().try_into().expect("interner handles only u32"),
                                after: tokens.after.len().try_into().expect("interner handles only u32"),
                                similarity,
                            }
                            .into(),
                        )));
                    }
                }
                Operation::ExternalCommand { .. } => {
                    unreachable!("we have disabled this possibility with an option")
                }
                Operation::SourceOrDestinationIsBinary => {
                    // TODO: figure out if git does more here
                }
            };
        }
    }
    Ok(None)
}

mod diff {
    use std::ops::Range;

    pub struct Statistics<'a, 'data> {
        pub removed_bytes: usize,
        pub input: &'a crate::blob::intern::InternedInput<&'data [u8]>,
    }

    impl<'a, 'data> crate::blob::Sink for Statistics<'a, 'data> {
        type Out = usize;

        fn process_change(&mut self, before: Range<u32>, _after: Range<u32>) {
            self.removed_bytes = self.input.before[before.start as usize..before.end as usize]
                .iter()
                .map(|token| self.input.interner[*token].len())
                .sum();
        }

        fn finish(self) -> Self::Out {
            self.removed_bytes
        }
    }
}

#[cfg(test)]
mod estimate_involved_items {
    use super::estimate_involved_items;
    use crate::rewrites::tracker::{visit::SourceKind, ChangeKind};

    #[test]
    fn renames_count_unemitted_as_sources_and_destinations() {
        let items = [
            (false, ChangeKind::Addition),
            (true, ChangeKind::Deletion),
            (true, ChangeKind::Deletion),
        ];
        assert_eq!(
            estimate_involved_items(items, SourceKind::Rename),
            (0, 1),
            "here we only have one eligible source, hence nothing to do"
        );
        assert_eq!(
            estimate_involved_items(items.into_iter().map(|t| (false, t.1)), SourceKind::Rename),
            (2, 1),
            "now we have more possibilities as renames count un-emitted deletions as source"
        );
    }

    #[test]
    fn copies_do_not_count_additions_as_sources() {
        let items = [
            (false, ChangeKind::Addition),
            (true, ChangeKind::Addition),
            (true, ChangeKind::Deletion),
        ];
        assert_eq!(
            estimate_involved_items(items, SourceKind::Copy),
            (0, 1),
            "one addition as source, the other isn't counted as it's emitted, nor is it considered a copy-source.\
            deletions don't count"
        );
    }

    #[test]
    fn copies_count_modifications_as_sources() {
        let items = [
            (false, ChangeKind::Addition),
            (true, ChangeKind::Modification),
            (false, ChangeKind::Modification),
        ];
        assert_eq!(
            estimate_involved_items(items, SourceKind::Copy),
            (2, 1),
            "any modifications is a valid source, emitted or not"
        );
    }
}
