use crate::{entry, Entry};

use crate::entry::PathspecMatch;
use crate::walk::{Context, Error, ForDeletionMode, Options};
use bstr::{BStr, BString, ByteSlice};
use std::ffi::OsStr;
use std::path::{Component, Path, PathBuf};

/// Classify the `worktree_relative_root` path and return the first `PathKind` that indicates that
/// it isn't a directory, leaving `buf` with the path matching the returned `PathKind`,
/// which is at most equal to `worktree_relative_root`.
pub fn root(
    worktree_root: &Path,
    buf: &mut BString,
    worktree_relative_root: &Path,
    options: Options,
    ctx: &mut Context<'_>,
) -> Result<Outcome, Error> {
    buf.clear();
    let mut last_length = None;
    let mut path_buf = worktree_root.to_owned();
    // These initial values kick in if worktree_relative_root.is_empty();
    let mut out = None;
    for component in worktree_relative_root
        .components()
        .chain(if worktree_relative_root.as_os_str().is_empty() {
            Some(Component::Normal(OsStr::new("")))
        } else {
            None
        })
    {
        if last_length.is_some() {
            buf.push(b'/');
        }
        path_buf.push(component);
        buf.extend_from_slice(gix_path::os_str_into_bstr(component.as_os_str()).expect("no illformed UTF8"));
        let file_kind = path_buf.symlink_metadata().map(|m| m.file_type().into()).ok();

        let res = path(
            &mut path_buf,
            buf,
            last_length.map(|l| l + 1 /* slash */).unwrap_or_default(),
            file_kind,
            || None,
            options,
            ctx,
        )?;
        out = Some(res);
        if !res
            .status
            .can_recurse(res.disk_kind, res.pathspec_match, options.for_deletion)
        {
            break;
        }
        last_length = Some(buf.len());
    }
    Ok(out.expect("One iteration of the loop at least"))
}
/// The product of [`path()`] calls.
#[derive(Debug, Copy, Clone)]
pub struct Outcome {
    /// The computed status of an entry. It can be seen as aggregate of things we know about an entry.
    pub status: entry::Status,
    /// What the entry is on disk, or `None` if we aborted the classification early.
    ///
    /// Note that the index is used to avoid disk access provided its entries are marked uptodate
    /// (possibly by a prior call to update the status).
    pub disk_kind: Option<entry::Kind>,
    /// What the entry looks like in the index, or `None` if we aborted early.
    pub index_kind: Option<entry::Kind>,
    /// If a pathspec matched, this is how it matched. Maybe `None` if computation didn't see the need to evaluate it.
    pub pathspec_match: Option<PathspecMatch>,
}

impl Outcome {
    fn with_status(mut self, status: entry::Status) -> Self {
        self.status = status;
        self
    }

    fn with_kind(mut self, disk_kind: Option<entry::Kind>, index_kind: Option<entry::Kind>) -> Self {
        self.disk_kind = disk_kind;
        self.index_kind = index_kind;
        self
    }
}

impl From<&Entry> for Outcome {
    fn from(e: &Entry) -> Self {
        Outcome {
            status: e.status,
            disk_kind: e.disk_kind,
            index_kind: e.index_kind,
            pathspec_match: e.pathspec_match,
        }
    }
}

/// Figure out what to do with `rela_path`, provided as worktree-relative path, with `disk_file_type` if it is known already
/// as it helps to match pathspecs correctly, which can be different for directories.
/// `path` is a disk-accessible variant of `rela_path` which is within the `worktree_root`, and will be modified temporarily but remain unchanged.
///
/// Note that `rela_path` is used as buffer for convenience, but will be left as is when this function returns.
/// `filename_start_idx` is the index at which the filename begins, i.e. `a/b` has `2` as index.
/// It may resemble a directory on the way to a leaf (like a file)
///
/// Returns `(status, file_kind, pathspec_matches_how)` to identify the `status` on disk, along with a classification `file_kind`,
/// and if `file_kind` is not a directory, the way the pathspec matched with `pathspec_matches_how`.
pub fn path(
    path: &mut PathBuf,
    rela_path: &mut BString,
    filename_start_idx: usize,
    disk_kind: Option<entry::Kind>,
    on_demand_disk_kind: impl FnOnce() -> Option<entry::Kind>,
    Options {
        ignore_case,
        recurse_repositories,
        emit_ignored,
        for_deletion,
        classify_untracked_bare_repositories,
        ..
    }: Options,
    ctx: &mut Context<'_>,
) -> Result<Outcome, Error> {
    let mut out = Outcome {
        status: entry::Status::DotGit,
        disk_kind,
        index_kind: None,
        pathspec_match: None,
    };
    if is_eq(rela_path[filename_start_idx..].as_bstr(), ".git", ignore_case) {
        return Ok(out);
    }
    let pathspec_could_match = rela_path.is_empty()
        || ctx
            .pathspec
            .can_match_relative_path(rela_path.as_bstr(), disk_kind.map(|ft| ft.is_dir()));
    if !pathspec_could_match {
        return Ok(out.with_status(entry::Status::Pruned));
    }

    let (uptodate_index_kind, index_kind, mut maybe_status) = resolve_file_type_with_index(
        rela_path,
        ctx.index,
        ctx.ignore_case_index_lookup.filter(|_| ignore_case),
    );
    let mut kind = uptodate_index_kind.or(disk_kind).or_else(on_demand_disk_kind);

    maybe_status = maybe_status
        .or_else(|| (index_kind.map(|k| k.is_dir()) == kind.map(|k| k.is_dir())).then_some(entry::Status::Tracked));

    // We always check the pathspec to have the value filled in reliably.
    out.pathspec_match = ctx
        .pathspec
        .pattern_matching_relative_path(
            rela_path.as_bstr(),
            disk_kind.map(|ft| ft.is_dir()),
            ctx.pathspec_attributes,
        )
        .map(|m| {
            if m.is_excluded() {
                PathspecMatch::Excluded
            } else {
                m.kind.into()
            }
        });

    let mut maybe_upgrade_to_repository = |current_kind, find_harder: bool| {
        if recurse_repositories {
            return current_kind;
        }
        if find_harder {
            let mut is_nested_repo = gix_discover::is_git(path).is_ok();
            if is_nested_repo {
                let git_dir_is_our_own =
                    gix_path::realpath_opts(path, ctx.current_dir, gix_path::realpath::MAX_SYMLINKS)
                        .ok()
                        .map_or(false, |realpath_candidate| realpath_candidate == ctx.git_dir_realpath);
                is_nested_repo = !git_dir_is_our_own;
            }
            if is_nested_repo {
                return Some(entry::Kind::Repository);
            }
        }
        path.push(gix_discover::DOT_GIT_DIR);
        let mut is_nested_nonbare_repo = gix_discover::is_git(path).is_ok();
        if is_nested_nonbare_repo {
            let git_dir_is_our_own = gix_path::realpath_opts(path, ctx.current_dir, gix_path::realpath::MAX_SYMLINKS)
                .ok()
                .map_or(false, |realpath_candidate| realpath_candidate == ctx.git_dir_realpath);
            is_nested_nonbare_repo = !git_dir_is_our_own;
        }
        path.pop();

        if is_nested_nonbare_repo {
            Some(entry::Kind::Repository)
        } else {
            current_kind
        }
    };
    if let Some(status) = maybe_status {
        if kind == Some(entry::Kind::Directory) && index_kind == Some(entry::Kind::Repository) {
            kind = maybe_upgrade_to_repository(kind, false);
        }
        return Ok(out.with_status(status).with_kind(kind, index_kind));
    }

    debug_assert!(maybe_status.is_none(), "It only communicates a single stae right now");
    if let Some(excluded) = ctx
        .excludes
        .as_mut()
        .map_or(Ok(None), |stack| {
            stack
                .at_entry(rela_path.as_bstr(), kind.map(|ft| ft.is_dir()), ctx.objects)
                .map(|platform| platform.excluded_kind())
        })
        .map_err(Error::ExcludesAccess)?
    {
        if emit_ignored.is_some() {
            if kind.map_or(false, |d| d.is_dir()) && out.pathspec_match.is_none() {
                // we have patterns that didn't match at all. Try harder.
                out.pathspec_match = ctx
                    .pathspec
                    .directory_matches_prefix(rela_path.as_bstr(), true)
                    .then_some(PathspecMatch::Prefix);
            }
            if matches!(
                for_deletion,
                Some(
                    ForDeletionMode::FindNonBareRepositoriesInIgnoredDirectories
                        | ForDeletionMode::FindRepositoriesInIgnoredDirectories
                )
            ) {
                kind = maybe_upgrade_to_repository(
                    kind,
                    matches!(
                        for_deletion,
                        Some(ForDeletionMode::FindRepositoriesInIgnoredDirectories)
                    ),
                );
            }
        }
        return Ok(out
            .with_status(entry::Status::Ignored(excluded))
            .with_kind(kind, index_kind));
    }

    debug_assert!(maybe_status.is_none());
    let mut status = entry::Status::Untracked;

    if kind.map_or(false, |ft| ft.is_dir()) {
        kind = maybe_upgrade_to_repository(kind, classify_untracked_bare_repositories);
    } else if out.pathspec_match.is_none() {
        status = entry::Status::Pruned;
    }
    Ok(out.with_status(status).with_kind(kind, index_kind))
}

/// Note that `rela_path` is used as buffer for convenience, but will be left as is when this function returns.
/// Also note `maybe_file_type` will be `None` for entries that aren't up-to-date and files, for directories at least one entry must be uptodate.
/// Returns `(maybe_file_type, Option<index_file_type>, Option(TrackedExcluded)`, with the last option being set only for sparse directories.
/// `tracked_exclued` indicates it's a sparse directory was found.
/// `index_file_type` is the type of `rela_path` as available in the index.
///
/// ### Shortcoming
///
/// In case-insensitive mode, if there is an entry `d` and a `D/a` both in the index, we will always find the file `d` first, and always consider
/// the entry as not uptodate, while classifying it as file (the first one we found). As quite a huge exception, this isn't properly represented
/// in the data model, and we emit a trace to make it more obvious when it happens, in case this leads to less expected results.
fn resolve_file_type_with_index(
    rela_path: &mut BString,
    index: &gix_index::State,
    ignore_case: Option<&gix_index::AccelerateLookup<'_>>,
) -> (Option<entry::Kind>, Option<entry::Kind>, Option<entry::Status>) {
    // TODO: either get this to work for icase as well, or remove the need for it. Logic is different in both branches.
    let mut special_status = None;

    fn entry_to_kinds(entry: &gix_index::Entry) -> (Option<entry::Kind>, Option<entry::Kind>) {
        let kind = if entry.mode.is_submodule() {
            entry::Kind::Repository.into()
        } else if entry.mode.contains(gix_index::entry::Mode::FILE) {
            entry::Kind::File.into()
        } else if entry.mode.contains(gix_index::entry::Mode::SYMLINK) {
            entry::Kind::Symlink.into()
        } else {
            None
        };
        (
            kind.filter(|_| entry.flags.contains(gix_index::entry::Flags::UPTODATE)),
            kind,
        )
    }

    fn icase_directory_to_kinds(dir: Option<&gix_index::Entry>) -> (Option<entry::Kind>, Option<entry::Kind>) {
        let index_kind = dir.map(|_| entry::Kind::Directory);
        let uptodate_kind = dir
            .filter(|entry| entry.flags.contains(gix_index::entry::Flags::UPTODATE))
            .map(|_| entry::Kind::Directory);
        (uptodate_kind, index_kind)
    }

    // TODO(perf): multi-threaded hash-table so it's always used, even for case-sensitive lookups, just like Git does it.
    let (uptodate_kind, index_kind) = if let Some(accelerate) = ignore_case {
        match index.entry_by_path_icase(rela_path.as_bstr(), true, accelerate) {
            None => {
                icase_directory_to_kinds(index.entry_closest_to_directory_icase(rela_path.as_bstr(), true, accelerate))
            }
            Some(entry) => {
                let icase_dir = index.entry_closest_to_directory_icase(rela_path.as_bstr(), true, accelerate);
                let directory_matches_exactly = icase_dir.map_or(false, |dir| {
                    let path = dir.path(index);
                    let slash_idx = path.rfind_byte(b'/').expect("dir");
                    path[..slash_idx].as_bstr() == rela_path
                });
                if directory_matches_exactly {
                    icase_directory_to_kinds(icase_dir)
                } else {
                    entry_to_kinds(entry)
                }
            }
        }
    } else {
        match index.entry_by_path(rela_path.as_bstr()) {
            None => {
                rela_path.push(b'/');
                let res = index.prefixed_entries_range(rela_path.as_bstr());
                rela_path.pop();

                let mut one_index_signalling_with_cone = None;
                let mut all_excluded_from_worktree_non_cone = false;
                let is_tracked = res.is_some();
                let kind = res
                    .filter(|range| {
                        if range.len() == 1 {
                            one_index_signalling_with_cone = range.start.into();
                        }
                        let entries = &index.entries()[range.clone()];
                        let any_up_to_date = entries
                            .iter()
                            .any(|e| e.flags.contains(gix_index::entry::Flags::UPTODATE));
                        if !any_up_to_date && one_index_signalling_with_cone.is_none() {
                            all_excluded_from_worktree_non_cone = entries
                                .iter()
                                .all(|e| e.flags.contains(gix_index::entry::Flags::SKIP_WORKTREE));
                        }
                        any_up_to_date
                    })
                    .map(|_| entry::Kind::Directory);

                if all_excluded_from_worktree_non_cone
                    || one_index_signalling_with_cone
                        .filter(|_| kind.is_none())
                        .map_or(false, |idx| index.entries()[idx].mode.is_sparse())
                {
                    special_status = Some(entry::Status::TrackedExcluded);
                }
                (kind, is_tracked.then_some(entry::Kind::Directory))
            }
            Some(entry) => entry_to_kinds(entry),
        }
    };
    (uptodate_kind, index_kind, special_status)
}

fn is_eq(lhs: &BStr, rhs: impl AsRef<BStr>, ignore_case: bool) -> bool {
    if ignore_case {
        lhs.eq_ignore_ascii_case(rhs.as_ref().as_ref())
    } else {
        lhs == rhs.as_ref()
    }
}
