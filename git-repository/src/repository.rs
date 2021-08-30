#![allow(missing_docs)]
mod access {
    use crate::{Kind, Repository};

    impl Repository {
        pub fn kind(&self) -> Kind {
            match self.work_tree {
                Some(_) => Kind::WorkTree,
                None => Kind::Bare,
            }
        }

        pub fn git_dir(&self) -> &std::path::Path {
            &self.refs.base
        }
        pub fn objects_dir(&self) -> &std::path::Path {
            &self.odb.dbs[0].loose.path
        }
    }
}

pub mod from_path {
    use std::convert::TryFrom;

    use crate::Path;

    impl TryFrom<crate::Path> for crate::Repository {
        type Error = crate::open::Error;

        fn try_from(value: Path) -> Result<Self, Self::Error> {
            let (git_dir, worktree_dir) = value.into_repository_and_work_tree_directories();
            crate::Repository::open_from_paths(git_dir, worktree_dir)
        }
    }
}

pub mod open {
    use std::path::PathBuf;

    use git_config::values::Boolean;

    use crate::Repository;

    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error(transparent)]
        Config(#[from] git_config::parser::ParserOrIoError<'static>),
        #[error(transparent)]
        NotARepository(#[from] crate::path::is_git::Error),
        #[error(transparent)]
        ObjectStoreInitialization(#[from] git_odb::linked::init::Error),
    }

    impl Repository {
        pub fn open(path: impl Into<std::path::PathBuf>) -> Result<Self, Error> {
            let path = path.into();
            let (path, kind) = match crate::path::is_git(&path) {
                Ok(kind) => (path, kind),
                Err(_) => {
                    let git_dir = path.join(".git");
                    crate::path::is_git(&git_dir).map(|kind| (git_dir, kind))?
                }
            };
            let (git_dir, worktree_dir) =
                crate::Path::from_dot_git_dir(path, kind).into_repository_and_work_tree_directories();
            Repository::open_from_paths(git_dir, worktree_dir)
        }

        pub(in crate::repository) fn open_from_paths(
            git_dir: PathBuf,
            mut worktree_dir: Option<PathBuf>,
        ) -> Result<Self, Error> {
            if worktree_dir.is_none() {
                let config = git_config::file::GitConfig::open(git_dir.join("config"))?;
                let is_bare = config
                    .value::<Boolean<'_>>("core", None, "bare")
                    .map_or(false, |b| matches!(b, Boolean::True(_)));
                if !is_bare {
                    worktree_dir = Some(git_dir.parent().expect("parent is always available").to_owned());
                }
            }
            Ok(crate::Repository {
                odb: git_odb::linked::Store::at(git_dir.join("objects"))?,
                refs: git_ref::file::Store::at(
                    git_dir,
                    if worktree_dir.is_none() {
                        git_ref::file::WriteReflog::Disable
                    } else {
                        git_ref::file::WriteReflog::Normal
                    },
                ),
                work_tree: worktree_dir,
            })
        }
    }
}

pub mod init {
    use std::{convert::TryInto, path::Path};

    use crate::Repository;

    #[derive(Debug, thiserror::Error)]
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

pub mod discover {
    use std::{convert::TryInto, path::Path};

    use crate::{path::discover, Repository};

    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error(transparent)]
        Discover(#[from] discover::existing::Error),
        #[error(transparent)]
        Open(#[from] crate::open::Error),
    }

    impl Repository {
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
