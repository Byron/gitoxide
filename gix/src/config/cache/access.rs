#![allow(clippy::result_large_err)]
use std::{borrow::Cow, path::PathBuf, time::Duration};

use gix_lock::acquire::Fail;

use crate::{
    bstr::BStr,
    config,
    config::{
        cache::util::{ApplyLeniency, ApplyLeniencyDefault},
        checkout_options,
        tree::{Checkout, Core, Key},
        Cache,
    },
    remote,
    repository::identity,
};

/// Access
impl Cache {
    pub(crate) fn diff_algorithm(&self) -> Result<gix_diff::blob::Algorithm, config::diff::algorithm::Error> {
        use crate::config::diff::algorithm::Error;
        self.diff_algorithm
            .get_or_try_init(|| {
                let name = self
                    .resolved
                    .string("diff", None, "algorithm")
                    .unwrap_or_else(|| Cow::Borrowed("myers".into()));
                config::tree::Diff::ALGORITHM
                    .try_into_algorithm(name)
                    .or_else(|err| match err {
                        Error::Unimplemented { .. } if self.lenient_config => Ok(gix_diff::blob::Algorithm::Histogram),
                        err => Err(err),
                    })
                    .with_lenient_default(self.lenient_config)
            })
            .copied()
    }

    /// Returns a user agent for use with servers.
    #[cfg(any(feature = "async-network-client", feature = "blocking-network-client"))]
    pub(crate) fn user_agent_tuple(&self) -> (&'static str, Option<Cow<'static, str>>) {
        use config::tree::Gitoxide;
        let agent = self
            .user_agent
            .get_or_init(|| {
                self.resolved
                    .string_by_key(Gitoxide::USER_AGENT.logical_name().as_str())
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| crate::env::agent().into())
            })
            .to_owned();
        ("agent", Some(gix_protocol::agent(agent).into()))
    }

    pub(crate) fn personas(&self) -> &identity::Personas {
        self.personas
            .get_or_init(|| identity::Personas::from_config_and_env(&self.resolved))
    }

    pub(crate) fn url_rewrite(&self) -> &remote::url::Rewrite {
        self.url_rewrite
            .get_or_init(|| remote::url::Rewrite::from_config(&self.resolved, self.filter_config_section))
    }

    #[cfg(any(feature = "blocking-network-client", feature = "async-network-client"))]
    pub(crate) fn url_scheme(&self) -> Result<&remote::url::SchemePermission, config::protocol::allow::Error> {
        self.url_scheme
            .get_or_try_init(|| remote::url::SchemePermission::from_config(&self.resolved, self.filter_config_section))
    }

    pub(crate) fn diff_renames(
        &self,
    ) -> Result<Option<crate::object::tree::diff::Rewrites>, crate::object::tree::diff::rewrites::Error> {
        self.diff_renames
            .get_or_try_init(|| {
                crate::object::tree::diff::Rewrites::try_from_config(&self.resolved, self.lenient_config)
            })
            .copied()
    }

    /// Returns (file-timeout, pack-refs timeout)
    pub(crate) fn lock_timeout(
        &self,
    ) -> Result<(gix_lock::acquire::Fail, gix_lock::acquire::Fail), config::lock_timeout::Error> {
        let mut out: [gix_lock::acquire::Fail; 2] = Default::default();
        for (idx, (key, default_ms)) in [(&Core::FILES_REF_LOCK_TIMEOUT, 100), (&Core::PACKED_REFS_TIMEOUT, 1000)]
            .into_iter()
            .enumerate()
        {
            out[idx] = self
                .resolved
                .integer_filter("core", None, key.name, &mut self.filter_config_section.clone())
                .map(|res| key.try_into_lock_timeout(res))
                .transpose()
                .with_leniency(self.lenient_config)?
                .unwrap_or_else(|| Fail::AfterDurationWithBackoff(Duration::from_millis(default_ms)));
        }
        Ok((out[0], out[1]))
    }

    /// The path to the user-level excludes file to ignore certain files in the worktree.
    pub(crate) fn excludes_file(&self) -> Option<Result<PathBuf, gix_config::path::interpolate::Error>> {
        self.trusted_file_path("core", None, Core::EXCLUDES_FILE.name)?
            .map(|p| p.into_owned())
            .into()
    }

    /// A helper to obtain a file from trusted configuration at `section_name`, `subsection_name`, and `key`, which is interpolated
    /// if present.
    pub(crate) fn trusted_file_path(
        &self,
        section_name: impl AsRef<str>,
        subsection_name: Option<&BStr>,
        key: impl AsRef<str>,
    ) -> Option<Result<Cow<'_, std::path::Path>, gix_config::path::interpolate::Error>> {
        let path = self.resolved.path_filter(
            section_name,
            subsection_name,
            key,
            &mut self.filter_config_section.clone(),
        )?;

        let install_dir = crate::path::install_dir().ok();
        let home = self.home_dir();
        let ctx = crate::config::cache::interpolate_context(install_dir.as_deref(), home.as_deref());
        Some(path.interpolate(ctx))
    }

    pub(crate) fn apply_leniency<T, E>(&self, res: Option<Result<T, E>>) -> Result<Option<T>, E> {
        res.transpose().with_leniency(self.lenient_config)
    }

    /// Collect everything needed to checkout files into a worktree.
    /// Note that some of the options being returned will be defaulted so safe settings, the caller might have to override them
    /// depending on the use-case.
    pub(crate) fn checkout_options(
        &self,
        git_dir: &std::path::Path,
    ) -> Result<gix_worktree::index::checkout::Options, checkout_options::Error> {
        fn boolean(
            me: &Cache,
            full_key: &str,
            key: &'static config::tree::keys::Boolean,
            default: bool,
        ) -> Result<bool, checkout_options::Error> {
            debug_assert_eq!(
                full_key,
                key.logical_name(),
                "BUG: key name and hardcoded name must match"
            );
            Ok(me
                .apply_leniency(me.resolved.boolean_by_key(full_key).map(|v| key.enrich_error(v)))?
                .unwrap_or(default))
        }

        fn assemble_attribute_globals(
            me: &Cache,
            _git_dir: &std::path::Path,
        ) -> Result<gix_attributes::MatchGroup, checkout_options::Error> {
            let _attributes_file = match me
                .trusted_file_path("core", None, Core::ATTRIBUTES_FILE.name)
                .transpose()?
            {
                Some(attributes) => Some(attributes.into_owned()),
                None => me.xdg_config_path("attributes").ok().flatten(),
            };
            // TODO: implement gix_attributes::MatchGroup::<gix_attributes::Attributes>::from_git_dir(), similar to what's done for `Ignore`.
            Ok(Default::default())
        }

        let thread_limit = self.apply_leniency(
            self.resolved
                .integer_filter_by_key("checkout.workers", &mut self.filter_config_section.clone())
                .map(|value| Checkout::WORKERS.try_from_workers(value)),
        )?;
        Ok(gix_worktree::index::checkout::Options {
            fs: gix_worktree::fs::Capabilities {
                precompose_unicode: boolean(self, "core.precomposeUnicode", &Core::PRECOMPOSE_UNICODE, false)?,
                ignore_case: boolean(self, "core.ignoreCase", &Core::IGNORE_CASE, false)?,
                executable_bit: boolean(self, "core.fileMode", &Core::FILE_MODE, true)?,
                symlink: boolean(self, "core.symlinks", &Core::SYMLINKS, true)?,
            },
            thread_limit,
            destination_is_initially_empty: false,
            overwrite_existing: false,
            keep_going: false,
            trust_ctime: boolean(self, "core.trustCTime", &Core::TRUST_C_TIME, true)?,
            check_stat: self
                .apply_leniency(
                    self.resolved
                        .string("core", None, "checkStat")
                        .map(|v| Core::CHECK_STAT.try_into_checkstat(v)),
                )?
                .unwrap_or(true),
            attribute_globals: assemble_attribute_globals(self, git_dir)?,
        })
    }
    pub(crate) fn xdg_config_path(
        &self,
        resource_file_name: &str,
    ) -> Result<Option<PathBuf>, gix_sec::permission::Error<PathBuf>> {
        std::env::var_os("XDG_CONFIG_HOME")
            .map(|path| (PathBuf::from(path), &self.xdg_config_home_env))
            .or_else(|| {
                std::env::var_os("HOME").map(|path| {
                    (
                        {
                            let mut p = PathBuf::from(path);
                            p.push(".config");
                            p
                        },
                        &self.home_env,
                    )
                })
            })
            .and_then(|(base, permission)| {
                let resource = base.join("git").join(resource_file_name);
                permission.check(resource).transpose()
            })
            .transpose()
    }

    /// Return the home directory if we are allowed to read it and if it is set in the environment.
    ///
    /// We never fail for here even if the permission is set to deny as we `gix-config` will fail later
    /// if it actually wants to use the home directory - we don't want to fail prematurely.
    pub(crate) fn home_dir(&self) -> Option<PathBuf> {
        std::env::var_os("HOME")
            .map(PathBuf::from)
            .and_then(|path| self.home_env.check_opt(path))
    }
}
