use crate::{extension, Entry, State, Version};

impl State {
    pub fn version(&self) -> Version {
        self.version
    }

    pub fn entries(&self) -> &[Entry] {
        &self.entries
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
