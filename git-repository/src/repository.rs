mod access {
    use crate::{easy, Kind, Repository};

    impl Repository {
        /// Return the kind of repository, either bare or one with a work tree.
        pub fn kind(&self) -> Kind {
            match self.work_tree {
                Some(_) => Kind::WorkTree,
                None => Kind::Bare,
            }
        }

        /// Add thread-local state to an easy-to-use handle for the most convenient API.
        pub fn to_easy(&self) -> easy::Handle {
            self.into()
        }

        // TODO: tests
        /// Load the index file of this repository's workspace, if present.
        ///
        /// Note that it is loaded into memory each time this method is called, but also is independent of the workspace.
        #[cfg(feature = "git-index")]
        pub fn load_index(&self) -> Option<Result<git_index::File, git_index::file::init::Error>> {
            // TODO: choose better/correct options
            let opts = git_index::decode::Options {
                object_hash: self.object_hash,
                thread_limit: None,
                min_extension_block_in_bytes_for_threading: 1024 * 256,
            };
            match git_index::File::at(self.git_dir().join("index"), opts) {
                Ok(index) => Some(Ok(index)),
                Err(git_index::file::init::Error::Io(err)) if err.kind() == std::io::ErrorKind::NotFound => None,
                Err(err) => Some(Err(err)),
            }
        }
    }
}

mod from_path {
    use std::convert::TryFrom;

    use crate::Path;

    impl TryFrom<crate::Path> for crate::Repository {
        type Error = crate::open::Error;

        fn try_from(value: Path) -> Result<Self, Self::Error> {
            let (git_dir, worktree_dir) = value.into_repository_and_work_tree_directories();
            crate::Repository::open_from_paths(git_dir, worktree_dir, Default::default())
        }
    }
}

///
pub mod open {
    use std::{borrow::Cow, path::PathBuf};

    use git_config::{
        file::GitConfig,
        values::{Boolean, Integer},
    };
    use git_features::threading::OwnShared;

    use crate::Repository;

    /// The options used in [`Repository::open_opts
    #[derive(Default)]
    pub struct Options {
        object_store_slots: git_odb::store::init::Slots,
    }

    impl Options {
        /// Set the amount of slots to use for the object database. It's a value that doesn't need changes on the client, typically,
        /// but should be controlled on the server.
        pub fn object_store_slots(mut self, slots: git_odb::store::init::Slots) -> Self {
            self.object_store_slots = slots;
            self
        }

        /// Open a repository at `path` with the options set so far.
        pub fn open(self, path: impl Into<std::path::PathBuf>) -> Result<Repository, Error> {
            Repository::open_opts(path, self)
        }
    }

    /// The error returned by [`Repository::open()`].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        Config(#[from] git_config::parser::ParserOrIoError<'static>),
        #[error(transparent)]
        NotARepository(#[from] crate::path::is::Error),
        #[error(transparent)]
        ObjectStoreInitialization(#[from] std::io::Error),
        #[error("Cannot handle objects formatted as {:?}", .name)]
        UnsupportedObjectFormat { name: crate::bstr::BString },
    }

    impl Repository {
        /// Open a git repository at the given `path`, possibly expanding it to `path/.git` if `path` is a work tree dir.
        pub fn open(path: impl Into<std::path::PathBuf>) -> Result<Self, Error> {
            Self::open_opts(path, Options::default())
        }

        /// Open a git repository at the given `path`, possibly expanding it to `path/.git` if `path` is a work tree dir.
        fn open_opts(path: impl Into<std::path::PathBuf>, options: Options) -> Result<Self, Error> {
            let path = path.into();
            let (path, kind) = match crate::path::is::git(&path) {
                Ok(kind) => (path, kind),
                Err(_) => {
                    let git_dir = path.join(".git");
                    crate::path::is::git(&git_dir).map(|kind| (git_dir, kind))?
                }
            };
            let (git_dir, worktree_dir) =
                crate::Path::from_dot_git_dir(path, kind).into_repository_and_work_tree_directories();
            Repository::open_from_paths(git_dir, worktree_dir, options)
        }

        pub(in crate::repository) fn open_from_paths(
            git_dir: PathBuf,
            mut worktree_dir: Option<PathBuf>,
            Options { object_store_slots }: Options,
        ) -> Result<Self, Error> {
            let config = git_config::file::GitConfig::open(git_dir.join("config"))?;
            if worktree_dir.is_none() {
                let is_bare = config_bool(&config, "core.bare", false);
                if !is_bare {
                    worktree_dir = Some(git_dir.parent().expect("parent is always available").to_owned());
                }
            }
            let use_multi_pack_index = config_bool(&config, "core.multiPackIndex", true);
            let repo_format_version = config
                .value::<Integer>("core", None, "repositoryFormatVersion")
                .map_or(0, |v| v.value);
            let object_hash = if repo_format_version == 1 {
                if let Ok(format) = config.value::<Cow<'_, [u8]>>("extensions", None, "objectFormat") {
                    match format.as_ref() {
                        b"sha1" => git_hash::Kind::Sha1,
                        _ => {
                            return Err(Error::UnsupportedObjectFormat {
                                name: format.to_vec().into(),
                            })
                        }
                    }
                } else {
                    git_hash::Kind::Sha1
                }
            } else {
                git_hash::Kind::Sha1
            };

            Ok(crate::Repository {
                objects: OwnShared::new(git_odb::Store::at_opts(
                    git_dir.join("objects"),
                    git_odb::store::init::Options {
                        slots: object_store_slots,
                        object_hash,
                        use_multi_pack_index,
                    },
                )?),
                refs: crate::RefStore::at(
                    git_dir,
                    if worktree_dir.is_none() {
                        git_ref::store::WriteReflog::Disable
                    } else {
                        git_ref::store::WriteReflog::Normal
                    },
                    object_hash,
                ),
                work_tree: worktree_dir,
                object_hash,
            })
        }
    }

    fn config_bool(config: &GitConfig<'_>, key: &str, default: bool) -> bool {
        let (section, key) = key.split_once(".").expect("valid section.key format");
        config
            .value::<Boolean<'_>>(section, None, key)
            .map_or(default, |b| matches!(b, Boolean::True(_)))
    }
}

///
pub mod init {
    use std::{convert::TryInto, path::Path};

    use crate::Repository;

    /// The error returned by [`Repository::init()`].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        Init(#[from] crate::path::create::Error),
        #[error(transparent)]
        Open(#[from] crate::open::Error),
    }

    impl Repository {
        /// Create a repository with work-tree within `directory`, creating intermediate directories as needed.
        ///
        /// Fails without action if there is already a `.git` repository inside of `directory`, but
        /// won't mind if the `directory` otherwise is non-empty.
        pub fn init(directory: impl AsRef<Path>, kind: crate::Kind) -> Result<Self, Error> {
            let path = crate::path::create::into(directory.as_ref(), kind)?;
            Ok(path.try_into()?)
        }
    }
}

mod location {
    use crate::Repository;

    impl Repository {
        /// The path to the `.git` directory itself, or equivalent if this is a bare repository.
        pub fn path(&self) -> &std::path::Path {
            self.git_dir()
        }

        /// Return the path to the repository itself, containing objects, references, configuration, and more.
        ///
        /// Synonymous to [`path()`][Repository::path()].
        pub fn git_dir(&self) -> &std::path::Path {
            self.refs.base()
        }

        /// Return the path to the working directory if this is not a bare repository.
        pub fn workdir(&self) -> Option<&std::path::Path> {
            self.work_tree.as_deref()
        }

        /// Return the path to the directory containing all objects.
        pub fn objects_dir(&self) -> &std::path::Path {
            self.objects.path()
        }
    }
}

///
pub mod discover {
    use std::{convert::TryInto, path::Path};

    use crate::{path::discover, Repository};

    /// The error returned by [`Repository::discover()`].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        Discover(#[from] discover::existing::Error),
        #[error(transparent)]
        Open(#[from] crate::open::Error),
    }

    impl Repository {
        /// Try to open a git repository in `directory` and search upwards through its parents until one is found.
        pub fn discover(directory: impl AsRef<Path>) -> Result<Self, Error> {
            let path = discover::existing(directory)?;
            Ok(path.try_into()?)
        }
    }
}

mod impls {
    use crate::Repository;

    impl std::fmt::Debug for Repository {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "Repository(git = '{}', working_tree: {:?}",
                self.git_dir().display(),
                self.work_tree
            )
        }
    }

    impl PartialEq<Repository> for Repository {
        fn eq(&self, other: &Repository) -> bool {
            self.git_dir() == other.git_dir() && self.work_tree == other.work_tree
        }
    }
}
