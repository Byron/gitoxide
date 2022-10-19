use super::{interpolate_context, util, Error, StageOne};
use crate::bstr::BString;
use crate::{config::Cache, repository};

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
        filter_config_section: fn(&git_config::file::Metadata) -> bool,
        git_install_dir: Option<&std::path::Path>,
        home: Option<&std::path::Path>,
        repository::permissions::Environment {
            git_prefix,
            home: home_env,
            xdg_config_home: xdg_config_home_env,
            ssh_prefix: _,
        }: repository::permissions::Environment,
        repository::permissions::Config {
            git_binary: use_installation,
            system: use_system,
            git: use_git,
            user: use_user,
            env: use_env,
            includes: use_includes,
        }: repository::permissions::Config,
        lenient_config: bool,
        config_overrides: &[BString],
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
            let metas = [
                git_config::source::Kind::GitInstallation,
                git_config::source::Kind::System,
                git_config::source::Kind::Global,
            ]
            .iter()
            .flat_map(|kind| kind.sources())
            .filter_map(|source| {
                match source {
                    git_config::Source::GitInstallation if !use_installation => return None,
                    git_config::Source::System if !use_system => return None,
                    git_config::Source::Git if !use_git => return None,
                    git_config::Source::User if !use_user => return None,
                    _ => {}
                }
                source
                    .storage_location(&mut |name| {
                        match name {
                            git_ if git_.starts_with("GIT_") => Some(git_prefix),
                            "XDG_CONFIG_HOME" => Some(xdg_config_home_env),
                            "HOME" => Some(home_env),
                            _ => None,
                        }
                        .and_then(|perm| std::env::var_os(name).and_then(|val| perm.check_opt(val)))
                    })
                    .map(|p| (source, p.into_owned()))
            })
            .map(|(source, path)| git_config::file::Metadata {
                path: Some(path),
                source: *source,
                level: 0,
                trust: git_sec::Trust::Full,
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
            if !config_overrides.is_empty() {
                crate::config::overrides::apply(&mut globals, config_overrides, git_config::Source::Api)?;
            }
            globals
        };

        let hex_len = util::check_lenient(util::parse_core_abbrev(&config, object_hash), lenient_config)?;

        use util::config_bool;
        let reflog = util::query_refupdates(&config, lenient_config)?;
        let ignore_case = config_bool(&config, "core.ignoreCase", false, lenient_config)?;
        let use_multi_pack_index = config_bool(&config, "core.multiPackIndex", true, lenient_config)?;
        let object_kind_hint = util::disambiguate_hint(&config);
        // NOTE: When adding a new initial cache, consider adjusting `reread_values_and_clear_caches()` as well.
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
            xdg_config_home_env,
            home_env,
            lenient_config,
            personas: Default::default(),
            url_rewrite: Default::default(),
            #[cfg(any(feature = "blocking-network-client", feature = "async-network-client"))]
            url_scheme: Default::default(),
            git_prefix,
        })
    }

    /// Call this with new `config` to update values and clear caches. Note that none of the values will be applied if a single
    /// one is invalid.
    /// However, those that are lazily read won't be re-evaluated right away and might thus pass now but fail later.
    ///
    /// Note that we unconditionally re-read all values.
    pub fn reread_values_and_clear_caches_replacing_config(&mut self, config: crate::Config) -> Result<(), Error> {
        let prev = std::mem::replace(&mut self.resolved, config);
        match self.reread_values_and_clear_caches() {
            Err(err) => {
                drop(std::mem::replace(&mut self.resolved, prev));
                Err(err)
            }
            Ok(()) => Ok(()),
        }
    }

    /// Similar to `reread_values_and_clear_caches_replacing_config()`, but works on the existing configuration instead of a passed
    /// in one that it them makes the default.
    pub fn reread_values_and_clear_caches(&mut self) -> Result<(), Error> {
        let config = &self.resolved;
        let hex_len = util::check_lenient(util::parse_core_abbrev(config, self.object_hash), self.lenient_config)?;

        use util::config_bool;
        let ignore_case = config_bool(config, "core.ignoreCase", false, self.lenient_config)?;
        let object_kind_hint = util::disambiguate_hint(config);
        let reflog = util::query_refupdates(config, self.lenient_config)?;

        self.hex_len = hex_len;
        self.ignore_case = ignore_case;
        self.object_kind_hint = object_kind_hint;
        self.reflog = reflog;

        self.personas = Default::default();
        self.url_rewrite = Default::default();
        #[cfg(any(feature = "blocking-network-client", feature = "async-network-client"))]
        {
            self.url_scheme = Default::default();
        }

        Ok(())
    }
}
