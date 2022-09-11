use std::path::PathBuf;

use git_features::threading::OwnShared;

use crate::{config, config::cache::interpolate_context, permission, Permissions, ThreadSafeRepository};

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

/// The options used in [`ThreadSafeRepository::open_opts`]
#[derive(Clone)]
pub struct Options {
    pub(crate) object_store_slots: git_odb::store::init::Slots,
    pub(crate) replacement_objects: ReplacementObjects,
    /// Define what is allowed while openeing a repository.
    pub permissions: Permissions,
    pub(crate) git_dir_trust: Option<git_sec::Trust>,
    /// Warning: this one is copied to to config::Cache - don't change it after repo open or keep in sync.
    pub(crate) filter_config_section: Option<fn(&git_config::file::Metadata) -> bool>,
    pub(crate) lossy_config: Option<bool>,
    pub(crate) lenient_config: bool,
    pub(crate) bail_if_untrusted: bool,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            object_store_slots: Default::default(),
            replacement_objects: Default::default(),
            permissions: Default::default(),
            git_dir_trust: None,
            filter_config_section: None,
            lossy_config: None,
            lenient_config: true,
            bail_if_untrusted: false,
        }
    }
}

#[derive(Default, Clone)]
#[allow(dead_code)]
pub(crate) struct EnvironmentOverrides {
    /// An override of the worktree typically from the environment, and overrides even worktree dirs set as parameter.
    ///
    /// This emulates the way git handles this override.
    worktree_dir: Option<PathBuf>,
    /// An override for the .git directory, typically from the environment.
    ///
    /// If set, the passed in `git_dir` parameter will be ignored in favor of this one.
    git_dir: Option<PathBuf>,
}

impl EnvironmentOverrides {
    fn from_env() -> Result<Self, permission::env_var::resource::Error> {
        let mut worktree_dir = None;
        if let Some(path) = std::env::var_os("GIT_WORK_TREE") {
            worktree_dir = PathBuf::from(path).into();
        }
        let mut git_dir = None;
        if let Some(path) = std::env::var_os("GIT_DIR") {
            git_dir = PathBuf::from(path).into();
        }
        Ok(EnvironmentOverrides { worktree_dir, git_dir })
    }
}

/// Instantiation
impl Options {
    /// Options configured to prevent accessing anything else than the repository configuration file, prohibiting
    /// accessing the environment or spreading beyond the git repository location.
    pub fn isolated() -> Self {
        Options::default().permissions(Permissions::isolated())
    }
}

/// Builder methods
impl Options {
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
    pub fn permissions(mut self, permissions: Permissions) -> Self {
        self.permissions = permissions;
        self
    }

    /// Set the trust level of the `.git` directory we are about to open.
    ///
    /// This can be set manually to force trust even though otherwise it might
    /// not be fully trusted, leading to limitations in how configuration files
    /// are interpreted.
    ///
    /// If not called explicitly, it will be determined by looking at its
    /// ownership via [`git_sec::Trust::from_path_ownership()`].
    ///
    /// # Security Warning
    ///
    /// Use with extreme care and only if it's absolutely known that the repository
    /// is always controlled by the desired user. Using this capability _only_ saves
    /// a permission check and only so if the [`open()`][Self::open()] method is used,
    /// as opposed to discovery.
    pub fn with(mut self, trust: git_sec::Trust) -> Self {
        self.git_dir_trust = trust.into();
        self
    }

    /// If true, default false, and if the repository's trust level is not `Full`
    /// (see [`with()`][Self::with()] for more), then the open operation will fail.
    ///
    /// Use this to mimic `git`s way of handling untrusted repositories. Note that `gitoxide` solves
    /// this by not using configuration from untrusted sources and by generally being secured against
    /// doctored input files which at worst could cause out-of-memory at the time of writing.
    pub fn bail_if_untrusted(mut self, toggle: bool) -> Self {
        self.bail_if_untrusted = toggle;
        self
    }

    /// Set the filter which determines if a configuration section can be used to read values from,
    /// hence it returns true if it is eligible.
    ///
    /// The default filter selects sections whose trust level is [`full`][git_sec::Trust::Full] or
    /// whose source is not [`repository-local`][git_config::source::Kind::Repository].
    pub fn filter_config_section(mut self, filter: fn(&git_config::file::Metadata) -> bool) -> Self {
        self.filter_config_section = Some(filter);
        self
    }

    /// By default, in release mode configuration will be read without retaining non-essential information like
    /// comments or whitespace to optimize lookup performance.
    ///
    /// Some application might want to toggle this to false in they want to display or edit configuration losslessly
    /// with all whitespace and comments included.
    pub fn lossy_config(mut self, toggle: bool) -> Self {
        self.lossy_config = toggle.into();
        self
    }

    /// If set, default is false, invalid configuration values will cause an error even if these can safely be defaulted.
    ///
    /// This is recommended for all applications that prefer correctness over usability.
    /// `git` itself by defaults to strict configuration mode to let you know if configuration is incorrect.
    pub fn strict_config(mut self, toggle: bool) -> Self {
        self.lenient_config = !toggle;
        self
    }

    /// Open a repository at `path` with the options set so far.
    pub fn open(self, path: impl Into<PathBuf>) -> Result<ThreadSafeRepository, Error> {
        ThreadSafeRepository::open_opts(path, self)
    }
}

impl git_sec::trust::DefaultForLevel for Options {
    fn default_for_level(level: git_sec::Trust) -> Self {
        match level {
            git_sec::Trust::Full => Options {
                object_store_slots: Default::default(),
                replacement_objects: Default::default(),
                permissions: Permissions::default_for_level(level),
                git_dir_trust: git_sec::Trust::Full.into(),
                filter_config_section: Some(config::section::is_trusted),
                lossy_config: None,
                bail_if_untrusted: false,
                lenient_config: true,
            },
            git_sec::Trust::Reduced => Options {
                object_store_slots: git_odb::store::init::Slots::Given(32), // limit resource usage
                replacement_objects: ReplacementObjects::Disable, // don't be tricked into seeing manufactured objects
                permissions: Permissions::default_for_level(level),
                git_dir_trust: git_sec::Trust::Reduced.into(),
                filter_config_section: Some(config::section::is_trusted),
                bail_if_untrusted: false,
                lenient_config: true,
                lossy_config: None,
            },
        }
    }
}

/// The error returned by [`crate::open()`].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("Failed to load the git configuration")]
    Config(#[from] config::Error),
    #[error(transparent)]
    NotARepository(#[from] git_discover::is_git::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("The git directory at '{}' is considered unsafe as it's not owned by the current user.", .path.display())]
    UnsafeGitDir { path: PathBuf },
    #[error(transparent)]
    EnvironmentAccessDenied(#[from] permission::env_var::resource::Error),
}

impl ThreadSafeRepository {
    /// Open a git repository at the given `path`, possibly expanding it to `path/.git` if `path` is a work tree dir.
    pub fn open(path: impl Into<PathBuf>) -> Result<Self, Error> {
        Self::open_opts(path, Options::default())
    }

    /// Open a git repository at the given `path`, possibly expanding it to `path/.git` if `path` is a work tree dir, and use
    /// `options` for fine-grained control.
    ///
    /// Note that you should use [`crate::discover()`] if security should be adjusted by ownership.
    pub fn open_opts(path: impl Into<PathBuf>, mut options: Options) -> Result<Self, Error> {
        let (path, kind) = {
            let path = path.into();
            match git_discover::is_git(&path) {
                Ok(kind) => (path, kind),
                Err(_err) => {
                    let git_dir = path.join(git_discover::DOT_GIT_DIR);
                    git_discover::is_git(&git_dir).map(|kind| (git_dir, kind))?
                }
            }
        };
        let (git_dir, worktree_dir) =
            git_discover::repository::Path::from_dot_git_dir(path, kind).into_repository_and_work_tree_directories();
        if options.git_dir_trust.is_none() {
            options.git_dir_trust = git_sec::Trust::from_path_ownership(&git_dir)?.into();
        }
        ThreadSafeRepository::open_from_paths(git_dir, worktree_dir, options)
    }

    /// Try to open a git repository in `fallback_directory` (can be worktree or `.git` directory) only if there is no override
    /// from of the `gitdir` using git environment variables.
    ///
    /// Use the `trust_map` to apply options depending in the trust level for `directory` or the directory it's overridden with.
    /// The `.git` directory whether given or computed is used for trust checks.
    ///
    /// Note that this will read various `GIT_*` environment variables to check for overrides, and is probably most useful when implementing
    /// custom hooks.
    // TODO: tests, with hooks, GIT_QUARANTINE for ref-log and transaction control (needs git-sec support to remove write access in git-ref)
    pub fn open_with_environment_overrides(
        fallback_directory: impl Into<PathBuf>,
        trust_map: git_sec::trust::Mapping<Options>,
    ) -> Result<Self, Error> {
        let overrides = EnvironmentOverrides::from_env()?;
        let (path, path_kind): (PathBuf, _) = match overrides.git_dir {
            Some(git_dir) => git_discover::is_git(&git_dir).map(|kind| (git_dir, kind))?,
            None => {
                let fallback_directory = fallback_directory.into();
                git_discover::is_git(&fallback_directory).map(|kind| (fallback_directory, kind))?
            }
        };

        let (git_dir, worktree_dir) = git_discover::repository::Path::from_dot_git_dir(path, path_kind)
            .into_repository_and_work_tree_directories();
        let worktree_dir = worktree_dir.or(overrides.worktree_dir);

        let git_dir_trust = git_sec::Trust::from_path_ownership(&git_dir)?;
        let options = trust_map.into_value_by_level(git_dir_trust);
        ThreadSafeRepository::open_from_paths(git_dir, worktree_dir, options)
    }

    pub(crate) fn open_from_paths(
        git_dir: PathBuf,
        mut worktree_dir: Option<PathBuf>,
        options: Options,
    ) -> Result<Self, Error> {
        let Options {
            git_dir_trust,
            object_store_slots,
            filter_config_section,
            ref replacement_objects,
            lossy_config,
            lenient_config,
            bail_if_untrusted,
            permissions: Permissions { ref env, config },
        } = options;
        let git_dir_trust = git_dir_trust.expect("trust must be been determined by now");

        // TODO: assure we handle the worktree-dir properly as we can have config per worktree with an extension.
        //       This would be something read in later as have to first check for extensions. Also this means
        //       that each worktree, even if accessible through this instance, has to come in its own Repository instance
        //       as it may have its own configuration. That's fine actually.
        let common_dir = git_discover::path::from_plain_file(git_dir.join("commondir"))
            .transpose()?
            .map(|cd| git_dir.join(cd));
        let common_dir_ref = common_dir.as_deref().unwrap_or(&git_dir);

        let repo_config = config::cache::StageOne::new(common_dir_ref, git_dir_trust, lossy_config, lenient_config)?;
        let mut refs = {
            let reflog = repo_config.reflog.unwrap_or(git_ref::store::WriteReflog::Disable);
            let object_hash = repo_config.object_hash;
            match &common_dir {
                Some(common_dir) => crate::RefStore::for_linked_worktree(&git_dir, common_dir, reflog, object_hash),
                None => crate::RefStore::at(&git_dir, reflog, object_hash),
            }
        };
        let head = refs.find("HEAD").ok();
        let git_install_dir = crate::path::install_dir().ok();
        let home = std::env::var_os("HOME")
            .map(PathBuf::from)
            .and_then(|home| env.home.check_opt(home));

        let mut filter_config_section = filter_config_section.unwrap_or(config::section::is_trusted);
        let config = config::Cache::from_stage_one(
            repo_config,
            common_dir_ref,
            head.as_ref().and_then(|head| head.target.try_name()),
            filter_config_section,
            git_install_dir.as_deref(),
            home.as_deref(),
            env.clone(),
            config,
            lenient_config,
        )?;

        if bail_if_untrusted && git_dir_trust != git_sec::Trust::Full {
            check_safe_directories(&git_dir, git_install_dir.as_deref(), home.as_deref(), &config)?;
        }

        // core.worktree might be used to overwrite the worktree directory
        if !config.is_bare {
            if let Some(wt) = config
                .resolved
                .path_filter("core", None, "worktree", &mut filter_config_section)
            {
                let wt_path = wt
                    .interpolate(interpolate_context(git_install_dir.as_deref(), home.as_deref()))
                    .map_err(config::Error::PathInterpolation)?;
                worktree_dir = {
                    let wt = git_path::absolutize(git_dir.join(wt_path), None::<std::path::PathBuf>).into_owned();
                    wt.is_dir().then(|| wt)
                }
            }
        }

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

        refs.write_reflog = config.reflog.unwrap_or_else(|| {
            if worktree_dir.is_none() {
                git_ref::store::WriteReflog::Disable
            } else {
                git_ref::store::WriteReflog::Normal
            }
        });

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

        Ok(ThreadSafeRepository {
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
            // used when spawning new repositories off this one when following worktrees
            linked_worktree_options: options,
            index: git_features::fs::MutableSnapshot::new().into(),
        })
    }
}

fn check_safe_directories(
    git_dir: &std::path::Path,
    git_install_dir: Option<&std::path::Path>,
    home: Option<&std::path::Path>,
    config: &config::Cache,
) -> Result<(), Error> {
    let mut is_safe = false;
    let git_dir = match git_path::realpath(git_dir) {
        Ok(p) => p,
        Err(_) => git_dir.to_owned(),
    };
    for safe_dir in config
        .resolved
        .strings_filter("safe", None, "directory", &mut |meta| {
            let kind = meta.source.kind();
            kind == git_config::source::Kind::System || kind == git_config::source::Kind::Global
        })
        .unwrap_or_default()
    {
        if safe_dir.as_ref() == "*" {
            is_safe = true;
            continue;
        }
        if safe_dir.is_empty() {
            is_safe = false;
            continue;
        }
        if !is_safe {
            let safe_dir = match git_config::Path::from(std::borrow::Cow::Borrowed(safe_dir.as_ref()))
                .interpolate(interpolate_context(git_install_dir, home))
            {
                Ok(path) => path,
                Err(_) => git_path::from_bstr(safe_dir),
            };
            if safe_dir == git_dir {
                is_safe = true;
                continue;
            }
        }
    }
    if is_safe {
        Ok(())
    } else {
        Err(Error::UnsafeGitDir { path: git_dir })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_of_options() {
        assert_eq!(
            std::mem::size_of::<Options>(),
            72,
            "size shouldn't change without us knowing"
        );
    }
}
