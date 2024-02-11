use bstr::{BStr, BString, ByteSlice};
use std::borrow::Cow;
use std::path::PathBuf;

use crate::entry::{PathspecMatch, Status};
use crate::walk::function::{can_recurse, emit_entry};
use crate::walk::EmissionMode::CollapseDirectory;
use crate::walk::{classify, Action, Context, Delegate, Error, Options, Outcome};
use crate::{entry, walk, Entry};

/// ### Deviation
///
/// Git mostly silently ignores IO errors and stops iterating seemingly quietly, while we error loudly.
#[allow(clippy::too_many_arguments)]
pub(super) fn recursive(
    is_worktree_dir: bool,
    current: &mut PathBuf,
    current_bstr: &mut BString,
    current_info: classify::Outcome,
    ctx: &mut Context<'_>,
    opts: Options,
    delegate: &mut dyn Delegate,
    out: &mut Outcome,
    state: &mut State,
) -> Result<(Action, bool), Error> {
    out.read_dir_calls += 1;
    let entries = gix_fs::read_dir(current, opts.precompose_unicode).map_err(|err| Error::ReadDir {
        path: current.to_owned(),
        source: err,
    })?;

    let mut num_entries = 0;
    let mark = state.mark(is_worktree_dir);
    let mut prevent_collapse = false;
    for entry in entries {
        let entry = entry.map_err(|err| Error::DirEntry {
            parent_directory: current.to_owned(),
            source: err,
        })?;
        // Important to count right away, otherwise the directory could be seen as empty even though it's not.
        // That is, this should be independent of the kind.
        num_entries += 1;

        let prev_len = current_bstr.len();
        if prev_len != 0 {
            current_bstr.push(b'/');
        }
        let file_name = entry.file_name();
        current_bstr.extend_from_slice(
            gix_path::try_os_str_into_bstr(Cow::Borrowed(file_name.as_ref()))
                .expect("no illformed UTF-8")
                .as_ref(),
        );
        current.push(file_name);

        let info = classify::path(
            current,
            current_bstr,
            if prev_len == 0 { 0 } else { prev_len + 1 },
            None,
            || entry.file_type().ok().map(Into::into),
            opts,
            ctx,
        )?;

        if can_recurse(current_bstr.as_bstr(), info, opts.for_deletion, delegate) {
            let (action, subdir_prevent_collapse) =
                recursive(false, current, current_bstr, info, ctx, opts, delegate, out, state)?;
            prevent_collapse = subdir_prevent_collapse;
            if action != Action::Continue {
                break;
            }
        } else if !state.held_for_directory_collapse(current_bstr.as_bstr(), info, &opts) {
            let action = emit_entry(Cow::Borrowed(current_bstr.as_bstr()), info, None, opts, out, delegate);
            if action != Action::Continue {
                return Ok((action, prevent_collapse));
            }
        }
        current_bstr.truncate(prev_len);
        current.pop();
    }

    let res = mark.reduce_held_entries(
        num_entries,
        state,
        &mut prevent_collapse,
        current_bstr.as_bstr(),
        current_info,
        opts,
        out,
        ctx,
        delegate,
    );
    Ok((res, prevent_collapse))
}

#[derive(Default)]
pub(super) struct State {
    /// The entries to hold back until it's clear what to do with them.
    pub on_hold: Vec<Entry>,
}

impl State {
    /// Hold the entry with the given `status` if it's a candidate for collapsing the containing directory.
    fn held_for_directory_collapse(&mut self, rela_path: &BStr, info: classify::Outcome, opts: &Options) -> bool {
        if opts.should_hold(info.status) {
            self.on_hold.push(Entry {
                rela_path: rela_path.to_owned(),
                status: info.status,
                disk_kind: info.disk_kind,
                index_kind: info.index_kind,
                pathspec_match: info.pathspec_match,
            });
            true
        } else {
            false
        }
    }

    /// Keep track of state we need to later resolve the state.
    /// Worktree directories are special, as they don't fold.
    fn mark(&self, is_worktree_dir: bool) -> Mark {
        Mark {
            start_index: self.on_hold.len(),
            is_worktree_dir,
        }
    }
}

struct Mark {
    start_index: usize,
    is_worktree_dir: bool,
}

impl Mark {
    #[allow(clippy::too_many_arguments)]
    fn reduce_held_entries(
        mut self,
        num_entries: usize,
        state: &mut State,
        prevent_collapse: &mut bool,
        dir_rela_path: &BStr,
        dir_info: classify::Outcome,
        opts: Options,
        out: &mut walk::Outcome,
        ctx: &mut Context<'_>,
        delegate: &mut dyn walk::Delegate,
    ) -> walk::Action {
        if num_entries == 0 {
            let empty_info = classify::Outcome {
                disk_kind: if num_entries == 0 {
                    assert_ne!(
                        dir_info.disk_kind,
                        Some(entry::Kind::Repository),
                        "BUG: it shouldn't be possible to classify an empty dir as repository"
                    );
                    Some(entry::Kind::EmptyDirectory)
                } else {
                    dir_info.disk_kind
                },
                ..dir_info
            };
            if opts.should_hold(empty_info.status) {
                state.on_hold.push(Entry {
                    rela_path: dir_rela_path.to_owned(),
                    status: empty_info.status,
                    disk_kind: empty_info.disk_kind,
                    index_kind: empty_info.index_kind,
                    pathspec_match: empty_info.pathspec_match,
                });
                Action::Continue
            } else {
                emit_entry(Cow::Borrowed(dir_rela_path), empty_info, None, opts, out, delegate)
            }
        } else if *prevent_collapse {
            self.emit_all_held(state, opts, out, delegate)
        } else if let Some(action) = self.try_collapse(
            dir_rela_path,
            dir_info,
            state,
            prevent_collapse,
            out,
            opts,
            ctx,
            delegate,
        ) {
            action
        } else {
            self.emit_all_held(state, opts, out, delegate)
        }
    }

    fn emit_all_held(
        &mut self,
        state: &mut State,
        opts: Options,
        out: &mut walk::Outcome,
        delegate: &mut dyn walk::Delegate,
    ) -> Action {
        for entry in state.on_hold.drain(self.start_index..) {
            let info = classify::Outcome::from(&entry);
            let action = emit_entry(Cow::Owned(entry.rela_path), info, None, opts, out, delegate);
            if action != Action::Continue {
                return action;
            }
        }
        Action::Continue
    }

    #[allow(clippy::too_many_arguments)]
    fn try_collapse(
        &self,
        dir_rela_path: &BStr,
        dir_info: classify::Outcome,
        state: &mut State,
        prevent_collapse: &mut bool,
        out: &mut walk::Outcome,
        opts: Options,
        ctx: &mut Context<'_>,
        delegate: &mut dyn walk::Delegate,
    ) -> Option<Action> {
        if self.is_worktree_dir {
            return None;
        }
        let (mut expendable, mut precious, mut untracked, mut entries, mut matching_entries) = (0, 0, 0, 0, 0);
        for (kind, status, pathspec_match) in state.on_hold[self.start_index..]
            .iter()
            .map(|e| (e.disk_kind, e.status, e.pathspec_match))
        {
            entries += 1;
            if kind == Some(entry::Kind::Repository) {
                *prevent_collapse = true;
                return None;
            }
            if pathspec_match.map_or(false, |m| {
                matches!(m, PathspecMatch::Verbatim | PathspecMatch::Excluded)
            }) {
                return None;
            }
            matching_entries += usize::from(pathspec_match.map_or(false, |m| !m.should_ignore()));
            match status {
                Status::DotGit | Status::Pruned | Status::TrackedExcluded => {
                    unreachable!("BUG: pruned aren't ever held, check `should_hold()`")
                }
                Status::Tracked => { /* causes the folder not to be collapsed */ }
                Status::Ignored(gix_ignore::Kind::Expendable) => expendable += 1,
                Status::Ignored(gix_ignore::Kind::Precious) => precious += 1,
                Status::Untracked => untracked += 1,
            }
        }

        if matching_entries != 0 && matching_entries != entries {
            return None;
        }

        let dir_status = if opts.emit_untracked == CollapseDirectory
            && untracked != 0
            && untracked + expendable + precious == entries
            && (opts.for_deletion.is_none()
                || (precious == 0 && expendable == 0)
                || (precious == 0 && opts.emit_ignored.is_some()))
        {
            entry::Status::Untracked
        } else if opts.emit_ignored == Some(CollapseDirectory) {
            if expendable != 0 && expendable == entries {
                entry::Status::Ignored(gix_ignore::Kind::Expendable)
            } else if precious != 0 && precious == entries {
                entry::Status::Ignored(gix_ignore::Kind::Precious)
            } else {
                return None;
            }
        } else {
            return None;
        };

        if !matches!(dir_status, entry::Status::Untracked | entry::Status::Ignored(_)) {
            return None;
        }

        if !ctx.pathspec.directory_matches_prefix(dir_rela_path, false) {
            return None;
        }

        // Pathspecs affect the collapse of the next level, hence find the highest-value one.
        let dir_pathspec_match = state.on_hold[self.start_index..]
            .iter()
            .filter_map(|e| e.pathspec_match)
            .max()
            .or_else(|| {
                // Only take directory matches for value if they are above the 'guessed' ones.
                // Otherwise we end up with seemingly matched entries in the parent directory which
                // affects proper folding.
                dir_info
                    .pathspec_match
                    .filter(|m| matches!(m, PathspecMatch::WildcardMatch | PathspecMatch::Verbatim))
            });
        let mut removed_without_emitting = 0;
        let mut action = Action::Continue;
        for entry in state.on_hold.drain(self.start_index..) {
            if entry.status != dir_status && action == Action::Continue {
                let info = classify::Outcome::from(&entry);
                action = emit_entry(Cow::Owned(entry.rela_path), info, Some(dir_status), opts, out, delegate);
            } else {
                removed_without_emitting += 1;
            };
        }
        out.seen_entries += removed_without_emitting as u32;

        state.on_hold.push(Entry {
            rela_path: dir_rela_path.to_owned(),
            status: dir_status,
            disk_kind: dir_info.disk_kind,
            index_kind: dir_info.index_kind,
            pathspec_match: dir_pathspec_match,
        });
        Some(action)
    }
}

impl Options {
    fn should_hold(&self, status: entry::Status) -> bool {
        if status.is_pruned() {
            return false;
        }
        self.emit_ignored == Some(CollapseDirectory) || self.emit_untracked == CollapseDirectory
    }
}
