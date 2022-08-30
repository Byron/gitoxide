use super::{interpolate_context, util, Error, StageOne};
use crate::config::Cache;
use crate::{repository, revision::spec::parse::ObjectKindHint};
use std::path::PathBuf;

/// Initialization
impl Cache {
    #[allow(clippy::too_many_arguments)]
    pub fn from_stage_one(
        StageOne {
            git_dir_config,
            mut buf,
            lossy,
            is_bare,
            object_hash,
            reflog: _,
        }: StageOne,
        git_dir: &std::path::Path,
        branch_name: Option<&git_ref::FullNameRef>,
        mut filter_config_section: fn(&git_config::file::Metadata) -> bool,
        git_install_dir: Option<&std::path::Path>,
        home: Option<&std::path::Path>,
        repository::permissions::Environment {
            git_prefix,
            home: home_env,
            xdg_config_home: xdg_config_home_env,
        }: repository::permissions::Environment,
        repository::permissions::Config {
            system: use_system,
            git: use_git,
            user: use_user,
            env: use_env,
            includes: use_includes,
        }: repository::permissions::Config,
        lenient_config: bool,
    ) -> Result<Self, Error> {
        let options = git_config::file::init::Options {
            includes: if use_includes {
                git_config::file::includes::Options::follow(
                    interpolate_context(git_install_dir, home),
                    git_config::file::includes::conditional::Context {
                        git_dir: git_dir.into(),
                        branch_name,
                    },
                )
            } else {
                git_config::file::includes::Options::no_follow()
            },
            ..util::base_options(lossy)
        };

        let config = {
            let home_env = &home_env;
            let xdg_config_home_env = &xdg_config_home_env;
            let git_prefix = &git_prefix;
            let metas = [git_config::source::Kind::System, git_config::source::Kind::Global]
                .iter()
                .flat_map(|kind| kind.sources())
                .filter_map(|source| {
                    match source {
                        git_config::Source::System if !use_system => return None,
                        git_config::Source::Git if !use_git => return None,
                        git_config::Source::User if !use_user => return None,
                        _ => {}
                    }
                    let path = source
                        .storage_location(&mut |name| {
                            match name {
                                git_ if git_.starts_with("GIT_") => Some(git_prefix),
                                "XDG_CONFIG_HOME" => Some(xdg_config_home_env),
                                "HOME" => Some(home_env),
                                _ => None,
                            }
                            .and_then(|perm| std::env::var_os(name).and_then(|val| perm.check(val).ok().flatten()))
                        })
                        .map(|p| p.into_owned());

                    git_config::file::Metadata {
                        path,
                        source: *source,
                        level: 0,
                        trust: git_sec::Trust::Full,
                    }
                    .into()
                });

            let err_on_nonexisting_paths = false;
            let mut globals = git_config::File::from_paths_metadata_buf(
                metas,
                &mut buf,
                err_on_nonexisting_paths,
                git_config::file::init::Options {
                    includes: git_config::file::includes::Options::no_follow(),
                    ..options
                },
            )
            .map_err(|err| match err {
                git_config::file::init::from_paths::Error::Init(err) => Error::from(err),
                git_config::file::init::from_paths::Error::Io(err) => err.into(),
            })?
            .unwrap_or_default();

            globals.append(git_dir_config);
            globals.resolve_includes(options)?;
            if use_env {
                globals.append(git_config::File::from_env(options)?.unwrap_or_default());
            }
            globals
        };

        let excludes_file = match config
            .path_filter("core", None, "excludesFile", &mut filter_config_section)
            .map(|p| p.interpolate(options.includes.interpolate).map(|p| p.into_owned()))
            .transpose()
        {
            Ok(f) => f,
            Err(_err) if lenient_config => None,
            Err(err) => return Err(err.into()),
        };

        let hex_len = match util::parse_core_abbrev(&config, object_hash) {
            Ok(v) => v,
            Err(_err) if lenient_config => None,
            Err(err) => return Err(err),
        };

        use util::config_bool;
        let reflog = util::query_refupdates(&config);
        let ignore_case = config_bool(&config, "core.ignoreCase", false, lenient_config)?;
        let use_multi_pack_index = config_bool(&config, "core.multiPackIndex", true, lenient_config)?;
        let object_kind_hint = config.string("core", None, "disambiguate").and_then(|value| {
            Some(match value.as_ref().as_ref() {
                b"commit" => ObjectKindHint::Commit,
                b"committish" => ObjectKindHint::Committish,
                b"tree" => ObjectKindHint::Tree,
                b"treeish" => ObjectKindHint::Treeish,
                b"blob" => ObjectKindHint::Blob,
                _ => return None,
            })
        });
        Ok(Cache {
            resolved: config.into(),
            use_multi_pack_index,
            object_hash,
            object_kind_hint,
            reflog,
            is_bare,
            ignore_case,
            hex_len,
            filter_config_section,
            excludes_file,
            xdg_config_home_env,
            home_env,
            personas: Default::default(),
            url_rewrite: Default::default(),
            #[cfg(any(feature = "blocking-network-client", feature = "async-network-client-async-std"))]
            url_scheme: Default::default(),
            git_prefix,
        })
    }

    /// Return a path by using the `$XDF_CONFIG_HOME` or `$HOME/.config/…` environment variables locations.
    pub fn xdg_config_path(
        &self,
        resource_file_name: &str,
    ) -> Result<Option<PathBuf>, git_sec::permission::Error<PathBuf, git_sec::Permission>> {
        std::env::var_os("XDG_CONFIG_HOME")
            .map(|path| (path, &self.xdg_config_home_env))
            .or_else(|| std::env::var_os("HOME").map(|path| (path, &self.home_env)))
            .and_then(|(base, permission)| {
                let resource = std::path::PathBuf::from(base).join("git").join(resource_file_name);
                permission.check(resource).transpose()
            })
            .transpose()
    }

    /// Return the home directory if we are allowed to read it and if it is set in the environment.
    ///
    /// We never fail for here even if the permission is set to deny as we `git-config` will fail later
    /// if it actually wants to use the home directory - we don't want to fail prematurely.
    pub fn home_dir(&self) -> Option<PathBuf> {
        std::env::var_os("HOME")
            .map(PathBuf::from)
            .and_then(|path| self.home_env.check(path).ok().flatten())
    }
}
