use crate::{
    entry::{self, Flags, Mode, Stat, Time},
    extension, Entry, PathStorage, State, Version,
};
use bstr::{BStr, ByteSlice};
use git_object::{tree::EntryMode, TreeRefIter};
use git_traverse::tree::breadthfirst;

/// initialization
impl State {
    /// Takes in an oid of a tree object and creates and returns a [`State`][git_index::State] from its children.
    pub fn from_tree<Find>(tree: &git_hash::oid, mut find: Find) -> Result<Self, breadthfirst::Error>
    where
        Find: for<'a> FnMut(&git_hash::oid, &'a mut Vec<u8>) -> Option<TreeRefIter<'a>>,
    {
        let mut buf = Vec::new();
        let root = find(tree, &mut buf).expect("couldn't find a tree for given oid");
        let state = breadthfirst::State::default();
        let mut delegate = git_traverse::tree::Recorder::default();
        breadthfirst(root, state, &mut find, &mut delegate)?;

        // TODO: estimate size
        let mut path_backing: PathStorage = Vec::new();

        // TODO: insert entries directly into the correct position
        // TODO: double check this way of sorting is actually the right one
        delegate.records.sort_by(|a, b| a.filepath.cmp(&b.filepath));

        let entries = delegate
            .records
            .into_iter()
            .filter_map(|file| {
                let mode = match file.mode {
                    EntryMode::Tree => Mode::DIR,
                    EntryMode::Blob => Mode::FILE,
                    EntryMode::BlobExecutable => Mode::FILE_EXECUTABLE,
                    EntryMode::Link => Mode::SYMLINK,
                    EntryMode::Commit => Mode::COMMIT,
                };

                match mode {
                    Mode::FILE => {
                        let path_start = path_backing.len();
                        path_backing.extend_from_slice(&file.filepath);
                        Some(Entry {
                            // TODO: represent uninitialized state
                            stat: Stat {
                                mtime: Time { secs: 0, nsecs: 0 },
                                ctime: Time { secs: 0, nsecs: 0 },
                                dev: 0,
                                ino: 0,
                                uid: 0,
                                gid: 0,
                                size: 0,
                            },
                            id: file.oid,
                            flags: Flags::empty(),
                            mode,
                            path: path_start..path_backing.len(),
                        })
                    }
                    _ => None,
                }
            })
            .collect::<Vec<Entry>>();

        Ok(State {
            timestamp: filetime::FileTime::now(),
            version: Version::V2,
            entries,
            path_backing,
            is_sparse: false,
            tree: None,
            link: None,
            resolve_undo: None,
            untracked: None,
            fs_monitor: None,
        })
    }
}

/// General information and entries
impl State {
    /// Return the version used to store this state's information on disk.
    pub fn version(&self) -> Version {
        self.version
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
            let path = (&paths[e.path.clone()]).as_bstr();
            (e, path)
        })
    }

    /// Return mutable entries along with their path, as obtained from `backing`.
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
