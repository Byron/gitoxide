use crate::{
    entry::{Flags, Mode, Stat, Time},
    Entry, PathStorage, State, Version,
};
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
