use bstr::BStr;
use gix_dir::{entry, walk, Entry};
use gix_testtools::scripted_fixture_read_only;
use std::path::{Path, PathBuf};

pub fn fixture_in(filename: &str, name: &str) -> PathBuf {
    let root = scripted_fixture_read_only(format!("{filename}.sh")).expect("script works");
    root.join(name)
}

pub fn fixture(name: &str) -> PathBuf {
    fixture_in("many", name)
}

/// Default options
pub fn options() -> walk::Options {
    walk::Options::default()
}

/// Default options
pub fn options_emit_all() -> walk::Options {
    walk::Options {
        precompose_unicode: false,
        ignore_case: false,
        recurse_repositories: false,
        for_deletion: None,
        classify_untracked_bare_repositories: false,
        emit_pruned: true,
        emit_ignored: Some(walk::EmissionMode::Matching),
        emit_tracked: true,
        emit_untracked: walk::EmissionMode::Matching,
        emit_empty_directories: true,
    }
}

pub fn entry(
    rela_path: impl AsRef<BStr>,
    status: entry::Status,
    disk_kind: entry::Kind,
) -> (Entry, Option<entry::Status>) {
    entryps(rela_path, status, disk_kind, entry::PathspecMatch::Always)
}

pub fn entry_nomatch(
    rela_path: impl AsRef<BStr>,
    status: entry::Status,
    disk_kind: entry::Kind,
) -> (Entry, Option<entry::Status>) {
    (
        Entry {
            rela_path: rela_path.as_ref().to_owned(),
            status,
            disk_kind: Some(disk_kind),
            index_kind: index_kind_from_status(status, disk_kind),
            pathspec_match: None,
        },
        None,
    )
}

pub fn entry_nokind(rela_path: impl AsRef<BStr>, status: entry::Status) -> (Entry, Option<entry::Status>) {
    (
        Entry {
            rela_path: rela_path.as_ref().to_owned(),
            status,
            disk_kind: None,
            index_kind: None,
            pathspec_match: None,
        },
        None,
    )
}

pub fn entryps(
    rela_path: impl AsRef<BStr>,
    status: entry::Status,
    disk_kind: entry::Kind,
    pathspec_match: entry::PathspecMatch,
) -> (Entry, Option<entry::Status>) {
    (
        Entry {
            rela_path: rela_path.as_ref().to_owned(),
            status,
            disk_kind: Some(disk_kind),
            index_kind: index_kind_from_status(status, disk_kind),
            pathspec_match: Some(pathspec_match),
        },
        None,
    )
}

pub fn entry_dirstat(
    rela_path: impl AsRef<BStr>,
    status: entry::Status,
    disk_kind: entry::Kind,
    dir_status: entry::Status,
) -> (Entry, Option<entry::Status>) {
    (
        Entry {
            rela_path: rela_path.as_ref().to_owned(),
            status,
            disk_kind: Some(disk_kind),
            index_kind: index_kind_from_status(status, disk_kind),
            pathspec_match: Some(entry::PathspecMatch::Always),
        },
        Some(dir_status),
    )
}

/// These are entries that have been collapsed into a single directory.
pub fn entryps_dirstat(
    rela_path: impl AsRef<BStr>,
    status: entry::Status,
    disk_kind: entry::Kind,
    pathspec_match: entry::PathspecMatch,
    dir_status: entry::Status,
) -> (Entry, Option<entry::Status>) {
    (
        Entry {
            rela_path: rela_path.as_ref().to_owned(),
            status,
            disk_kind: Some(disk_kind),
            index_kind: index_kind_from_status(status, disk_kind),
            pathspec_match: Some(pathspec_match),
        },
        Some(dir_status),
    )
}

fn index_kind_from_status(status: entry::Status, disk_kind: entry::Kind) -> Option<entry::Kind> {
    matches!(status, entry::Status::Tracked | entry::Status::TrackedExcluded).then_some(disk_kind)
}

pub trait EntryExt {
    fn with_index_kind(self, index_kind: entry::Kind) -> Self;
}

impl EntryExt for (Entry, Option<entry::Status>) {
    fn with_index_kind(mut self, index_kind: entry::Kind) -> Self {
        self.0.index_kind = index_kind.into();
        self
    }
}

pub fn collect(
    worktree_root: &Path,
    cb: impl FnOnce(&mut dyn walk::Delegate, walk::Context) -> Result<walk::Outcome, walk::Error>,
) -> (walk::Outcome, Entries) {
    try_collect(worktree_root, cb).unwrap()
}

pub fn collect_filtered(
    worktree_root: &Path,
    cb: impl FnOnce(&mut dyn walk::Delegate, walk::Context) -> Result<walk::Outcome, walk::Error>,
    patterns: impl IntoIterator<Item = impl AsRef<BStr>>,
) -> (walk::Outcome, Entries) {
    try_collect_filtered(worktree_root, cb, patterns).unwrap()
}

pub fn try_collect(
    worktree_root: &Path,
    cb: impl FnOnce(&mut dyn walk::Delegate, walk::Context) -> Result<walk::Outcome, walk::Error>,
) -> Result<(walk::Outcome, Entries), walk::Error> {
    try_collect_filtered(worktree_root, cb, None::<&str>)
}

pub fn try_collect_filtered(
    worktree_root: &Path,
    cb: impl FnOnce(&mut dyn walk::Delegate, walk::Context) -> Result<walk::Outcome, walk::Error>,
    patterns: impl IntoIterator<Item = impl AsRef<BStr>>,
) -> Result<(walk::Outcome, Entries), walk::Error> {
    try_collect_filtered_opts(worktree_root, cb, patterns, Default::default())
}

pub fn try_collect_filtered_opts(
    worktree_root: &Path,
    cb: impl FnOnce(&mut dyn walk::Delegate, walk::Context) -> Result<walk::Outcome, walk::Error>,
    patterns: impl IntoIterator<Item = impl AsRef<BStr>>,
    Options { fresh_index, git_dir }: Options<'_>,
) -> Result<(walk::Outcome, Entries), walk::Error> {
    let git_dir = worktree_root.join(git_dir.unwrap_or(".git"));
    let mut index = std::fs::read(git_dir.join("index")).ok().map_or_else(
        || gix_index::State::new(gix_index::hash::Kind::Sha1),
        |bytes| {
            gix_index::State::from_bytes(
                &bytes,
                std::time::UNIX_EPOCH.into(),
                gix_index::hash::Kind::Sha1,
                Default::default(),
            )
            .map(|t| t.0)
            .expect("valid index")
        },
    );
    if fresh_index {
        index
            .entries_mut()
            .iter_mut()
            .filter(|e| {
                // relevant for partial checkouts, all related entries will have skip-worktree set,
                // which also means they will never be up-to-date.
                !e.flags.contains(gix_index::entry::Flags::SKIP_WORKTREE)
            })
            .for_each(|e| {
                // pretend that the index was refreshed beforehand so we know what's uptodate.
                e.flags |= gix_index::entry::Flags::UPTODATE;
            });
    }
    let mut search = gix_pathspec::Search::from_specs(
        patterns.into_iter().map(|spec| {
            gix_pathspec::parse(spec.as_ref(), gix_pathspec::Defaults::default()).expect("tests use valid pattern")
        }),
        None,
        "we don't provide absolute pathspecs, thus need no worktree root".as_ref(),
    )
    .expect("search creation can't fail");
    let mut stack = gix_worktree::Stack::from_state_and_ignore_case(
        worktree_root,
        false, /* ignore case */
        gix_worktree::stack::State::IgnoreStack(gix_worktree::stack::state::Ignore::new(
            Default::default(),
            Default::default(),
            None,
            gix_worktree::stack::state::ignore::Source::WorktreeThenIdMappingIfNotSkipped,
        )),
        &index,
        index.path_backing(),
    );

    let cwd = gix_fs::current_dir(false).expect("valid cwd");
    let git_dir_realpath = gix_path::realpath_opts(&git_dir, &cwd, gix_path::realpath::MAX_SYMLINKS).unwrap();
    let mut dlg = gix_dir::walk::delegate::Collect::default();
    let lookup = index.prepare_icase_backing();
    let outcome = cb(
        &mut dlg,
        walk::Context {
            git_dir_realpath: &git_dir_realpath,
            current_dir: &cwd,
            index: &index,
            ignore_case_index_lookup: Some(&lookup),
            pathspec: &mut search,
            pathspec_attributes: &mut |_, _, _, _| panic!("we do not use pathspecs that require attributes access."),
            excludes: Some(&mut stack),
            objects: &gix_object::find::Never,
        },
    )?;

    Ok((outcome, dlg.into_entries_by_path()))
}

pub struct Options<'a> {
    pub fresh_index: bool,
    pub git_dir: Option<&'a str>,
}

impl<'a> Options<'a> {
    pub fn git_dir(dir: &'a str) -> Self {
        Options {
            git_dir: Some(dir),
            ..Default::default()
        }
    }
}

impl<'a> Default for Options<'a> {
    fn default() -> Self {
        Options {
            fresh_index: true,
            git_dir: None,
        }
    }
}

type Entries = Vec<(Entry, Option<entry::Status>)>;
