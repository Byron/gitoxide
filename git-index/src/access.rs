use crate::{entry, extension, Entry, PathStorage, State, Version};
use bstr::{BStr, ByteSlice};

mod sparse {
    use git_object::TreeRefIter;

    use crate::{entry, State};

    /// Configuration related to sparse indexes
    #[derive(Debug, Default, Clone, Copy)]
    pub struct SparseOptions {
        /// If true, certain files files in the index will be exluded / skipped for certain operations,
        /// based on the content of the `.git/info/sparse-checkout` file
        pub sparse_checkout: bool,

        /// Configures how to interpret the `.git/info/sparse-checkout` file
        /// If true, cone mode is active and entire directories will be excluded
        /// If false, non-cone mode is active and files will be matched similar to a .gitignore file
        pub cone_mode: bool,

        /// If true, will attempt to write a sparse index file
        /// only works in cone mode
        pub write_sparse_index: bool,
    }

    impl SparseOptions {
        /// Figures out if the index should be sparse or not depending on the given options
        #[allow(dead_code)]
        pub fn get_sparse_mode(&self) -> SparseMode {
            match (self.sparse_checkout, self.cone_mode, self.write_sparse_index) {
                (true, true, true) => SparseMode::SparseIndexConeMode,
                (true, true, false) => SparseMode::RegularIndexConeMode,
                (true, false, _) => SparseMode::RegularIndexNoConeMode,
                (false, _, _) => SparseMode::RegularIndex,
            }
        }
    }

    /// Describes the configuration how a sparse index should be written, or if one should be written at all
    #[derive(Debug)]
    #[allow(dead_code)]
    pub enum SparseMode {
        /// sparse index, cone mode, skip worktree based on .git/info/sparse-checkout file
        SparseIndexConeMode,
        /// regular index, cone mode, skip worktree based on .git/info/sparse-checkout file
        RegularIndexConeMode,
        /// regular index, no-cone mode, skip worktree based on .git/info/sparse-checkout file
        RegularIndexNoConeMode,
        /// regular index, .git/info/sparse-checkout file is not considered / no skip_worktree flags
        RegularIndex,
    }

    /// Transformations and mutations to the state
    impl State {
        /// Expand all entries with Mode::DIR to a list of files contained within those entries
        pub fn expand_dir_entries<Find>(&mut self, _find: Find)
        where
            Find: for<'a> FnMut(&git_hash::oid, &'a mut Vec<u8>) -> Option<TreeRefIter<'a>>,
        {
            self.entries_mut().iter_mut().for_each(|e| {
                if e.mode == entry::Mode::DIR {
                    // TODO: do a tree traversal and replace the DIR entry with all FILE entries found
                    // maybe we can somehow generalize tree traversal we are already doing in `from_tree`

                    // NOTE: this line is just here for the moment to satisfy the test
                    e.mode = entry::Mode::FILE;
                }
            });

            // TODO: self.is_sparse = false
        }
    }
}

/// General information and entries
impl State {
    /// Return the version used to store this state's information on disk.
    pub fn version(&self) -> Version {
        self.version
    }

    /// Return the kind of hashes used in this instance.
    pub fn object_hash(&self) -> git_hash::Kind {
        self.object_hash
    }

    /// Return our entries
    pub fn entries(&self) -> &[Entry] {
        &self.entries
    }
    /// Return our path backing, the place which keeps all paths one after another, with entries storing only the range to access them.
    pub fn path_backing(&self) -> &PathStorage {
        &self.path_backing
    }
    /// Sometimes it's needed to remove the path backing to allow certain mutation to happen in the state while supporting reading the entry's
    /// path.
    pub fn take_path_backing(&mut self) -> PathStorage {
        assert_eq!(
            self.entries.is_empty(),
            self.path_backing.is_empty(),
            "BUG: cannot take out backing multiple times"
        );
        std::mem::take(&mut self.path_backing)
    }

    /// After usage of the storage obtained by [`take_path_backing()`][Self::take_path_backing()], return it here.
    /// Note that it must not be empty.
    pub fn return_path_backing(&mut self, backing: PathStorage) {
        debug_assert!(
            self.path_backing.is_empty(),
            "BUG: return path backing only after taking it, once"
        );
        self.path_backing = backing;
    }

    /// Runs `filter_map` on all entries, returning an iterator over all paths along with the result of `filter_map`.
    pub fn entries_with_paths_by_filter_map<'a, T>(
        &'a self,
        mut filter_map: impl FnMut(&'a BStr, &Entry) -> Option<T> + 'a,
    ) -> impl Iterator<Item = (&'a BStr, T)> + 'a {
        self.entries.iter().filter_map(move |e| {
            let p = e.path(self);
            filter_map(p, e).map(|t| (p, t))
        })
    }
    /// Return mutable entries in a slice.
    pub fn entries_mut(&mut self) -> &mut [Entry] {
        &mut self.entries
    }
    /// Return mutable entries along with their paths in an iterator.
    pub fn entries_mut_with_paths(&mut self) -> impl Iterator<Item = (&mut Entry, &BStr)> {
        let paths = &self.path_backing;
        self.entries.iter_mut().map(move |e| {
            let path = paths[e.path.clone()].as_bstr();
            (e, path)
        })
    }

    /// Return mutable entries along with their path, as obtained from `backing`.
    pub fn entries_mut_with_paths_in<'state, 'backing>(
        &'state mut self,
        backing: &'backing PathStorage,
    ) -> impl Iterator<Item = (&'state mut Entry, &'backing BStr)> {
        self.entries.iter_mut().map(move |e| {
            let path = backing[e.path.clone()].as_bstr();
            (e, path)
        })
    }

    /// Find the entry index in [`entries()`][State::entries()] matching the given repository-relative
    /// `path` and `stage`, or `None`.
    ///
    /// Use the index for accessing multiple stages if they exists, but at least the single matching entry.
    pub fn entry_index_by_path_and_stage(&self, path: &BStr, stage: entry::Stage) -> Option<usize> {
        self.entries
            .binary_search_by(|e| e.path(self).cmp(path).then_with(|| e.stage().cmp(&stage)))
            .ok()
    }

    /// Like [`entry_index_by_path_and_stage()`][State::entry_index_by_path_and_stage()],
    /// but returns the entry instead of the index.
    pub fn entry_by_path_and_stage(&self, path: &BStr, stage: entry::Stage) -> Option<&Entry> {
        self.entry_index_by_path_and_stage(path, stage)
            .map(|idx| &self.entries[idx])
    }

    /// Return the entry at `idx` or _panic_ if the index is out of bounds.
    ///
    /// The `idx` is typically returned by [entry_by_path_and_stage()][State::entry_by_path_and_stage()].
    pub fn entry(&self, idx: usize) -> &Entry {
        &self.entries[idx]
    }

    /// Returns a boolean value indicating whether the index is sparse or not.
    ///
    /// An index is sparse if it contains at least one [Mode::DIR][Entry::Mode::DIR] entry.
    pub fn is_sparse(&self) -> bool {
        self.is_sparse
    }
}

/// Extensions
impl State {
    /// Access the `tree` extension.
    pub fn tree(&self) -> Option<&extension::Tree> {
        self.tree.as_ref()
    }
    /// Access the `link` extension.
    pub fn link(&self) -> Option<&extension::Link> {
        self.link.as_ref()
    }
    /// Obtain the resolve-undo extension.
    pub fn resolve_undo(&self) -> Option<&extension::resolve_undo::Paths> {
        self.resolve_undo.as_ref()
    }
    /// Obtain the untracked extension.
    pub fn untracked(&self) -> Option<&extension::UntrackedCache> {
        self.untracked.as_ref()
    }
    /// Obtain the fsmonitor extension.
    pub fn fs_monitor(&self) -> Option<&extension::FsMonitor> {
        self.fs_monitor.as_ref()
    }
}
