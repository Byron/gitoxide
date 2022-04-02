use std::{borrow::Cow, path::PathBuf};

use git_config::{
    file::GitConfig,
    values::{Boolean, Integer},
};
use git_features::threading::OwnShared;

/// A way to configure the usage of replacement objects, see `git replace`.
pub enum ReplacementObjects {
    /// Allow replacement objects and configure the ref prefix the standard environment variable `GIT_REPLACE_REF_BASE`,
    /// or default to the standard `refs/replace/` prefix.
    UseWithEnvironmentRefPrefixOrDefault {
        /// If true, default true, a standard environment variable `GIT_NO_REPLACE_OBJECTS`
        allow_disable_via_environment: bool,
    },
    /// Use replacement objects and configure the prefix yourself.
    UseWithRefPrefix {
        /// The ref prefix to use, like `refs/alternative/` - note the trailing slash.
        prefix: PathBuf,
        /// If true, default true, a standard environment variable `GIT_NO_REPLACE_OBJECTS`
        allow_disable_via_environment: bool,
    },
    /// Do not use replacement objects at all.
    Disable,
}

impl Default for ReplacementObjects {
    fn default() -> Self {
        ReplacementObjects::UseWithEnvironmentRefPrefixOrDefault {
            allow_disable_via_environment: true,
        }
    }
}

impl ReplacementObjects {
    fn refs_prefix(self) -> Option<PathBuf> {
        use ReplacementObjects::*;
        let is_disabled = |allow_env: bool| allow_env && std::env::var_os("GIT_NO_REPLACE_OBJECTS").is_some();
        match self {
            UseWithEnvironmentRefPrefixOrDefault {
                allow_disable_via_environment,
            } => {
                if is_disabled(allow_disable_via_environment) {
                    return None;
                };
                PathBuf::from(std::env::var("GIT_REPLACE_REF_BASE").unwrap_or_else(|_| "refs/replace/".into())).into()
            }
            UseWithRefPrefix {
                prefix,
                allow_disable_via_environment,
            } => {
                if is_disabled(allow_disable_via_environment) {
                    return None;
                };
                prefix.into()
            }
            Disable => None,
        }
    }
}

/// The options used in [`Repository::open_opts
#[derive(Default)]
pub struct Options {
    object_store_slots: git_odb::store::init::Slots,
    replacement_objects: ReplacementObjects,
}

impl Options {
    /// Set the amount of slots to use for the object database. It's a value that doesn't need changes on the client, typically,
    /// but should be controlled on the server.
    pub fn object_store_slots(mut self, slots: git_odb::store::init::Slots) -> Self {
        self.object_store_slots = slots;
        self
    }

    /// Configure replacement objects, see the [`ReplacementObjects`] type for details.
    pub fn replacement_objects(mut self, config: ReplacementObjects) -> Self {
        self.replacement_objects = config;
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
        Options {
            object_store_slots,
            replacement_objects,
        }: Options,
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

        let refs = crate::RefStore::at(
            &git_dir,
            if worktree_dir.is_none() {
                git_ref::store::WriteReflog::Disable
            } else {
                git_ref::store::WriteReflog::Normal
            },
            object_hash,
        );

        let replacements = replacement_objects
            .refs_prefix()
            .and_then(|prefix| {
                let platform = refs.iter().ok()?;
                let iter = platform.prefixed(&prefix).ok()?;
                let prefix = prefix.to_str()?;
                let replacements = iter
                    .filter_map(Result::ok)
                    .filter_map(|r: git_ref::Reference| {
                        let target = r.target.try_id()?.to_owned();
                        let source =
                            git_hash::ObjectId::from_hex(r.name.as_bstr().strip_prefix(prefix.as_bytes())?).ok()?;
                        Some((source, target))
                    })
                    .collect::<Vec<_>>();
                Some(replacements)
            })
            .unwrap_or_default();

        Ok(crate::ThreadSafeRepository {
            objects: OwnShared::new(git_odb::Store::at_opts(
                git_dir.join("objects"),
                replacements,
                git_odb::store::init::Options {
                    slots: object_store_slots,
                    object_hash,
                    use_multi_pack_index,
                },
            )?),
            refs,
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
