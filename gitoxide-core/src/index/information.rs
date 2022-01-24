use git_repository as git;

pub struct Options {
    pub index: super::Options,
    /// If true, show exstension in detail.
    pub extension_details: bool,
}

mod ext {
    #[cfg_attr(feature = "serde1", derive(serde::Serialize))]
    pub(crate) struct Tree {
        name: String,
        /// Only set if there are any entries in the index we are associated with.
        id: Option<tree::NodeId>,
        children: Vec<Tree>,
    }

    mod tree {
        use git_repository as git;
        use git_repository::bstr::ByteSlice;

        impl<'a> From<&'a git::index::extension::Tree> for super::Tree {
            fn from(t: &'a git_repository::index::extension::Tree) -> Self {
                super::Tree {
                    name: t.name.as_bstr().to_string(),
                    id: t.id.as_ref().map(|id| NodeId {
                        entry_count: id.entry_count,
                        id: id.id.to_hex().to_string(),
                    }),
                    children: t.children.iter().map(|t| t.into()).collect(),
                }
            }
        }

        #[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
        pub struct NodeId {
            /// The id of the directory tree of the associated tree object.
            id: String,
            /// The amount of non-tree entries contained within, and definitely not zero.
            entry_count: u32,
        }
    }
}

#[cfg_attr(feature = "serde1", derive(serde::Serialize))]
pub(crate) struct EntryKind {
    dir: usize,
    file: usize,
    executable: usize,
    symlink: usize,
    submodule: usize,
    other: usize,
}

#[cfg_attr(feature = "serde1", derive(serde::Serialize))]
pub(crate) struct EntryFlag {
    intent_to_add: usize,
    skip_worktree: usize,
}

#[cfg_attr(feature = "serde1", derive(serde::Serialize))]
pub(crate) struct Entries {
    stage_0: usize,
    stage_1: usize,
    stage_2: usize,
    kind: EntryKind,
    flags: EntryFlag,
}

#[cfg_attr(feature = "serde1", derive(serde::Serialize))]
pub(crate) struct Extensions {
    names: Vec<&'static str>,
    tree: Option<ext::Tree>,
}

#[cfg_attr(feature = "serde1", derive(serde::Serialize))]
pub(crate) struct Collection {
    version: u8,
    checksum: String,
    entries: Entries,
    extensions: Extensions,
}

impl Collection {
    pub fn try_from_file(f: git::index::File, extension_details: bool) -> anyhow::Result<Self> {
        Ok(Collection {
            version: f.version() as u8,
            checksum: f.checksum.to_hex().to_string(),
            extensions: {
                let mut names = Vec::new();
                let tree = f.tree().and_then(|tree| {
                    names.push("tree (TREE)");
                    extension_details.then(|| tree.into())
                });
                if f.link().is_some() {
                    names.push("link");
                };
                if f.resolve_undo().is_some() {
                    names.push("resolve-undo (REUC)");
                };
                if f.untracked().is_some() {
                    names.push("untracked (UNTR)");
                };
                if f.fs_monitor().is_some() {
                    names.push("fs-monitor (FSMN)");
                };
                Extensions { names, tree }
            },
            entries: {
                let (mut stage_0, mut stage_1, mut stage_2) = (0, 0, 0);
                let (mut dir, mut file, mut executable, mut symlink, mut submodule, mut other) = (0, 0, 0, 0, 0, 0);
                let (mut intent_to_add, mut skip_worktree) = (0, 0);
                for entry in f.entries() {
                    match entry.flags.stage() {
                        0 => stage_0 += 1,
                        1 => stage_1 += 1,
                        2 => stage_2 += 1,
                        invalid => anyhow::bail!("Invalid stage {} encountered", invalid),
                    }
                    match entry.mode {
                        git::index::entry::Mode::DIR => dir += 1,
                        git::index::entry::Mode::FILE => file += 1,
                        git::index::entry::Mode::FILE_EXECUTABLE => executable += 1,
                        git::index::entry::Mode::SYMLINK => symlink += 1,
                        git::index::entry::Mode::COMMIT => submodule += 1,
                        _ => other += 1,
                    }
                    if entry.flags.contains(git::index::entry::Flags::INTENT_TO_ADD) {
                        intent_to_add += 1;
                    }
                    if entry.flags.contains(git::index::entry::Flags::SKIP_WORKTREE) {
                        skip_worktree += 1;
                    }
                }
                Entries {
                    stage_0,
                    stage_1,
                    stage_2,
                    kind: EntryKind {
                        dir,
                        file,
                        executable,
                        symlink,
                        submodule,
                        other,
                    },
                    flags: EntryFlag {
                        intent_to_add,
                        skip_worktree,
                    },
                }
            },
        })
    }
}
