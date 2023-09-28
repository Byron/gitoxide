use std::cmp::Ordering;
use std::ops::Range;

use bstr::{BStr, ByteSlice, ByteVec};
use filetime::FileTime;

use crate::{entry, extension, Entry, PathStorage, PathStorageRef, State, Version};

// TODO: integrate this somehow, somewhere, depending on later usage.
#[allow(dead_code)]
mod sparse;

/// General information and entries
impl State {
    /// Return the version used to store this state's information on disk.
    pub fn version(&self) -> Version {
        self.version
    }

    /// Returns time at which the state was created, indicating its freshness compared to other files on disk.
    pub fn timestamp(&self) -> FileTime {
        self.timestamp
    }

    /// Updates the timestamp of this state, indicating its freshness compared to other files on disk.
    ///
    /// Be careful about using this as setting a timestamp without correctly updating the index
    /// **will cause (file system) race conditions** see racy-git.txt in the git documentation
    /// for more details.
    pub fn set_timestamp(&mut self, timestamp: FileTime) {
        self.timestamp = timestamp
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
    pub fn path_backing(&self) -> &PathStorageRef {
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
        backing: &'backing PathStorageRef,
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
        let mut stage_cmp = Ordering::Equal;
        let idx = self
            .entries
            .binary_search_by(|e| {
                let res = e.path(self).cmp(path);
                if res.is_eq() {
                    stage_cmp = e.stage().cmp(&stage);
                }
                res
            })
            .ok()?;
        self.entry_index_by_idx_and_stage(path, idx, stage, stage_cmp)
    }

    /// Walk as far in `direction` as possible, with [`Ordering::Greater`] towards higher stages, and [`Ordering::Less`]
    /// towards lower stages, and return the lowest or highest seen stage.
    /// Return `None` if there is no greater or smaller stage.
    fn walk_entry_stages(&self, path: &BStr, base: usize, direction: Ordering) -> Option<usize> {
        match direction {
            Ordering::Greater => self
                .entries
                .get(base + 1..)?
                .iter()
                .enumerate()
                .take_while(|(_, e)| e.path(self) == path)
                .last()
                .map(|(idx, _)| base + 1 + idx),
            Ordering::Equal => Some(base),
            Ordering::Less => self.entries[..base]
                .iter()
                .enumerate()
                .rev()
                .take_while(|(_, e)| e.path(self) == path)
                .last()
                .map(|(idx, _)| idx),
        }
    }

    fn entry_index_by_idx_and_stage(
        &self,
        path: &BStr,
        idx: usize,
        wanted_stage: entry::Stage,
        stage_cmp: Ordering,
    ) -> Option<usize> {
        match stage_cmp {
            Ordering::Greater => self.entries[..idx]
                .iter()
                .enumerate()
                .rev()
                .take_while(|(_, e)| e.path(self) == path)
                .find_map(|(idx, e)| (e.stage() == wanted_stage).then_some(idx)),
            Ordering::Equal => Some(idx),
            Ordering::Less => self
                .entries
                .get(idx + 1..)?
                .iter()
                .enumerate()
                .take_while(|(_, e)| e.path(self) == path)
                .find_map(|(ofs, e)| (e.stage() == wanted_stage).then_some(idx + ofs + 1)),
        }
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

    /// Return the entry at `path` that is either at stage 0, or at stage 2 (ours) in case of a merge conflict.
    ///
    /// Using this method is more efficient in comparison to doing two searches, one for stage 0 and one for stage 2.
    pub fn entry_by_path(&self, path: &BStr) -> Option<&Entry> {
        let mut stage_at_index = 0;
        let idx = self
            .entries
            .binary_search_by(|e| {
                let res = e.path(self).cmp(path);
                if res.is_eq() {
                    stage_at_index = e.stage();
                }
                res
            })
            .ok()?;
        let idx = if stage_at_index == 0 || stage_at_index == 2 {
            idx
        } else {
            self.entry_index_by_idx_and_stage(path, idx, 2, stage_at_index.cmp(&2))?
        };
        Some(&self.entries[idx])
    }

    /// Return the slice of entries which all share the same `prefix`, or `None` if there isn't a single such entry.
    ///
    /// If `prefix` is empty, all entries are returned.
    pub fn prefixed_entries(&self, prefix: &BStr) -> Option<&[Entry]> {
        self.prefixed_entries_range(prefix).map(|range| &self.entries[range])
    }

    /// Return the range of entries which all share the same `prefix`, or `None` if there isn't a single such entry.
    ///
    /// If `prefix` is empty, the range will include all entries.
    pub fn prefixed_entries_range(&self, prefix: &BStr) -> Option<Range<usize>> {
        if prefix.is_empty() {
            return Some(0..self.entries.len());
        }
        let prefix_len = prefix.len();
        let mut low = self.entries.partition_point(|e| {
            e.path(self)
                .get(..prefix_len)
                .map_or_else(|| e.path(self) <= &prefix[..e.path.len()], |p| p < prefix)
        });
        let mut high = low
            + self.entries[low..].partition_point(|e| e.path(self).get(..prefix_len).map_or(false, |p| p <= prefix));

        let low_entry = &self.entries.get(low)?;
        if low_entry.stage() != 0 {
            low = self
                .walk_entry_stages(low_entry.path(self), low, Ordering::Less)
                .unwrap_or(low);
        }
        if let Some(high_entry) = self.entries.get(high) {
            if high_entry.stage() != 0 {
                high = self
                    .walk_entry_stages(high_entry.path(self), high, Ordering::Less)
                    .unwrap_or(high);
            }
        }
        (low != high).then_some(low..high)
    }

    /// Return the entry at `idx` or _panic_ if the index is out of bounds.
    ///
    /// The `idx` is typically returned by [`entry_by_path_and_stage()`][State::entry_by_path_and_stage()].
    pub fn entry(&self, idx: usize) -> &Entry {
        &self.entries[idx]
    }

    /// Returns a boolean value indicating whether the index is sparse or not.
    ///
    /// An index is sparse if it contains at least one [`Mode::DIR`][entry::Mode::DIR] entry.
    pub fn is_sparse(&self) -> bool {
        self.is_sparse
    }

    /// Return the range of entries that exactly match the given `path`, in all available stages, or `None` if no entry with such
    /// path exists.
    ///
    /// The range can be used to access the respective entries via [`entries()`](Self::entries()) or [`entries_mut()](Self::entries_mut()).
    pub fn entry_range(&self, path: &BStr) -> Option<Range<usize>> {
        let mut stage_at_index = 0;
        let idx = self
            .entries
            .binary_search_by(|e| {
                let res = e.path(self).cmp(path);
                if res.is_eq() {
                    stage_at_index = e.stage();
                }
                res
            })
            .ok()?;

        let (start, end) = (
            self.walk_entry_stages(path, idx, Ordering::Less).unwrap_or(idx),
            self.walk_entry_stages(path, idx, Ordering::Greater).unwrap_or(idx) + 1,
        );
        Some(start..end)
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

    /// Return a writable slice to entries and read-access to their path storage at the same time.
    pub fn entries_mut_and_pathbacking(&mut self) -> (&mut [Entry], &PathStorageRef) {
        (&mut self.entries, &self.path_backing)
    }

    /// Return mutable entries along with their paths in an iterator.
    pub fn entries_mut_with_paths(&mut self) -> impl Iterator<Item = (&mut Entry, &BStr)> {
        let paths = &self.path_backing;
        self.entries.iter_mut().map(move |e| {
            let path = paths[e.path.clone()].as_bstr();
            (e, path)
        })
    }

    /// Return all parts that relate to entries, which includes path storage.
    ///
    /// This can be useful for obtaining a standalone, boxable iterator
    pub fn into_entries(self) -> (Vec<Entry>, PathStorage) {
        (self.entries, self.path_backing)
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
    /// Alternatively, make sure to call [`sort_entries()`][State::sort_entries()] before entry lookup by path to restore
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

    /// Physically remove all entries for which `should_remove(idx, path, entry)` returns `true`, traversing them from first to last.
    ///
    /// Note that the memory used for the removed entries paths is not freed, as it's append-only.
    ///
    /// ### Performance
    ///
    /// To implement this operation typically, one would rather add [entry::Flags::REMOVE] to each entry to remove
    /// them when [writing the index](Self::write_to()).
    pub fn remove_entries(&mut self, mut should_remove: impl FnMut(usize, &BStr, &mut Entry) -> bool) {
        let mut index = 0;
        let paths = &self.path_backing;
        self.entries.retain_mut(|e| {
            let path = e.path_in(paths);
            let res = !should_remove(index, path, e);
            index += 1;
            res
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

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    #[test]
    fn entry_by_path_with_conflicting_file() {
        let file = PathBuf::from("tests")
            .join("fixtures")
            .join(Path::new("loose_index").join("conflicting-file.git-index"));
        let file = crate::File::at(file, gix_hash::Kind::Sha1, false, Default::default()).expect("valid file");
        assert_eq!(
            file.entries().len(),
            3,
            "we have a set of conflict entries for a single file"
        );
        for idx in 0..3 {
            for wanted_stage in 1..=3 {
                let actual_idx = file
                    .entry_index_by_idx_and_stage(
                        "file".into(),
                        idx,
                        wanted_stage,
                        (idx + 1).cmp(&(wanted_stage as usize)),
                    )
                    .expect("found");
                assert_eq!(
                    actual_idx + 1,
                    wanted_stage as usize,
                    "the index and stage have a relation, and that is upheld if we search correctly"
                );
            }
        }
    }
}
