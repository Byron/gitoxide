use gix_dir::walk::{EmissionMode, ForDeletionMode};

/// Options for use in the [`Repository::dirwalk()`](crate::Repository::dirwalk()) function.
///
/// Note that all values start out disabled.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Options {
    precompose_unicode: bool,
    ignore_case: bool,

    recurse_repositories: bool,
    emit_pruned: bool,
    emit_ignored: Option<EmissionMode>,
    for_deletion: Option<ForDeletionMode>,
    emit_tracked: bool,
    emit_untracked: EmissionMode,
    emit_empty_directories: bool,
    classify_untracked_bare_repositories: bool,
}

/// Construction
impl Options {
    pub(crate) fn from_fs_caps(caps: gix_fs::Capabilities) -> Self {
        Self {
            precompose_unicode: caps.precompose_unicode,
            ignore_case: caps.ignore_case,
            recurse_repositories: false,
            emit_pruned: false,
            emit_ignored: None,
            for_deletion: None,
            emit_tracked: false,
            emit_untracked: Default::default(),
            emit_empty_directories: false,
            classify_untracked_bare_repositories: false,
        }
    }
}

impl From<Options> for gix_dir::walk::Options {
    fn from(v: Options) -> Self {
        gix_dir::walk::Options {
            precompose_unicode: v.precompose_unicode,
            ignore_case: v.ignore_case,
            recurse_repositories: v.recurse_repositories,
            emit_pruned: v.emit_pruned,
            emit_ignored: v.emit_ignored,
            for_deletion: v.for_deletion,
            emit_tracked: v.emit_tracked,
            emit_untracked: v.emit_untracked,
            emit_empty_directories: v.emit_empty_directories,
            classify_untracked_bare_repositories: v.classify_untracked_bare_repositories,
        }
    }
}

impl Options {
    /// If `toggle` is `true`, we will stop figuring out if any directory that is a candidate for recursion is also a nested repository,
    /// which saves time but leads to recurse into it. If `false`, nested repositories will not be traversed.
    pub fn recurse_repositories(mut self, toggle: bool) -> Self {
        self.recurse_repositories = toggle;
        self
    }
    /// If `toggle` is `true`, entries that are pruned and whose [Kind](gix_dir::entry::Kind) is known will be emitted.
    pub fn emit_pruned(mut self, toggle: bool) -> Self {
        self.emit_pruned = toggle;
        self
    }
    /// If `value` is `Some(mode)`, entries that are ignored will be emitted according to the given `mode`.
    /// If `None`, ignored entries will not be emitted at all.
    pub fn emit_ignored(mut self, value: Option<EmissionMode>) -> Self {
        self.emit_ignored = value;
        self
    }
    /// When the walk is for deletion, `value` must be `Some(_)` to assure we don't collapse directories that have precious files in
    /// them, and otherwise assure that no entries are observable that shouldn't be deleted.
    /// If `None`, precious files are treated like expendable files, which is usually what you want when displaying them
    /// for addition to the repository, and the collapse of folders can be more generous in relation to ignored files.
    pub fn for_deletion(mut self, value: Option<ForDeletionMode>) -> Self {
        self.for_deletion = value;
        self
    }
    /// If `toggle` is `true`, we will also emit entries for tracked items. Otherwise these will remain 'hidden',
    /// even if a pathspec directly refers to it.
    pub fn emit_tracked(mut self, toggle: bool) -> Self {
        self.emit_tracked = toggle;
        self
    }
    /// Controls the way untracked files are emitted. By default, this is happening immediately and without any simplification.
    pub fn emit_untracked(mut self, toggle: EmissionMode) -> Self {
        self.emit_untracked = toggle;
        self
    }
    /// If `toggle` is `true`, emit empty directories as well. Note that a directory also counts as empty if it has any
    /// amount or depth of nested subdirectories, as long as none of them includes a file.
    /// Thus, this makes leaf-level empty directories visible, as those don't have any content.
    pub fn emit_empty_directories(mut self, toggle: bool) -> Self {
        self.emit_empty_directories = toggle;
        self
    }

    /// If `toggle` is `true`, we will not only find non-bare repositories in untracked directories, but also bare ones.
    ///
    /// Note that this is very costly, but without it, bare repositories will appear like untracked directories when collapsed,
    /// and they will be recursed into.
    pub fn classify_untracked_bare_repositories(mut self, toggle: bool) -> Self {
        self.classify_untracked_bare_repositories = toggle;
        self
    }
}
