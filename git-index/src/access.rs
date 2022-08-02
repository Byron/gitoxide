use bstr::{BStr, ByteSlice};

use crate::{entry, extension, Entry, PathStorage, State, Version};

/// General information and entries
impl State {
    pub fn version(&self) -> Version {
        self.version
    }

    pub fn entries(&self) -> &[Entry] {
        &self.entries
    }
    pub fn path_backing(&self) -> &PathStorage {
        &self.path_backing
    }
    pub fn take_path_backing(&mut self) -> PathStorage {
        assert_eq!(
            self.entries.is_empty(),
            self.path_backing.is_empty(),
            "BUG: cannot take out backing multiple times"
        );
        std::mem::take(&mut self.path_backing)
    }

    pub fn return_path_backing(&mut self, backing: PathStorage) {
        assert!(
            self.path_backing.is_empty(),
            "BUG: return path backing only after taking it, once"
        );
        self.path_backing = backing;
    }

    pub fn entries_with_paths_by_filter_map<'a, T>(
        &'a self,
        mut filter_map: impl FnMut(&'a BStr, &Entry) -> Option<T> + 'a,
    ) -> impl Iterator<Item = (&'a BStr, T)> + 'a {
        self.entries.iter().filter_map(move |e| {
            let p = e.path(self);
            filter_map(p, e).map(|t| (p, t))
        })
    }
    pub fn entries_mut(&mut self) -> &mut [Entry] {
        &mut self.entries
    }
    pub fn entries_mut_with_paths(&mut self) -> impl Iterator<Item = (&mut Entry, &BStr)> {
        let paths = &self.path_backing;
        self.entries.iter_mut().map(move |e| {
            let path = (&paths[e.path.clone()]).as_bstr();
            (e, path)
        })
    }
    pub fn entries_mut_with_paths_in<'state, 'backing>(
        &'state mut self,
        backing: &'backing PathStorage,
    ) -> impl Iterator<Item = (&'state mut Entry, &'backing BStr)> {
        self.entries.iter_mut().map(move |e| {
            let path = (&backing[e.path.clone()]).as_bstr();
            (e, path)
        })
    }

    /// Find the entry index in [`entries()`][State::entries()] matching the given repository-relative
    /// `path` and `stage`, or `None`.
    ///
    /// Use the index for accessing multiple stages if they exists, but at least the single matching entry.
    pub fn entry_by_path_and_stage(&self, path: &BStr, stage: entry::Stage) -> Option<usize> {
        self.entries
            .binary_search_by(|e| e.path(self).cmp(path).then_with(|| e.stage().cmp(&stage)))
            .ok()
    }
}

/// Extensions
impl State {
    pub fn tree(&self) -> Option<&extension::Tree> {
        self.tree.as_ref()
    }
    pub fn link(&self) -> Option<&extension::Link> {
        self.link.as_ref()
    }
    pub fn resolve_undo(&self) -> Option<&extension::resolve_undo::Paths> {
        self.resolve_undo.as_ref()
    }
    pub fn untracked(&self) -> Option<&extension::UntrackedCache> {
        self.untracked.as_ref()
    }
    pub fn fs_monitor(&self) -> Option<&extension::FsMonitor> {
        self.fs_monitor.as_ref()
    }
}
