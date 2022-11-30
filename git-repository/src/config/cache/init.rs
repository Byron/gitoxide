use std::borrow::Cow;

use git_config::File;
use git_sec::Permission;

use super::{interpolate_context, util, Error, StageOne};
use crate::{
    bstr::BString,
    config::{cache::util::ApplyLeniency, Cache},
    repository,
};

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
            http_transport,
            identity,
            objects,
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
        api_config_overrides: &[BString],
        cli_config_overrides: &[BString],
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
            if !cli_config_overrides.is_empty() {
                crate::config::overrides::append(&mut globals, cli_config_overrides, git_config::Source::Cli, |_| None)
                    .map_err(|err| Error::ConfigOverrides {
                        err,
                        source: git_config::Source::Cli,
                    })?;
            }
            if !api_config_overrides.is_empty() {
                crate::config::overrides::append(&mut globals, api_config_overrides, git_config::Source::Api, |_| None)
                    .map_err(|err| Error::ConfigOverrides {
                        err,
                        source: git_config::Source::Api,
                    })?;
            }
            apply_environment_overrides(&mut globals, *git_prefix, http_transport, identity, objects)?;
            globals
        };

        let hex_len = util::parse_core_abbrev(&config, object_hash).with_leniency(lenient_config)?;

        use util::config_bool;
        let reflog = util::query_refupdates(&config, lenient_config)?;
        let ignore_case = config_bool(&config, "core.ignoreCase", false, lenient_config)?;
        let use_multi_pack_index = config_bool(&config, "core.multiPackIndex", true, lenient_config)?;
        let object_kind_hint = util::disambiguate_hint(&config);
        let (pack_cache_bytes, object_cache_bytes) =
            util::parse_object_caches(&config, lenient_config, filter_config_section)?;
        // NOTE: When adding a new initial cache, consider adjusting `reread_values_and_clear_caches()` as well.
        Ok(Cache {
            resolved: config.into(),
            use_multi_pack_index,
            object_hash,
            object_kind_hint,
            pack_cache_bytes,
            object_cache_bytes,
            reflog,
            is_bare,
            ignore_case,
            hex_len,
            filter_config_section,
            xdg_config_home_env,
            home_env,
            lenient_config,
            user_agent: Default::default(),
            personas: Default::default(),
            url_rewrite: Default::default(),
            #[cfg(any(feature = "blocking-network-client", feature = "async-network-client"))]
            url_scheme: Default::default(),
            diff_algorithm: Default::default(),
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
        let hex_len = util::parse_core_abbrev(config, self.object_hash).with_leniency(self.lenient_config)?;

        use util::config_bool;
        let ignore_case = config_bool(config, "core.ignoreCase", false, self.lenient_config)?;
        let object_kind_hint = util::disambiguate_hint(config);
        let reflog = util::query_refupdates(config, self.lenient_config)?;

        self.hex_len = hex_len;
        self.ignore_case = ignore_case;
        self.object_kind_hint = object_kind_hint;
        self.reflog = reflog;

        self.user_agent = Default::default();
        self.personas = Default::default();
        self.url_rewrite = Default::default();
        self.diff_algorithm = Default::default();
        (self.pack_cache_bytes, self.object_cache_bytes) =
            util::parse_object_caches(config, self.lenient_config, self.filter_config_section)?;
        #[cfg(any(feature = "blocking-network-client", feature = "async-network-client"))]
        {
            self.url_scheme = Default::default();
        }

        Ok(())
    }
}

impl crate::Repository {
    /// Causes our configuration to re-read cached values which will also be applied to the repository in-memory state if applicable.
    ///
    /// Similar to `reread_values_and_clear_caches_replacing_config()`, but works on the existing instance instead of a passed
    /// in one that it them makes the default.
    #[cfg(feature = "blocking-network-client")]
    pub(crate) fn reread_values_and_clear_caches(&mut self) -> Result<(), Error> {
        self.config.reread_values_and_clear_caches()?;
        self.apply_changed_values();
        Ok(())
    }

    /// Replace our own configuration with `config` and re-read all cached values, and apply them to select in-memory instances.
    pub(crate) fn reread_values_and_clear_caches_replacing_config(
        &mut self,
        config: crate::Config,
    ) -> Result<(), Error> {
        self.config.reread_values_and_clear_caches_replacing_config(config)?;
        self.apply_changed_values();
        Ok(())
    }

    fn apply_changed_values(&mut self) {
        self.refs.write_reflog = util::reflog_or_default(self.config.reflog, self.work_dir().is_some());
    }
}

fn apply_environment_overrides(
    config: &mut File<'static>,
    git_prefix: Permission,
    http_transport: Permission,
    identity: Permission,
    objects: Permission,
) -> Result<(), Error> {
    fn var_as_bstring(var: &str, perm: Permission) -> Option<BString> {
        perm.check_opt(var)
            .and_then(std::env::var_os)
            .and_then(|val| git_path::os_string_into_bstring(val).ok())
    }

    let mut env_override = git_config::File::new(git_config::file::Metadata::from(git_config::Source::EnvOverride));
    {
        let mut section = env_override
            .new_section("http", None)
            .expect("statically known valid section name");
        for (var, key) in [
            ("GIT_HTTP_LOW_SPEED_LIMIT", "lowSpeedLimit"),
            ("GIT_HTTP_LOW_SPEED_TIME", "lowSpeedTime"),
            ("GIT_HTTP_USER_AGENT", "userAgent"),
            ("GIT_HTTP_PROXY_AUTHMETHOD", "proxyAuthMethod"),
            ("all_proxy", "all-proxy-lower"),
            ("ALL_PROXY", "all-proxy"),
        ] {
            if let Some(value) = var_as_bstring(var, http_transport) {
                section.push_with_comment(
                    key.try_into().expect("statically known to be valid"),
                    Some(value.as_ref()),
                    format!("from {var}").as_str(),
                );
            }
        }
        if section.num_values() == 0 {
            let id = section.id();
            env_override.remove_section_by_id(id);
        }
    }

    {
        let mut section = env_override
            .new_section("gitoxide", Some(Cow::Borrowed("https".into())))
            .expect("statically known valid section name");

        for (var, key) in [("HTTPS_PROXY", "proxy"), ("https_proxy", "proxy")] {
            if let Some(value) = var_as_bstring(var, http_transport) {
                section.push_with_comment(
                    key.try_into().expect("statically known to be valid"),
                    Some(value.as_ref()),
                    format!("from {var}").as_str(),
                );
            }
        }

        if section.num_values() == 0 {
            let id = section.id();
            env_override.remove_section_by_id(id);
        }
    }

    {
        let mut section = env_override
            .new_section("gitoxide", Some(Cow::Borrowed("committer".into())))
            .expect("statically known valid section name");

        for (var, key) in [
            ("GIT_COMMITTER_NAME", "nameFallback"),
            ("GIT_COMMITTER_EMAIL", "emailFallback"),
        ] {
            if let Some(value) = var_as_bstring(var, identity) {
                section.push_with_comment(
                    key.try_into().expect("statically known to be valid"),
                    Some(value.as_ref()),
                    format!("from {var}").as_str(),
                );
            }
        }

        if section.num_values() == 0 {
            let id = section.id();
            env_override.remove_section_by_id(id);
        }
    }

    {
        let mut section = env_override
            .new_section("gitoxide", Some(Cow::Borrowed("author".into())))
            .expect("statically known valid section name");

        for (var, key) in [
            ("GIT_AUTHOR_NAME", "nameFallback"),
            ("GIT_AUTHOR_EMAIL", "emailFallback"),
        ] {
            if let Some(value) = var_as_bstring(var, identity) {
                section.push_with_comment(
                    key.try_into().expect("statically known to be valid"),
                    Some(value.as_ref()),
                    format!("from {var}").as_str(),
                );
            }
        }

        if section.num_values() == 0 {
            let id = section.id();
            env_override.remove_section_by_id(id);
        }
    }

    {
        let mut section = env_override
            .new_section("gitoxide", Some(Cow::Borrowed("commit".into())))
            .expect("statically known valid section name");

        for (var, key) in [
            ("GIT_COMMITTER_DATE", "committerDate"),
            ("GIT_AUTHOR_DATE", "authorDate"),
        ] {
            if let Some(value) = var_as_bstring(var, git_prefix) {
                section.push_with_comment(
                    key.try_into().expect("statically known to be valid"),
                    Some(value.as_ref()),
                    format!("from {var}").as_str(),
                );
            }
        }

        if section.num_values() == 0 {
            let id = section.id();
            env_override.remove_section_by_id(id);
        }
    }

    {
        let mut section = env_override
            .new_section("gitoxide", Some(Cow::Borrowed("allow".into())))
            .expect("statically known valid section name");

        for (var, key) in [("GIT_PROTOCOL_FROM_USER", "protocolFromUser")] {
            if let Some(value) = var_as_bstring(var, http_transport) {
                section.push_with_comment(
                    key.try_into().expect("statically known to be valid"),
                    Some(value.as_ref()),
                    format!("from {var}").as_str(),
                );
            }
        }

        if section.num_values() == 0 {
            let id = section.id();
            env_override.remove_section_by_id(id);
        }
    }

    {
        let mut section = env_override
            .new_section("gitoxide", Some(Cow::Borrowed("user".into())))
            .expect("statically known valid section name");

        for (var, key) in [("EMAIL", "emailFallback")] {
            if let Some(value) = var_as_bstring(var, identity) {
                section.push_with_comment(
                    key.try_into().expect("statically known to be valid"),
                    Some(value.as_ref()),
                    format!("from {var}").as_str(),
                );
            }
        }

        if section.num_values() == 0 {
            let id = section.id();
            env_override.remove_section_by_id(id);
        }
    }

    {
        let mut section = env_override
            .new_section("gitoxide", Some(Cow::Borrowed("objects".into())))
            .expect("statically known valid section name");

        for (var, key, permission) in [
            ("GIT_NO_REPLACE_OBJECTS", "noReplace", objects),
            ("GIT_REPLACE_REF_BASE", "replaceRefBase", objects),
            ("GITOXIDE_OBJECT_CACHE_MEMORY", "cacheLimit", objects),
        ] {
            if let Some(value) = var_as_bstring(var, permission) {
                section.push_with_comment(
                    key.try_into().expect("statically known to be valid"),
                    Some(value.as_ref()),
                    format!("from {var}").as_str(),
                );
            }
        }

        if section.num_values() == 0 {
            let id = section.id();
            env_override.remove_section_by_id(id);
        }
    }

    {
        let mut section = env_override
            .new_section("core", None)
            .expect("statically known valid section name");

        for (var, key) in [("GITOXIDE_PACK_CACHE_MEMORY", "deltaBaseCacheLimit")] {
            if let Some(value) = var_as_bstring(var, objects) {
                section.push_with_comment(
                    key.try_into().expect("statically known to be valid"),
                    Some(value.as_ref()),
                    format!("from {var}").as_str(),
                );
            }
        }

        if section.num_values() == 0 {
            let id = section.id();
            env_override.remove_section_by_id(id);
        }
    }

    {
        let mut section = env_override
            .new_section("gitoxide", Some(Cow::Borrowed("http".into())))
            .expect("statically known valid section name");

        for (var, key) in [
            ("ALL_PROXY", "allProxy"),
            ("all_proxy", "allProxy"),
            ("NO_PROXY", "noProxy"),
            ("no_proxy", "noProxy"),
            ("http_proxy", "proxy"),
            ("GIT_CURL_VERBOSE", "verbose"),
        ] {
            if let Some(value) = var_as_bstring(var, http_transport) {
                section.push_with_comment(
                    key.try_into().expect("statically known to be valid"),
                    Some(value.as_ref()),
                    format!("from {var}").as_str(),
                );
            }
        }

        if section.num_values() == 0 {
            let id = section.id();
            env_override.remove_section_by_id(id);
        }
    }

    if !env_override.is_void() {
        config.append(env_override);
    }
    Ok(())
}
