#![allow(clippy::result_large_err)]
use std::{borrow::Cow, path::PathBuf};

use gix_features::threading::OwnShared;
use gix_macros::momo;

use super::{Error, Options};
use crate::{
    config,
    config::{
        cache::{interpolate_context, util::ApplyLeniency},
        tree::{gitoxide, Core, Key, Safe},
    },
    open::Permissions,
    ThreadSafeRepository,
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
    fn from_env() -> Result<Self, gix_sec::permission::Error<std::path::PathBuf>> {
        let mut worktree_dir = None;
        if let Some(path) = std::env::var_os(Core::WORKTREE.the_environment_override()) {
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
    ///
    /// ### Differences to `git2::Repository::open_ext()`
    ///
    /// Whereas `open_ext()` is the jack-of-all-trades that can do anything depending on its options, `gix` will always differentiate
    /// between discovering git repositories by searching, and opening a well-known repository by work tree or `.git` repository.
    ///
    /// Note that opening a repository for implementing custom hooks is also handle specifically in
    /// [`open_with_environment_overrides()`][Self::open_with_environment_overrides()].
    #[momo]
    pub fn open_opts(path: impl Into<PathBuf>, mut options: Options) -> Result<Self, Error> {
        let _span = gix_trace::coarse!("ThreadSafeRepository::open()");
        let (path, kind) = {
            let path = path.into();
            let looks_like_git_dir =
                path.ends_with(gix_discover::DOT_GIT_DIR) || path.extension() == Some(std::ffi::OsStr::new("git"));
            let candidate = if !options.open_path_as_is && !looks_like_git_dir {
                Cow::Owned(path.join(gix_discover::DOT_GIT_DIR))
            } else {
                Cow::Borrowed(&path)
            };
            match gix_discover::is_git(candidate.as_ref()) {
                Ok(kind) => (candidate.into_owned(), kind),
                Err(err) => {
                    if options.open_path_as_is || matches!(candidate, Cow::Borrowed(_)) {
                        return Err(Error::NotARepository {
                            source: err,
                            path: candidate.into_owned(),
                        });
                    }
                    match gix_discover::is_git(&path) {
                        Ok(kind) => (path, kind),
                        Err(err) => return Err(Error::NotARepository { source: err, path }),
                    }
                }
            }
        };
        let cwd = std::env::current_dir()?;
        let (git_dir, worktree_dir) = gix_discover::repository::Path::from_dot_git_dir(path, kind, &cwd)
            .expect("we have sanitized path with is_git()")
            .into_repository_and_work_tree_directories();
        if options.git_dir_trust.is_none() {
            options.git_dir_trust = gix_sec::Trust::from_path_ownership(&git_dir)?.into();
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
    // TODO: tests, with hooks, GIT_QUARANTINE for ref-log and transaction control (needs gix-sec support to remove write access in gix-ref)
    // TODO: The following vars should end up as overrides of the respective configuration values (see git-config).
    //       GIT_PROXY_SSL_CERT, GIT_PROXY_SSL_KEY, GIT_PROXY_SSL_CERT_PASSWORD_PROTECTED.
    //       GIT_PROXY_SSL_CAINFO, GIT_SSL_CIPHER_LIST, GIT_HTTP_MAX_REQUESTS, GIT_CURL_FTP_NO_EPSV,
    #[doc(alias = "open_from_env", alias = "git2")]
    #[momo]
    pub fn open_with_environment_overrides(
        fallback_directory: impl Into<PathBuf>,
        trust_map: gix_sec::trust::Mapping<Options>,
    ) -> Result<Self, Error> {
        let _span = gix_trace::coarse!("ThreadSafeRepository::open_with_environment_overrides()");
        let overrides = EnvironmentOverrides::from_env()?;
        let (path, path_kind): (PathBuf, _) = match overrides.git_dir {
            Some(git_dir) => gix_discover::is_git(&git_dir)
                .map_err(|err| Error::NotARepository {
                    source: err,
                    path: git_dir.clone(),
                })
                .map(|kind| (git_dir, kind))?,
            None => {
                let fallback_directory = fallback_directory.into();
                gix_discover::is_git(&fallback_directory)
                    .map_err(|err| Error::NotARepository {
                        source: err,
                        path: fallback_directory.clone(),
                    })
                    .map(|kind| (fallback_directory, kind))?
            }
        };

        let cwd = std::env::current_dir()?;
        let (git_dir, worktree_dir) = gix_discover::repository::Path::from_dot_git_dir(path, path_kind, &cwd)
            .expect("we have sanitized path with is_git()")
            .into_repository_and_work_tree_directories();
        let worktree_dir = worktree_dir.or(overrides.worktree_dir);

        let git_dir_trust = gix_sec::Trust::from_path_ownership(&git_dir)?;
        let mut options = trust_map.into_value_by_level(git_dir_trust);
        options.current_dir = Some(cwd);
        ThreadSafeRepository::open_from_paths(git_dir, worktree_dir, options)
    }

    pub(crate) fn open_from_paths(
        git_dir: PathBuf,
        mut worktree_dir: Option<PathBuf>,
        options: Options,
    ) -> Result<Self, Error> {
        let _span = gix_trace::detail!("open_from_paths()");
        let Options {
            git_dir_trust,
            object_store_slots,
            filter_config_section,
            lossy_config,
            lenient_config,
            bail_if_untrusted,
            open_path_as_is: _,
            permissions:
                Permissions {
                    ref env,
                    config,
                    attributes,
                },
            ref api_config_overrides,
            ref cli_config_overrides,
            ref current_dir,
        } = options;
        let current_dir = current_dir.as_deref().expect("BUG: current_dir must be set by caller");
        let git_dir_trust = git_dir_trust.expect("trust must be determined by now");

        // TODO: assure we handle the worktree-dir properly as we can have config per worktree with an extension.
        //       This would be something read in later as have to first check for extensions. Also this means
        //       that each worktree, even if accessible through this instance, has to come in its own Repository instance
        //       as it may have its own configuration. That's fine actually.
        let common_dir = gix_discover::path::from_plain_file(git_dir.join("commondir").as_ref())
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
            let reflog = repo_config.reflog.unwrap_or(gix_ref::store::WriteReflog::Disable);
            let object_hash = repo_config.object_hash;
            match &common_dir {
                Some(common_dir) => {
                    crate::RefStore::for_linked_worktree(git_dir.to_owned(), common_dir.into(), reflog, object_hash)
                }
                None => crate::RefStore::at(git_dir.to_owned(), reflog, object_hash),
            }
        };
        let head = refs.find("HEAD").ok();
        let git_install_dir = crate::path::install_dir().ok();
        let home = gix_path::env::home_dir().and_then(|home| env.home.check_opt(home));

        let mut filter_config_section = filter_config_section.unwrap_or(config::section::is_trusted);
        let config = config::Cache::from_stage_one(
            repo_config,
            common_dir_ref,
            head.as_ref().and_then(|head| head.target.try_name()),
            filter_config_section,
            git_install_dir.as_deref(),
            home.as_deref(),
            *env,
            attributes,
            config,
            lenient_config,
            api_config_overrides,
            cli_config_overrides,
        )?;

        if bail_if_untrusted && git_dir_trust != gix_sec::Trust::Full {
            check_safe_directories(
                &git_dir,
                git_install_dir.as_deref(),
                current_dir,
                home.as_deref(),
                &config,
            )?;
        }

        // core.worktree might be used to overwrite the worktree directory
        if !config.is_bare {
            if let Some(wt) = config
                .resolved
                .path_filter("core", None, Core::WORKTREE.name, &mut filter_config_section)
            {
                let wt_clone = wt.clone();
                let wt_path = wt
                    .interpolate(interpolate_context(git_install_dir.as_deref(), home.as_deref()))
                    .map_err(|err| config::Error::PathInterpolation {
                        path: wt_clone.value.into_owned(),
                        source: err,
                    })?;
                worktree_dir = gix_path::normalize(git_dir.join(wt_path).into(), current_dir).map(Cow::into_owned);
                #[allow(unused_variables)]
                if let Some(worktree_path) = worktree_dir.as_deref().filter(|wtd| !wtd.is_dir()) {
                    gix_trace::warn!("The configured worktree path '{}' is not a directory or doesn't exist - `core.worktree` may be misleading", worktree_path.display());
                }
            } else if !config.lenient_config
                && config
                    .resolved
                    .boolean_filter("core", None, Core::WORKTREE.name, &mut filter_config_section)
                    .is_some()
            {
                return Err(Error::from(config::Error::ConfigTypedString(
                    config::key::GenericErrorWithValue::from(&Core::WORKTREE),
                )));
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
                let _span = gix_trace::detail!("find replacement objects");
                let platform = refs.iter().ok()?;
                let iter = platform.prefixed(&prefix).ok()?;
                let prefix = prefix.to_str()?;
                let replacements = iter
                    .filter_map(Result::ok)
                    .filter_map(|r: gix_ref::Reference| {
                        let target = r.target.try_id()?.to_owned();
                        let source =
                            gix_hash::ObjectId::from_hex(r.name.as_bstr().strip_prefix(prefix.as_bytes())?).ok()?;
                        Some((source, target))
                    })
                    .collect::<Vec<_>>();
                Some(replacements)
            })
            .unwrap_or_default();

        Ok(ThreadSafeRepository {
            objects: OwnShared::new(gix_odb::Store::at_opts(
                common_dir_ref.join("objects"),
                &mut replacements.into_iter(),
                gix_odb::store::init::Options {
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
            #[cfg(feature = "index")]
            index: gix_fs::SharedFileSnapshotMut::new().into(),
            shallow_commits: gix_fs::SharedFileSnapshotMut::new().into(),
            #[cfg(feature = "attributes")]
            modules: gix_fs::SharedFileSnapshotMut::new().into(),
        })
    }
}

// TODO: tests
fn replacement_objects_refs_prefix(
    config: &gix_config::File<'static>,
    lenient: bool,
    mut filter_config_section: fn(&gix_config::file::Metadata) -> bool,
) -> Result<Option<PathBuf>, Error> {
    let is_disabled = config
        .boolean_filter_by_key("core.useReplaceRefs", &mut filter_config_section)
        .map(|b| Core::USE_REPLACE_REFS.enrich_error(b))
        .transpose()
        .with_leniency(lenient)
        .map_err(config::Error::ConfigBoolean)?
        .unwrap_or(true);

    if is_disabled {
        return Ok(None);
    }

    let ref_base = gix_path::from_bstr({
        let key = "gitoxide.objects.replaceRefBase";
        debug_assert_eq!(gitoxide::Objects::REPLACE_REF_BASE.logical_name(), key);
        config
            .string_filter_by_key(key, &mut filter_config_section)
            .unwrap_or_else(|| Cow::Borrowed("refs/replace/".into()))
    })
    .into_owned();
    Ok(ref_base.into())
}

fn check_safe_directories(
    git_dir: &std::path::Path,
    git_install_dir: Option<&std::path::Path>,
    current_dir: &std::path::Path,
    home: Option<&std::path::Path>,
    config: &config::Cache,
) -> Result<(), Error> {
    let mut is_safe = false;
    let git_dir = match gix_path::realpath_opts(git_dir, current_dir, gix_path::realpath::MAX_SYMLINKS) {
        Ok(p) => p,
        Err(_) => git_dir.to_owned(),
    };
    for safe_dir in config
        .resolved
        .strings_filter("safe", None, Safe::DIRECTORY.name, &mut Safe::directory_filter)
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
            let safe_dir = match gix_config::Path::from(std::borrow::Cow::Borrowed(safe_dir.as_ref()))
                .interpolate(interpolate_context(git_install_dir, home))
            {
                Ok(path) => path,
                Err(_) => gix_path::from_bstr(safe_dir),
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
