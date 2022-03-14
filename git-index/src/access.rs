use bstr::{BStr, ByteSlice};

use crate::{extension, Entry, State, Version};

impl State {
    pub fn version(&self) -> Version {
        self.version
    }

    pub fn entries(&self) -> &[Entry] {
        &self.entries
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
    pub fn take_paths_backing(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.path_backing)
    }
    pub fn set_path_backing(&mut self, paths: Vec<u8>) {
        self.path_backing = paths;
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
