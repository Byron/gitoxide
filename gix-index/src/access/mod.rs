use std::cmp::Ordering;

use bstr::{BStr, ByteSlice, ByteVec};

use crate::{entry, extension, Entry, PathStorage, State, Version};

// TODO: integrate this somehow, somewhere, depending on later usage.
#[allow(dead_code)]
mod sparse;

/// General information and entries
impl State {
    /// Return the version used to store this state's information on disk.
    pub fn version(&self) -> Version {
        self.version
    }

    /// Return the kind of hashes used in this instance.
    pub fn object_hash(&self) -> gix_hash::Kind {
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

    /// Find the entry index in [`entries()[..upper_bound]`][State::entries()] matching the given repository-relative
    /// `path` and `stage`, or `None`.
    ///
    /// Use the index for accessing multiple stages if they exists, but at least the single matching entry.
    ///
    /// # Panics
    ///
    /// If `upper_bound` is out of bounds of our entries array.
    pub fn entry_index_by_path_and_stage_bounded(
        &self,
        path: &BStr,
        stage: entry::Stage,
        upper_bound: usize,
    ) -> Option<usize> {
        self.entries[..upper_bound]
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
    /// An index is sparse if it contains at least one [Mode::DIR][entry::Mode::DIR] entry.
    pub fn is_sparse(&self) -> bool {
        self.is_sparse
    }
}

/// Mutation
impl State {
    /// After usage of the storage obtained by [`take_path_backing()`][Self::take_path_backing()], return it here.
    /// Note that it must not be empty.
    pub fn return_path_backing(&mut self, backing: PathStorage) {
        debug_assert!(
            self.path_backing.is_empty(),
            "BUG: return path backing only after taking it, once"
        );
        self.path_backing = backing;
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

    /// Like [`entry_index_by_path_and_stage()`][State::entry_index_by_path_and_stage()],
    /// but returns the mutable entry instead of the index.
    pub fn entry_mut_by_path_and_stage(&mut self, path: &BStr, stage: entry::Stage) -> Option<&mut Entry> {
        self.entry_index_by_path_and_stage(path, stage)
            .map(|idx| &mut self.entries[idx])
    }

    /// Push a new entry containing `stat`, `id`, `flags` and `mode` and `path` to the end of our storage, without performing
    /// any sanity checks. This means it's possible to push a new entry to the same path on the same stage and even after sorting
    /// the entries lookups may still return the wrong one of them unless the correct binary search criteria is chosen.
    ///
    /// Note that this *is likely* to break invariants that will prevent further lookups by path unless
    /// [`entry_index_by_path_and_stage_bounded()`][State::entry_index_by_path_and_stage_bounded()] is used with
    /// the `upper_bound` being the amount of entries before the first call to this method.
    ///
    /// Alternatively, make sure to call [sort_entries()][State::sort_entries()] before entry lookup by path to restore
    /// the invariant.
    pub fn dangerously_push_entry(
        &mut self,
        stat: entry::Stat,
        id: gix_hash::ObjectId,
        flags: entry::Flags,
        mode: entry::Mode,
        path: &BStr,
    ) {
        let path = {
            let path_start = self.path_backing.len();
            self.path_backing.push_str(path);
            path_start..self.path_backing.len()
        };

        self.entries.push(Entry {
            stat,
            id,
            flags,
            mode,
            path,
        });
    }

    /// Unconditionally sort entries as needed to perform lookups quickly.
    pub fn sort_entries(&mut self) {
        let path_backing = &self.path_backing;
        self.entries.sort_by(|a, b| {
            Entry::cmp_filepaths(a.path_in(path_backing), b.path_in(path_backing))
                .then_with(|| a.stage().cmp(&b.stage()))
        });
    }

    /// Similar to [`sort_entries()`][State::sort_entries()], but applies `compare` after comparing
    /// by path and stage as a third criteria.
    pub fn sort_entries_by(&mut self, mut compare: impl FnMut(&Entry, &Entry) -> Ordering) {
        let path_backing = &self.path_backing;
        self.entries.sort_by(|a, b| {
            Entry::cmp_filepaths(a.path_in(path_backing), b.path_in(path_backing))
                .then_with(|| a.stage().cmp(&b.stage()))
                .then_with(|| compare(a, b))
        });
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
