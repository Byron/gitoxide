use std::borrow::Cow;
use std::path::PathBuf;

use git_features::threading::OwnShared;
use git_sec::Trust;

use crate::Permissions;

/// A way to configure the usage of replacement objects, see `git replace`.
#[derive(Debug, Clone)]
pub enum ReplacementObjects {
    /// Allow replacement objects and configure the ref prefix the standard environment variable `GIT_REPLACE_REF_BASE`,
    /// or default to the standard `refs/replace/` prefix.
    UseWithEnvironmentRefPrefixOrDefault {
        /// If true, default true, a standard environment variable `GIT_NO_REPLACE_OBJECTS` to disable replacement objects entirely.
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
#[derive(Default, Clone)]
pub struct Options {
    object_store_slots: git_odb::store::init::Slots,
    replacement_objects: ReplacementObjects,
    permissions: crate::Permissions,
    /// The path to the main .git repository and typically discovered by reading `commondir` files.
    common_dir: Option<PathBuf>,
    /// An override of the worktree typically from the environment, and overrides even worktree dirs set as parameter.
    ///
    /// This emulates the way git handles this override.
    worktree_dir_override: Option<PathBuf>,
    /// An override for the .git directory, typically from the environment.
    ///
    /// If set, the passed in `git_dir` parameter will be ignored.
    git_dir_override: Option<PathBuf>,
}

impl Options {
    /// Apply common environment variables to this instance, setting only values which aren't already set.
    // TODO: tests
    pub fn apply_environment(mut self) -> Result<Self, crate::permission::env_var::resource::Error> {
        if self.common_dir.is_none() {
            if let Some(path) = std::env::var_os("GIT_COMMON_DIR") {
                self.common_dir = self.permissions.env.with_git_prefix.check(PathBuf::from(path))?;
            }
        }
        if self.worktree_dir_override.is_none() {
            if let Some(path) = std::env::var_os("GIT_WORK_TREE") {
                self.worktree_dir_override = self.permissions.env.with_git_prefix.check(PathBuf::from(path))?;
            }
        }
        if self.git_dir_override.is_none() {
            if let Some(path) = std::env::var_os("GIT_DIR") {
                self.git_dir_override = self.permissions.env.with_git_prefix.check(PathBuf::from(path))?;
            }
        }
        Ok(self)
    }
    /// Set the amount of slots to use for the object database. It's a value that doesn't need changes on the client, typically,
    /// but should be controlled on the server.
    pub fn object_store_slots(mut self, slots: git_odb::store::init::Slots) -> Self {
        self.object_store_slots = slots;
        self
    }

    // TODO: tests
    /// Configure replacement objects, see the [`ReplacementObjects`] type for details.
    pub fn replacement_objects(mut self, config: ReplacementObjects) -> Self {
        self.replacement_objects = config;
        self
    }

    // TODO: tests
    /// Set the given permissions, which are typically derived by a `Trust` level.
    pub fn permissions(mut self, permissions: crate::Permissions) -> Self {
        self.permissions = permissions;
        self
    }

    /// Open a repository at `path` with the options set so far.
    pub fn open(self, path: impl Into<std::path::PathBuf>) -> Result<crate::ThreadSafeRepository, Error> {
        crate::ThreadSafeRepository::open_opts(path, self)
    }
}

impl git_sec::trust::DefaultForLevel for Options {
    fn default_for_level(level: Trust) -> Self {
        match level {
            git_sec::Trust::Full => Options {
                object_store_slots: Default::default(),
                replacement_objects: Default::default(),
                permissions: Permissions::all(),
                common_dir: None,
                worktree_dir_override: None,
                git_dir_override: None,
            },
            git_sec::Trust::Reduced => Options {
                object_store_slots: git_odb::store::init::Slots::Given(32), // limit resource usage
                replacement_objects: ReplacementObjects::Disable, // don't be tricked into seeing manufactured objects
                permissions: Default::default(),
                common_dir: None,
                worktree_dir_override: None,
                git_dir_override: None,
            },
        }
    }
}

/// The error returned by [`crate::open()`].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error(transparent)]
    Config(#[from] crate::config::Error),
    #[error(transparent)]
    NotARepository(#[from] git_discover::is_git::Error),
    #[error(transparent)]
    ObjectStoreInitialization(#[from] std::io::Error),
    #[error("The git directory at '{}' is considered unsafe as it's not owned by the current user.", .path.display())]
    UnsafeGitDir { path: std::path::PathBuf },
}

impl crate::ThreadSafeRepository {
    /// Open a git repository at the given `path`, possibly expanding it to `path/.git` if `path` is a work tree dir.
    pub fn open(path: impl Into<std::path::PathBuf>) -> Result<Self, Error> {
        Self::open_opts(path, Options::default())
    }

    /// Open a git repository at the given `path`, possibly expanding it to `path/.git` if `path` is a work tree dir, and use
    /// `options` for fine-grained control.
    pub fn open_opts(path: impl Into<std::path::PathBuf>, options: Options) -> Result<Self, Error> {
        let path = path.into();
        let (path, kind) = match git_discover::is_git(&path) {
            Ok(kind) => (path, kind),
            Err(_) => {
                let git_dir = path.join(".git");
                git_discover::is_git(&git_dir).map(|kind| (git_dir, kind))?
            }
        };
        let (git_dir, worktree_dir) =
            git_discover::repository::Path::from_dot_git_dir(path, kind).into_repository_and_work_tree_directories();
        crate::ThreadSafeRepository::open_from_paths(git_dir, worktree_dir, options)
    }

    pub(crate) fn open_from_paths(
        git_dir: PathBuf,
        worktree_dir: Option<PathBuf>,
        Options {
            object_store_slots,
            replacement_objects,
            common_dir,
            worktree_dir_override,
            git_dir_override,
            permissions: Permissions {
                git_dir: git_dir_perm,
                env,
            },
        }: Options,
    ) -> Result<Self, Error> {
        if *git_dir_perm != git_sec::ReadWrite::all() {
            // TODO: respect `save.directory`, which needs more support from git-config to do properly.
            return Err(Error::UnsafeGitDir { path: git_dir });
        }
        // TODO: assure we handle the worktree-dir properly as we can have config per worktree with an extension.
        //       This would be something read in later as have to first check for extensions. Also this means
        //       that each worktree, even if accessible through this instance, has to come in its own Repository instance
        //       as it may have its own configuration. That's fine actually.
        let git_dir = git_dir_override.unwrap_or(git_dir);
        let mut worktree_dir = worktree_dir_override.map(|wt| git_dir.join(wt)).or(worktree_dir);
        let common_dir = common_dir
            .map(|common| Ok(git_dir.join(common)))
            .or_else(|| crate::path::read_from_file(git_dir.join("commondir")))
            .transpose()?;
        let common_dir_ref = common_dir
            .as_deref()
            .map(|cmn| Cow::Owned(git_dir.join(cmn)))
            .unwrap_or_else(|| Cow::Borrowed(&git_dir));
        let config = crate::config::Cache::new(
            &common_dir_ref,
            env.xdg_config_home.clone(),
            env.home.clone(),
            crate::path::install_dir().ok().as_deref(),
        )?;
        match worktree_dir {
            None if !config.is_bare => {
                worktree_dir = Some(git_dir.parent().expect("parent is always available").to_owned());
            }
            Some(_) => {
                // note that we might be bare even with a worktree directory - work trees don't have to be
                // the parent of a non-bare repository.
            }
            None => {}
        }

        let refs = crate::RefStore::at(
            &git_dir,
            if worktree_dir.is_none() {
                git_ref::store::WriteReflog::Disable
            } else {
                git_ref::store::WriteReflog::Normal
            },
            config.object_hash,
        );

        let replacements = replacement_objects
            .clone()
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

        // used when spawning new repositories off this one when following worktrees
        let linked_worktree_options = Options {
            object_store_slots,
            replacement_objects,
            permissions: Permissions {
                env,
                git_dir: git_dir_perm,
            },
            common_dir: None,
            worktree_dir_override: None,
            git_dir_override: None,
        };

        Ok(crate::ThreadSafeRepository {
            objects: OwnShared::new(git_odb::Store::at_opts(
                common_dir_ref.join("objects"),
                replacements,
                git_odb::store::init::Options {
                    slots: object_store_slots,
                    object_hash: config.object_hash,
                    use_multi_pack_index: config.use_multi_pack_index,
                },
            )?),
            common_dir,
            refs,
            work_tree: worktree_dir,
            config,
            linked_worktree_options,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_of_options() {
        assert_eq!(
            std::mem::size_of::<Options>(),
            128,
            "size shouldn't change without us knowing"
        );
    }
}
