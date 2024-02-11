use std::borrow::Cow;
use std::path::{Path, PathBuf};

use bstr::{BStr, BString, ByteSlice};

use crate::walk::{classify, readdir, Action, Context, Delegate, Error, ForDeletionMode, Options, Outcome};
use crate::{entry, EntryRef};

/// A function to perform a git-style, unsorted, directory walk.
///
/// * `root` - the starting point of the walk and a readable directory.
///     - Note that if the path leading to this directory or `root` itself is excluded, it will be provided to [`Delegate::emit()`]
///       without further traversal.
///     - If [`Options::precompose_unicode`] is enabled, this path must be precomposed.
///     - Must be contained in `worktree_root`.
/// * `worktree_root` - the top-most root of the worktree, which must be a prefix to `root`.
///     - If [`Options::precompose_unicode`] is enabled, this path must be precomposed.
/// * `ctx` - everything needed to classify the paths seen during the traversal.
/// * `delegate` - an implementation of [`Delegate`] to control details of the traversal and receive its results.
///
/// ### Performance Notes
///
/// In theory, parallel directory traversal can be significantly faster, and what's possible for our current
/// `gix_features::fs::WalkDir` implementation is to abstract a `filter_entry()` method so it works both for
/// the iterator from the `walkdir` crate as well as from `jwalk`. However, doing so as initial version
/// has the risk of not being significantly harder if not impossible to implement as flow-control is very
/// limited.
///
/// Thus the decision was made to start out with something akin to the Git implementation, get all tests and
/// baseline comparison to pass, and see if an iterator with just `filter_entry` would be capable of dealing with
/// it. Note that `filter_entry` is the only just-in-time traversal control that `walkdir` offers, even though
/// one could consider switching to `jwalk` and just use its single-threaded implementation if a unified interface
/// is necessary to make this work - `jwalk` has a more powerful API for this to work.
///
/// If that was the case, we are talking about 0.5s for single-threaded traversal (without doing any extra work)
/// or 0.25s for optimal multi-threaded performance, all in the WebKit directory with 388k items to traverse.
/// Thus, the speedup could easily be 2x or more and thus worth investigating in due time.
pub fn walk(
    root: &Path,
    worktree_root: &Path,
    mut ctx: Context<'_>,
    options: Options,
    delegate: &mut dyn Delegate,
) -> Result<Outcome, Error> {
    let _span = gix_trace::coarse!("walk", root = ?root, worktree_root = ?worktree_root, options = ?options);
    let (mut current, worktree_root_relative) = assure_no_symlink_in_root(worktree_root, root)?;
    let mut out = Outcome::default();
    let mut buf = BString::default();
    let root_info = classify::root(
        worktree_root,
        &mut buf,
        worktree_root_relative.as_ref(),
        options,
        &mut ctx,
    )?;
    if !can_recurse(buf.as_bstr(), root_info, options.for_deletion, delegate) {
        if buf.is_empty() && !matches!(root_info.disk_kind, Some(entry::Kind::Directory { .. })) {
            return Err(Error::WorktreeRootIsFile { root: root.to_owned() });
        }
        if options.precompose_unicode {
            buf = gix_utils::str::precompose_bstr(buf.into()).into_owned();
        }
        let _ = emit_entry(
            Cow::Borrowed(buf.as_bstr()),
            root_info,
            None,
            options,
            &mut out,
            delegate,
        );
        return Ok(out);
    }

    let mut state = readdir::State::default();
    let _ = readdir::recursive(
        root == worktree_root,
        &mut current,
        &mut buf,
        root_info,
        &mut ctx,
        options,
        delegate,
        &mut out,
        &mut state,
    )?;
    assert_eq!(state.on_hold.len(), 0, "BUG: must be fully consumed");
    gix_trace::debug!(statistics = ?out);
    Ok(out)
}

/// Note that we only check symlinks on the way from `worktree_root` to `root`,
/// so `worktree_root` may go through a symlink.
/// Returns `(worktree_root, normalized_worktree_relative_root)`.
fn assure_no_symlink_in_root<'root>(
    worktree_root: &Path,
    root: &'root Path,
) -> Result<(PathBuf, Cow<'root, Path>), Error> {
    let mut current = worktree_root.to_owned();
    let worktree_relative = root.strip_prefix(worktree_root).map_err(|_| Error::RootNotInWorktree {
        worktree_root: worktree_root.to_owned(),
        root: root.to_owned(),
    })?;
    let worktree_relative = gix_path::normalize(worktree_relative.into(), Path::new(""))
        .ok_or(Error::NormalizeRoot { root: root.to_owned() })?;

    for (idx, component) in worktree_relative.components().enumerate() {
        current.push(component);
        let meta = current.symlink_metadata().map_err(|err| Error::SymlinkMetadata {
            source: err,
            path: current.to_owned(),
        })?;
        if meta.is_symlink() {
            return Err(Error::SymlinkInRoot {
                root: root.to_owned(),
                worktree_root: worktree_root.to_owned(),
                component_index: idx,
            });
        }
    }
    Ok((current, worktree_relative))
}

pub(super) fn can_recurse(
    rela_path: &BStr,
    info: classify::Outcome,
    for_deletion: Option<ForDeletionMode>,
    delegate: &mut dyn Delegate,
) -> bool {
    if info.disk_kind.map_or(true, |k| !k.is_dir()) {
        return false;
    }
    let entry = EntryRef {
        rela_path: Cow::Borrowed(rela_path),
        status: info.status,
        disk_kind: info.disk_kind,
        index_kind: info.index_kind,
        pathspec_match: info.pathspec_match,
    };
    delegate.can_recurse(entry, for_deletion)
}

/// Possibly emit an entry to `for_each` in case the provided information makes that possible.
#[allow(clippy::too_many_arguments)]
pub(super) fn emit_entry(
    rela_path: Cow<'_, BStr>,
    info: classify::Outcome,
    dir_status: Option<entry::Status>,
    Options {
        emit_pruned,
        emit_tracked,
        emit_ignored,
        emit_empty_directories,
        ..
    }: Options,
    out: &mut Outcome,
    delegate: &mut dyn Delegate,
) -> Action {
    out.seen_entries += 1;

    if (!emit_empty_directories && info.disk_kind == Some(entry::Kind::EmptyDirectory)
        || !emit_tracked && info.status == entry::Status::Tracked)
        || emit_ignored.is_none() && matches!(info.status, entry::Status::Ignored(_))
        || !emit_pruned
            && (info.status.is_pruned()
                || info
                    .pathspec_match
                    .map_or(true, |m| m == entry::PathspecMatch::Excluded))
    {
        return Action::Continue;
    }

    out.returned_entries += 1;
    delegate.emit(
        EntryRef {
            rela_path,
            status: info.status,
            disk_kind: info.disk_kind,
            index_kind: info.index_kind,
            pathspec_match: info.pathspec_match,
        },
        dir_status,
    )
}
