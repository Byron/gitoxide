use std::{borrow::Cow, path::PathBuf};

use git_config::{
    file::GitConfig,
    values::{Boolean, Integer},
};
use git_features::threading::OwnShared;

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
    pub fn open(self, path: impl Into<std::path::PathBuf>) -> Result<crate::ThreadSafeRepository, Error> {
        crate::ThreadSafeRepository::open_opts(path, self)
    }
}

/// The error returned by [`crate::open()`].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error(transparent)]
    Config(#[from] crate::config::open::Error),
    #[error(transparent)]
    NotARepository(#[from] crate::path::is::Error),
    #[error(transparent)]
    ObjectStoreInitialization(#[from] std::io::Error),
    #[error("Cannot handle objects formatted as {:?}", .name)]
    UnsupportedObjectFormat { name: crate::bstr::BString },
}

impl crate::ThreadSafeRepository {
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
        crate::ThreadSafeRepository::open_from_paths(git_dir, worktree_dir, options)
    }

    pub(crate) fn open_from_paths(
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

        Ok(crate::ThreadSafeRepository {
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
            config: config.into(),
        })
    }
}

fn config_bool(config: &GitConfig<'_>, key: &str, default: bool) -> bool {
    let (section, key) = key.split_once('.').expect("valid section.key format");
    config
        .value::<Boolean<'_>>(section, None, key)
        .map_or(default, |b| matches!(b, Boolean::True(_)))
}
