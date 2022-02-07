pub struct Options {
    pub index: super::Options,
    /// If true, show exstension in detail.
    pub extension_details: bool,
}

#[cfg(feature = "serde1")]
mod serde_only {
    use git_repository as git;
    mod ext {
        #[derive(serde::Serialize)]
        pub(crate) struct Tree {
            name: String,
            /// The id of the directory tree of the associated tree object.
            id: String,
            /// The amount of non-tree entries contained within, and definitely not zero.
            num_entries: u32,
            children: Vec<Tree>,
        }

        mod tree {
            use git_repository as git;
            use git_repository::bstr::ByteSlice;

            impl<'a> From<&'a git::index::extension::Tree> for super::Tree {
                fn from(t: &'a git_repository::index::extension::Tree) -> Self {
                    super::Tree {
                        name: t.name.as_bstr().to_string(),
                        id: t.id.to_hex().to_string(),
                        num_entries: t.num_entries,
                        children: t.children.iter().map(|t| t.into()).collect(),
                    }
                }
            }

            #[derive(serde::Serialize, serde::Deserialize)]
            pub struct NodeId {}
        }
    }

    #[derive(serde::Serialize)]
    pub(crate) struct EntryKind {
        dir: usize,
        file: usize,
        executable: usize,
        symlink: usize,
        submodule: usize,
        other: usize,
    }

    #[derive(serde::Serialize)]
    pub(crate) struct EntryFlag {
        intent_to_add: usize,
        skip_worktree: usize,
    }

    #[derive(serde::Serialize)]
    pub struct Entries {
        stage_0_merged: usize,
        stage_1_base: usize,
        stage_2_ours: usize,
        stage_3_theirs: usize,
        kind: EntryKind,
        flags: EntryFlag,
    }

    #[derive(serde::Serialize)]
    pub struct Extensions {
        names: Vec<&'static str>,
        tree: Option<ext::Tree>,
    }

    #[derive(serde::Serialize)]
    pub struct Collection {
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
                    let (mut stage_0_merged, mut stage_1_base, mut stage_2_ours, mut stage_3_theirs) = (0, 0, 0, 0);
                    let (mut dir, mut file, mut executable, mut symlink, mut submodule, mut other) = (0, 0, 0, 0, 0, 0);
                    let (mut intent_to_add, mut skip_worktree) = (0, 0);
                    for entry in f.entries() {
                        match entry.flags.stage() {
                            0 => stage_0_merged += 1,
                            1 => stage_1_base += 1,
                            2 => stage_2_ours += 1,
                            3 => stage_3_theirs += 1,
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
                        stage_0_merged,
                        stage_1_base,
                        stage_2_ours,
                        stage_3_theirs,
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
}
#[cfg(feature = "serde1")]
pub(crate) use serde_only::Collection;
