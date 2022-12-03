use std::{borrow::Cow, path::PathBuf};

use git_features::threading::OwnShared;

use super::{Error, Options};
use crate::{
    config,
    config::cache::{interpolate_context, util::ApplyLeniency},
    permission, Permissions, ThreadSafeRepository,
};

#[derive(Default, Clone)]
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
        let cwd = std::env::current_dir()?;
        let (git_dir, worktree_dir) = git_discover::repository::Path::from_dot_git_dir(path, kind, &cwd)
            .expect("we have sanitized path with is_git()")
            .into_repository_and_work_tree_directories();
        if options.git_dir_trust.is_none() {
            options.git_dir_trust = git_sec::Trust::from_path_ownership(&git_dir)?.into();
        }
        options.current_dir = Some(cwd);
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
    // TODO: The following vars should end up as overrides of the respective configuration values (see git-config).
    //       GIT_PROXY_SSL_CERT, GIT_PROXY_SSL_KEY, GIT_PROXY_SSL_CERT_PASSWORD_PROTECTED.
    //       GIT_PROXY_SSL_CAINFO, GIT_SSL_VERSION, GIT_SSL_CIPHER_LIST, GIT_HTTP_MAX_REQUESTS, GIT_CURL_FTP_NO_EPSV,
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

        let cwd = std::env::current_dir()?;
        let (git_dir, worktree_dir) = git_discover::repository::Path::from_dot_git_dir(path, path_kind, &cwd)
            .expect("we have sanitized path with is_git()")
            .into_repository_and_work_tree_directories();
        let worktree_dir = worktree_dir.or(overrides.worktree_dir);

        let git_dir_trust = git_sec::Trust::from_path_ownership(&git_dir)?;
        let mut options = trust_map.into_value_by_level(git_dir_trust);
        options.current_dir = Some(cwd);
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
            lossy_config,
            lenient_config,
            bail_if_untrusted,
            permissions: Permissions { ref env, config },
            ref api_config_overrides,
            ref cli_config_overrides,
            ref current_dir,
        } = options;
        let current_dir = current_dir.as_deref().expect("BUG: current_dir must be set by caller");
        let git_dir_trust = git_dir_trust.expect("trust must be been determined by now");

        // TODO: assure we handle the worktree-dir properly as we can have config per worktree with an extension.
        //       This would be something read in later as have to first check for extensions. Also this means
        //       that each worktree, even if accessible through this instance, has to come in its own Repository instance
        //       as it may have its own configuration. That's fine actually.
        let common_dir = git_discover::path::from_plain_file(git_dir.join("commondir"))
            .transpose()?
            .map(|cd| git_dir.join(cd));
        let common_dir_ref = common_dir.as_deref().unwrap_or(&git_dir);

        let repo_config = config::cache::StageOne::new(
            common_dir_ref,
            git_dir.as_ref(),
            git_dir_trust,
            lossy_config,
            lenient_config,
        )?;
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
            api_config_overrides,
            cli_config_overrides,
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
                    git_path::normalize(git_dir.join(wt_path), current_dir)
                        .and_then(|wt| wt.as_ref().is_dir().then(|| wt.into_owned()))
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

        refs.write_reflog = config::cache::util::reflog_or_default(config.reflog, worktree_dir.is_some());
        let replacements = replacement_objects_refs_prefix(&config.resolved, lenient_config, filter_config_section)?
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
                    current_dir: current_dir.to_owned().into(),
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

// TODO: tests
fn replacement_objects_refs_prefix(
    config: &git_config::File<'static>,
    lenient: bool,
    mut filter_config_section: fn(&git_config::file::Metadata) -> bool,
) -> Result<Option<PathBuf>, Error> {
    let key = "gitoxide.objects.noReplace";
    let is_disabled = config
        .boolean_filter_by_key(key, &mut filter_config_section)
        .transpose()
        .with_leniency(lenient)
        .map_err(|err| config::Error::Value { source: err, key })?
        .unwrap_or_default();

    if is_disabled {
        return Ok(None);
    }

    let ref_base = git_path::from_bstr(
        config
            .string_filter_by_key("gitoxide.objects.replaceRefBase", &mut filter_config_section)
            .unwrap_or_else(|| Cow::Borrowed("refs/replace/".into())),
    )
    .into_owned();
    Ok(ref_base.into())
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
