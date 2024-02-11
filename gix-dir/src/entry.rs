use crate::walk::ForDeletionMode;
use crate::{Entry, EntryRef};
use std::borrow::Cow;

/// The kind of the entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum Kind {
    /// The entry is a blob, executable or not.
    File,
    /// The entry is a symlink.
    Symlink,
    /// A directory that contains no file or directory.
    EmptyDirectory,
    /// The entry is an ordinary directory.
    ///
    /// Note that since we don't check for bare repositories, this could in fact be a collapsed
    /// bare repository. To be sure, check it again with [`gix_discover::is_git()`] and act accordingly.
    Directory,
    /// The entry is a directory which *contains* a `.git` folder.
    Repository,
}

/// The kind of entry as obtained from a directory.
///
/// The order of variants roughly relates from cheap-to-compute to most expensive, as each level needs more tests to assert.
/// Thus, `DotGit` is the cheapest, while `Untracked` is among the most expensive and one of the major outcomes of any
/// [`walk`](crate::walk()) run.
/// For example, if an entry was `Pruned`, we effectively don't know if it would have been `Untracked` as well as we stopped looking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum Status {
    /// The filename of an entry was `.git`, which is generally pruned.
    DotGit,
    /// The provided pathspec prevented further processing as the path didn't match.
    /// If this happens, no further checks are done so we wouldn't know if the path is also ignored for example (by mention in `.gitignore`).
    Pruned,
    /// Always in conjunction with a directory on disk that is also known as cone-mode sparse-checkout exclude marker - i.e. a directory
    /// that is excluded, so its whole content is excluded and not checked out nor is part of the index.
    TrackedExcluded,
    /// The entry is tracked in Git.
    Tracked,
    /// The entry is ignored as per `.gitignore` files and their rules.
    ///
    /// If this is a directory, then its entire contents is ignored. Otherwise, possibly due to configuration, individual ignored files are listed.
    Ignored(gix_ignore::Kind),
    /// The entry is not tracked by git yet, it was not found in the [index](gix_index::State).
    ///
    /// If it's a directory, the entire directory contents is untracked.
    Untracked,
}

/// Describe how a pathspec pattern matched.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash, Ord, PartialOrd)]
pub enum PathspecMatch {
    /// The match happened because there wasn't any pattern, which matches all, or because there was a nil pattern or one with an empty path.
    /// Thus this is not a match by merit.
    Always,
    /// A match happened, but the pattern excludes everything it matches, which means this entry was excluded.
    Excluded,
    /// The first part of a pathspec matches, like `dir/` that matches `dir/a`.
    Prefix,
    /// The whole pathspec matched and used a wildcard match, like `a/*` matching `a/file`.
    WildcardMatch,
    /// The entire pathspec matched, letter by letter, e.g. `a/file` matching `a/file`.
    Verbatim,
}

impl PathspecMatch {
    pub(crate) fn should_ignore(&self) -> bool {
        match self {
            PathspecMatch::Always | PathspecMatch::Excluded => true,
            PathspecMatch::Prefix | PathspecMatch::WildcardMatch | PathspecMatch::Verbatim => false,
        }
    }
}

impl From<gix_pathspec::search::MatchKind> for PathspecMatch {
    fn from(kind: gix_pathspec::search::MatchKind) -> Self {
        match kind {
            gix_pathspec::search::MatchKind::Always => Self::Always,
            gix_pathspec::search::MatchKind::Prefix => Self::Prefix,
            gix_pathspec::search::MatchKind::WildcardMatch => Self::WildcardMatch,
            gix_pathspec::search::MatchKind::Verbatim => Self::Verbatim,
        }
    }
}

impl EntryRef<'_> {
    /// Strip the lifetime to obtain a fully owned copy.
    pub fn to_owned(&self) -> Entry {
        Entry {
            rela_path: self.rela_path.clone().into_owned(),
            status: self.status,
            disk_kind: self.disk_kind,
            index_kind: self.index_kind,
            pathspec_match: self.pathspec_match,
        }
    }

    /// Turn this instance into a fully owned copy.
    pub fn into_owned(self) -> Entry {
        Entry {
            rela_path: self.rela_path.into_owned(),
            status: self.status,
            disk_kind: self.disk_kind,
            index_kind: self.index_kind,
            pathspec_match: self.pathspec_match,
        }
    }
}

impl Entry {
    /// Obtain an [`EntryRef`] from this instance.
    pub fn to_ref(&self) -> EntryRef<'_> {
        EntryRef {
            rela_path: Cow::Borrowed(self.rela_path.as_ref()),
            status: self.status,
            disk_kind: self.disk_kind,
            index_kind: self.index_kind,
            pathspec_match: self.pathspec_match,
        }
    }
}

impl From<std::fs::FileType> for Kind {
    fn from(value: std::fs::FileType) -> Self {
        if value.is_dir() {
            Kind::Directory
        } else if value.is_symlink() {
            Kind::Symlink
        } else {
            Kind::File
        }
    }
}

impl Status {
    /// Return true if this status is considered pruned. A pruned entry is typically hidden from view due to a pathspec.
    pub fn is_pruned(&self) -> bool {
        match self {
            Status::DotGit | Status::TrackedExcluded | Status::Pruned => true,
            Status::Ignored(_) | Status::Untracked | Status::Tracked => false,
        }
    }
    /// Return `true` if `file_type` is a directory on disk and isn't ignored, and is not a repository.
    /// This implements the default rules of `git status`, which is good for a minimal traversal through
    /// tracked and non-ignored portions of a worktree.
    /// `for_deletion` is used to determine if recursion into a directory is allowed even though it otherwise wouldn't be.
    ///
    /// Use `pathspec_match` to determine if a pathspec matches in any way, affecting the decision to recurse.
    pub fn can_recurse(
        &self,
        file_type: Option<Kind>,
        pathspec_match: Option<PathspecMatch>,
        for_deletion: Option<ForDeletionMode>,
    ) -> bool {
        let is_dir_on_disk = file_type.map_or(false, |ft| ft.is_recursable_dir());
        if !is_dir_on_disk {
            return false;
        }
        match self {
            Status::DotGit | Status::TrackedExcluded | Status::Pruned => false,
            Status::Ignored(_) => {
                for_deletion.map_or(false, |fd| {
                    matches!(
                        fd,
                        ForDeletionMode::FindNonBareRepositoriesInIgnoredDirectories
                            | ForDeletionMode::FindRepositoriesInIgnoredDirectories
                    )
                }) || pathspec_match.map_or(false, |m| !m.should_ignore())
            }
            Status::Untracked | Status::Tracked => true,
        }
    }
}

impl Kind {
    fn is_recursable_dir(&self) -> bool {
        matches!(self, Kind::Directory)
    }

    /// Return `true` if this is a directory on disk. Note that this is true for repositories as well.
    pub fn is_dir(&self) -> bool {
        matches!(self, Kind::EmptyDirectory | Kind::Directory | Kind::Repository)
    }
}
