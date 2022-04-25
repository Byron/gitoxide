use bstr::{BStr, ByteSlice};

use crate::{extension, Entry, State, Version};

impl State {
    pub fn version(&self) -> Version {
        self.version
    }

    pub fn entries(&self) -> &[Entry] {
        &self.entries
    }
    pub fn entries_with_paths_by_filter_map<'a, T>(
        &'a self,
        mut filter_map: impl FnMut(&BStr, &Entry) -> Option<T> + 'a,
    ) -> impl Iterator<Item = T> + 'a {
        self.entries.iter().filter_map(move |e| filter_map(e.path(self), e))
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
