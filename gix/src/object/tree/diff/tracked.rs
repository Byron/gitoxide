use std::ops::Range;

use gix_diff::tree::visit::Change;
use gix_object::tree::EntryMode;

use crate::{
    bstr::BStr,
    ext::ObjectIdExt,
    object::tree::diff::{
        change::DiffLineStats,
        rewrites::{CopySource, Outcome},
        Rewrites,
    },
    Repository, Tree,
};

/// A set of tracked items allows to figure out their relations by figuring out their similarity.
pub struct Item {
    /// The underlying raw change
    change: Change,
    /// That slice into the backing for paths.
    location: Range<usize>,
    /// If true, this item was already emitted, i.e. seen by the caller.
    emitted: bool,
}

impl Item {
    fn location<'a>(&self, backing: &'a [u8]) -> &'a BStr {
        backing[self.location.clone()].as_ref()
    }
    fn entry_mode_compatible(&self, mode: EntryMode) -> bool {
        use EntryMode::*;
        matches!(
            (mode, self.change.entry_mode()),
            (Blob | BlobExecutable, Blob | BlobExecutable) | (Link, Link)
        )
    }

    fn is_source_for_destination_of(&self, kind: visit::Kind, dest_item_mode: EntryMode) -> bool {
        self.entry_mode_compatible(dest_item_mode)
            && match kind {
                visit::Kind::RenameTarget => !self.emitted && matches!(self.change, Change::Deletion { .. }),
                visit::Kind::CopyDestination => {
                    matches!(self.change, Change::Modification { .. })
                }
            }
    }
}

pub struct State {
    items: Vec<Item>,
    path_backing: Vec<u8>,
    rewrites: Rewrites,
    tracking: Option<gix_diff::tree::recorder::Location>,
}

pub mod visit {
    use crate::{bstr::BStr, object::tree::diff::change::DiffLineStats};

    pub struct Source<'a> {
        pub mode: gix_object::tree::EntryMode,
        pub id: gix_hash::ObjectId,
        pub kind: Kind,
        pub location: &'a BStr,
        pub diff: Option<DiffLineStats>,
    }

    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub enum Kind {
        RenameTarget,
        CopyDestination,
    }

    pub struct Destination<'a> {
        pub change: gix_diff::tree::visit::Change,
        pub location: &'a BStr,
    }
}

impl State {
    pub(crate) fn new(renames: Rewrites, tracking: Option<gix_diff::tree::recorder::Location>) -> Self {
        State {
            items: vec![],
            path_backing: vec![],
            rewrites: renames,
            tracking,
        }
    }
}

/// build state and find matches.
impl State {
    /// We may refuse the push if that information isn't needed for what we have to track.
    pub fn try_push_change(&mut self, change: Change, location: &BStr) -> Option<Change> {
        if !change.entry_mode().is_blob_or_symlink() {
            return Some(change);
        }
        let keep = match (self.rewrites.copies, &change) {
            (Some(_find_copies), _) => true,
            (None, Change::Modification { .. }) => false,
            (None, _) => true,
        };

        if !keep {
            return Some(change);
        }

        let start = self.path_backing.len();
        self.path_backing.extend_from_slice(location);
        self.items.push(Item {
            location: start..self.path_backing.len(),
            change,
            emitted: false,
        });
        None
    }

    /// Can only be called once effectively as it alters its own state.
    ///
    /// `cb(destination, source)` is called for each item, either with `Some(source)` if it's
    /// the destination of a copy or rename, or with `None` for source if no relation to other
    /// items in the tracked set exist.
    pub fn emit(
        &mut self,
        mut cb: impl FnMut(visit::Destination<'_>, Option<visit::Source<'_>>) -> gix_diff::tree::visit::Action,
        src_tree: &Tree<'_>,
    ) -> Result<Outcome, crate::object::tree::diff::for_each::Error> {
        fn by_id_and_location(a: &Item, b: &Item) -> std::cmp::Ordering {
            a.change.oid().cmp(b.change.oid()).then_with(|| {
                a.location
                    .start
                    .cmp(&b.location.start)
                    .then(a.location.end.cmp(&b.location.end))
            })
        }
        self.items.sort_by(by_id_and_location);

        let mut out = Outcome {
            options: self.rewrites,
            ..Default::default()
        };
        out = self.match_pairs_of_kind(
            visit::Kind::RenameTarget,
            &mut cb,
            self.rewrites.percentage,
            out,
            src_tree.repo,
        )?;

        if let Some(copies) = self.rewrites.copies {
            out = self.match_pairs_of_kind(
                visit::Kind::CopyDestination,
                &mut cb,
                copies.percentage,
                out,
                src_tree.repo,
            )?;

            match copies.source {
                CopySource::FromSetOfModifiedFiles => {}
                CopySource::FromSetOfModifiedFilesAndSourceTree => {
                    src_tree
                        .traverse()
                        .breadthfirst(&mut tree_to_events::Delegate::new(self))?;
                    self.items.sort_by(by_id_and_location);

                    out = self.match_pairs_of_kind(
                        visit::Kind::CopyDestination,
                        &mut cb,
                        copies.percentage,
                        out,
                        src_tree.repo,
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
            ) == gix_diff::tree::visit::Action::Cancel
            {
                break;
            }
        }
        Ok(out)
    }

    fn match_pairs_of_kind(
        &mut self,
        kind: visit::Kind,
        cb: &mut impl FnMut(visit::Destination<'_>, Option<visit::Source<'_>>) -> gix_diff::tree::visit::Action,
        percentage: Option<f32>,
        mut out: Outcome,
        repo: &Repository,
    ) -> Result<Outcome, crate::object::tree::diff::for_each::Error> {
        // we try to cheaply reduce the set of possibilities first, before possibly looking more exhaustively.
        let needs_second_pass = !needs_exact_match(percentage);
        if self.match_pairs(cb, None /* by identity */, kind, repo, &mut out)? == gix_diff::tree::visit::Action::Cancel
        {
            return Ok(out);
        }
        if needs_second_pass {
            let is_limited = if self.rewrites.limit == 0 {
                false
            } else if let Some(permutations) = permutations_over_limit(&self.items, self.rewrites.limit, kind) {
                match kind {
                    visit::Kind::RenameTarget => {
                        out.num_similarity_checks_skipped_for_rename_tracking_due_to_limit = permutations;
                    }
                    visit::Kind::CopyDestination => {
                        out.num_similarity_checks_skipped_for_copy_tracking_due_to_limit = permutations;
                    }
                }
                true
            } else {
                false
            };
            if !is_limited {
                self.match_pairs(cb, self.rewrites.percentage, kind, repo, &mut out)?;
            }
        }
        Ok(out)
    }

    fn match_pairs(
        &mut self,
        cb: &mut impl FnMut(visit::Destination<'_>, Option<visit::Source<'_>>) -> gix_diff::tree::visit::Action,
        percentage: Option<f32>,
        kind: visit::Kind,
        repo: &Repository,
        stats: &mut Outcome,
    ) -> Result<gix_diff::tree::visit::Action, crate::object::tree::diff::for_each::Error> {
        // TODO(perf): reuse object data and interner state and interned tokens, make these available to `find_match()`
        let mut dest_ofs = 0;
        while let Some((mut dest_idx, dest)) = self.items[dest_ofs..].iter().enumerate().find_map(|(idx, item)| {
            (!item.emitted && matches!(item.change, Change::Addition { .. })).then_some((idx, item))
        }) {
            dest_idx += dest_ofs;
            dest_ofs = dest_idx + 1;
            let src =
                find_match(&self.items, dest, dest_idx, percentage, kind, repo, stats)?.map(|(src_idx, src, diff)| {
                    let (id, mode) = src.change.oid_and_entry_mode();
                    let id = id.to_owned();
                    let location = src.location(&self.path_backing);
                    (
                        visit::Source {
                            mode,
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
            if cb(dest, src.map(|t| t.0)) == gix_diff::tree::visit::Action::Cancel {
                return Ok(gix_diff::tree::visit::Action::Cancel);
            }
        }
        Ok(gix_diff::tree::visit::Action::Continue)
    }
}

fn permutations_over_limit(items: &[Item], limit: usize, kind: visit::Kind) -> Option<usize> {
    let (sources, destinations) = items
        .iter()
        .filter(|item| match kind {
            visit::Kind::RenameTarget => !item.emitted,
            visit::Kind::CopyDestination => true,
        })
        .fold((0, 0), |(mut src, mut dest), item| {
            match item.change {
                Change::Addition { .. } => {
                    dest += 1;
                }
                Change::Deletion { .. } => {
                    if kind == visit::Kind::RenameTarget {
                        src += 1
                    }
                }
                Change::Modification { .. } => {
                    if kind == visit::Kind::CopyDestination {
                        src += 1
                    }
                }
            }
            (src, dest)
        });
    let permutations = sources * destinations;
    (permutations > limit * limit).then_some(permutations)
}

fn needs_exact_match(percentage: Option<f32>) -> bool {
    percentage.map_or(true, |p| p >= 1.0)
}

/// <`src_idx`, src, possibly diff stat>
type SourceTuple<'a> = (usize, &'a Item, Option<DiffLineStats>);

/// Find `item` in our set of items ignoring `item_idx` to avoid finding ourselves, by similarity indicated by `percentage`.
/// The latter can be `None` or `Some(x)` where `x>=1` for identity, and anything else for similarity.
/// We also ignore emitted items entirely.
/// Use `kind` to indicate what kind of match we are looking for, which might be deletions matching an `item` addition, or
/// any non-deletion otherwise.
/// Note that we always try to find by identity first even if a percentage is given as it's much faster and may reduce the set
/// of items to be searched.
fn find_match<'a>(
    items: &'a [Item],
    item: &Item,
    item_idx: usize,
    percentage: Option<f32>,
    kind: visit::Kind,
    repo: &Repository,
    stats: &mut Outcome,
) -> Result<Option<SourceTuple<'a>>, crate::object::tree::diff::for_each::Error> {
    let (item_id, item_mode) = item.change.oid_and_entry_mode();
    if needs_exact_match(percentage) || item_mode == gix_object::tree::EntryMode::Link {
        let first_idx = items.partition_point(|a| a.change.oid() < item_id);
        let range = match items.get(first_idx..).map(|items| {
            let end = items
                .iter()
                .position(|a| a.change.oid() != item_id)
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
        let new = item_id.to_owned().attach(repo).object()?;
        let percentage = percentage.expect("it's set to something below 1.0 and we assured this");
        debug_assert!(
            item.change.entry_mode().is_blob(),
            "symlinks are matched exactly, and trees aren't used here"
        );
        let algo = repo.config.diff_algorithm()?;
        for (can_idx, src) in items
            .iter()
            .enumerate()
            .filter(|(src_idx, src)| *src_idx != item_idx && src.is_source_for_destination_of(kind, item_mode))
        {
            let old = src.change.oid().to_owned().attach(repo).object()?;
            // TODO: make sure we get attribute handling and binary skips and filters right here. There is crate::object::blob::diff::Platform
            //       which should have facilities for that one day, but we don't use it because we need newlines in our tokens.
            let tokens = gix_diff::blob::intern::InternedInput::new(
                gix_diff::blob::sources::byte_lines_with_terminator(&old.data),
                gix_diff::blob::sources::byte_lines_with_terminator(&new.data),
            );
            let counts = gix_diff::blob::diff(
                algo,
                &tokens,
                gix_diff::blob::sink::Counter::new(diff::Statistics {
                    removed_bytes: 0,
                    input: &tokens,
                }),
            );
            let similarity = (old.data.len() - counts.wrapped) as f32 / old.data.len().max(new.data.len()) as f32;
            stats.num_similarity_checks += 1;
            if similarity >= percentage {
                return Ok(Some((
                    can_idx,
                    src,
                    DiffLineStats {
                        removals: counts.removals,
                        insertions: counts.insertions,
                        before: tokens.before.len().try_into().expect("interner handles only u32"),
                        after: tokens.after.len().try_into().expect("interner handles only u32"),
                    }
                    .into(),
                )));
            }
        }
    }
    Ok(None)
}

mod diff {
    use std::ops::Range;

    pub struct Statistics<'a, 'data> {
        pub removed_bytes: usize,
        pub input: &'a gix_diff::blob::intern::InternedInput<&'data [u8]>,
    }

    impl<'a, 'data> gix_diff::blob::Sink for Statistics<'a, 'data> {
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

mod tree_to_events {
    use gix_diff::tree::visit::Change;
    use gix_object::tree::EntryRef;

    use crate::bstr::BStr;

    pub struct Delegate<'a> {
        parent: &'a mut super::State,
        recorder: gix_traverse::tree::Recorder,
    }

    impl<'a> Delegate<'a> {
        pub fn new(parent: &'a mut super::State) -> Self {
            let tracking = parent.tracking.map(|t| match t {
                gix_diff::tree::recorder::Location::FileName => gix_traverse::tree::recorder::Location::FileName,
                gix_diff::tree::recorder::Location::Path => gix_traverse::tree::recorder::Location::Path,
            });
            Self {
                parent,
                recorder: gix_traverse::tree::Recorder::default().track_location(tracking),
            }
        }
    }

    impl gix_traverse::tree::Visit for Delegate<'_> {
        fn pop_front_tracked_path_and_set_current(&mut self) {
            self.recorder.pop_front_tracked_path_and_set_current()
        }

        fn push_back_tracked_path_component(&mut self, component: &BStr) {
            self.recorder.push_back_tracked_path_component(component)
        }

        fn push_path_component(&mut self, component: &BStr) {
            self.recorder.push_path_component(component)
        }

        fn pop_path_component(&mut self) {
            self.recorder.pop_path_component();
        }

        fn visit_tree(&mut self, _entry: &EntryRef<'_>) -> gix_traverse::tree::visit::Action {
            gix_traverse::tree::visit::Action::Continue
        }

        fn visit_nontree(&mut self, entry: &EntryRef<'_>) -> gix_traverse::tree::visit::Action {
            if entry.mode.is_blob() {
                self.parent.try_push_change(
                    Change::Modification {
                        previous_entry_mode: entry.mode,
                        previous_oid: gix_hash::ObjectId::null(entry.oid.kind()),
                        entry_mode: entry.mode,
                        oid: entry.oid.to_owned(),
                    },
                    self.recorder.path(),
                );
                // make sure these aren't viable to be emitted anymore.
                self.parent.items.last_mut().expect("just pushed").emitted = true;
            }
            gix_traverse::tree::visit::Action::Continue
        }
    }
}
