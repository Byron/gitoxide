/// Configuration related to sparse indexes.
#[derive(Debug, Default, Clone, Copy)]
pub struct Options {
    /// If true, certain entries in the index will be excluded / skipped for certain operations,
    /// based on the ignore patterns in the `.git/info/sparse-checkout` file. These entries will
    /// carry the [`SKIP_WORKTREE`][crate::entry::Flags::SKIP_WORKTREE] flag.
    ///
    /// This typically is the value of `core.sparseCheckout` in the git configuration.
    pub sparse_checkout: bool,

    /// Interpret the `.git/info/sparse-checkout` file using _cone mode_.
    ///
    /// If true, _cone mode_ is active and entire directories will be included in the checkout, as well as files in the root
    /// of the repository.
    /// If false, non-cone mode is active and entries to _include_ will be matched with patterns like those found in `.gitignore` files.
    ///
    /// This typically is the value of `core.sparseCheckoutCone` in the git configuration.
    pub directory_patterns_only: bool,

    /// If true, will attempt to write a sparse index file which only works in cone mode.
    ///
    /// A sparse index has [`DIR` entries][crate::entry::Mode::DIR] that represent entire directories to be skipped
    /// during checkout and other operations due to the added presence of
    /// the [`SKIP_WORKTREE`][crate::entry::Flags::SKIP_WORKTREE] flag.
    ///
    /// This is typically the value of `index.sparse` in the git configuration.
    pub write_sparse_index: bool,
}

impl Options {
    /// Derive a valid mode from all parameters that affect the 'sparseness' of the index.
    ///
    /// Some combinations of them degenerate to one particular mode.
    pub fn sparse_mode(&self) -> Mode {
        match (
            self.sparse_checkout,
            self.directory_patterns_only,
            self.write_sparse_index,
        ) {
            (true, true, true) => Mode::IncludeDirectoriesStoreIncludedEntriesAndExcludedDirs,
            (true, true, false) => Mode::IncludeDirectoriesStoreAllEntriesSkipUnmatched,
            (true, false, _) => Mode::IncludeByIgnorePatternStoreAllEntriesSkipUnmatched,
            (false, _, _) => Mode::Disabled,
        }
    }
}

/// Describes the configuration how a sparse index should be written, or if one should be written at all.
#[derive(Debug)]
pub enum Mode {
    /// index with DIR entries for exclusion and included entries, directory-only include patterns in `.git/info/sparse-checkout` file.
    IncludeDirectoriesStoreIncludedEntriesAndExcludedDirs,
    /// index with all file entries and skip worktree flags for exclusion, directory-only include patterns in `.git/info/sparse-checkout` file.
    IncludeDirectoriesStoreAllEntriesSkipUnmatched,
    /// index with all file entries and skip-worktree flags for exclusion, `ignore` patterns to include entries in `.git/info/sparse-checkout` file.
    IncludeByIgnorePatternStoreAllEntriesSkipUnmatched,
    /// index with all entries, non is excluded, `.git/info/sparse-checkout` file is not considered, a regular index.
    Disabled,
}
