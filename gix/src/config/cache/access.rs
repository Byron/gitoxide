#![allow(clippy::result_large_err)]
use std::{borrow::Cow, path::PathBuf, time::Duration};

use gix_lock::acquire::Fail;

use crate::{
    bstr::BStr,
    config,
    config::{
        boolean,
        cache::util::{ApplyLeniency, ApplyLeniencyDefaultValue},
        tree::{Core, Key},
        Cache,
    },
    remote,
    repository::identity,
};

/// Access
impl Cache {
    #[cfg(feature = "blob-diff")]
    pub(crate) fn diff_algorithm(&self) -> Result<gix_diff::blob::Algorithm, config::diff::algorithm::Error> {
        use crate::config::cache::util::ApplyLeniencyDefault;
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
                    .map_or_else(|| crate::env::agent().into(), |s| s.to_string())
            })
            .to_owned();
        ("agent", Some(gix_protocol::agent(agent).into()))
    }

    /// Return `true` if packet-tracing is enabled. Lenient and defaults to `false`.
    #[cfg(any(feature = "async-network-client", feature = "blocking-network-client"))]
    pub(crate) fn trace_packet(&self) -> bool {
        use crate::config::tree::Section;
        use config::tree::Gitoxide;
        self.resolved
            .boolean(Gitoxide.name(), None, Gitoxide::TRACE_PACKET.name())
            .and_then(Result::ok)
            .unwrap_or_default()
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

    pub(crate) fn may_use_commit_graph(&self) -> Result<bool, config::boolean::Error> {
        const DEFAULT: bool = true;
        self.resolved
            .boolean_by_key("core.commitGraph")
            .map_or(Ok(DEFAULT), |res| {
                Core::COMMIT_GRAPH
                    .enrich_error(res)
                    .with_lenient_default_value(self.lenient_config, DEFAULT)
            })
    }

    #[cfg(feature = "blob-diff")]
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
    #[cfg(feature = "excludes")]
    pub(crate) fn excludes_file(&self) -> Option<Result<PathBuf, gix_config::path::interpolate::Error>> {
        self.trusted_file_path("core", None, Core::EXCLUDES_FILE.name)?
            .map(std::borrow::Cow::into_owned)
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
        let ctx = config::cache::interpolate_context(install_dir.as_deref(), home.as_deref());
        Some(path.interpolate(ctx))
    }

    pub(crate) fn apply_leniency<T, E>(&self, res: Option<Result<T, E>>) -> Result<Option<T>, E> {
        res.transpose().with_leniency(self.lenient_config)
    }

    pub(crate) fn fs_capabilities(&self) -> Result<gix_fs::Capabilities, boolean::Error> {
        Ok(gix_fs::Capabilities {
            precompose_unicode: boolean(self, "core.precomposeUnicode", &Core::PRECOMPOSE_UNICODE, false)?,
            ignore_case: boolean(self, "core.ignoreCase", &Core::IGNORE_CASE, false)?,
            executable_bit: boolean(self, "core.fileMode", &Core::FILE_MODE, true)?,
            symlink: boolean(self, "core.symlinks", &Core::SYMLINKS, true)?,
        })
    }

    #[cfg(feature = "index")]
    pub(crate) fn stat_options(&self) -> Result<gix_index::entry::stat::Options, config::stat_options::Error> {
        use crate::config::tree::gitoxide;
        Ok(gix_index::entry::stat::Options {
            trust_ctime: boolean(self, "core.trustCTime", &Core::TRUST_C_TIME, true)?,
            use_nsec: boolean(self, "gitoxide.core.useNsec", &gitoxide::Core::USE_NSEC, false)?,
            use_stdev: boolean(self, "gitoxide.core.useStdev", &gitoxide::Core::USE_STDEV, false)?,
            check_stat: self
                .apply_leniency(
                    self.resolved
                        .string("core", None, "checkStat")
                        .map(|v| Core::CHECK_STAT.try_into_checkstat(v)),
                )?
                .unwrap_or(true),
        })
    }

    /// Collect everything needed to checkout files into a worktree.
    /// Note that some of the options being returned will be defaulted so safe settings, the caller might have to override them
    /// depending on the use-case.
    #[cfg(feature = "worktree-mutation")]
    pub(crate) fn checkout_options(
        &self,
        repo: &crate::Repository,
        attributes_source: gix_worktree::stack::state::attributes::Source,
    ) -> Result<gix_worktree_state::checkout::Options, config::checkout_options::Error> {
        use crate::config::tree::gitoxide;
        let git_dir = repo.git_dir();
        let thread_limit = self.apply_leniency(
            self.resolved
                .integer_filter_by_key("checkout.workers", &mut self.filter_config_section.clone())
                .map(|value| crate::config::tree::Checkout::WORKERS.try_from_workers(value)),
        )?;
        let capabilities = self.fs_capabilities()?;
        let filters = {
            let collection = Default::default();
            let mut filters = gix_filter::Pipeline::new(&collection, crate::filter::Pipeline::options(repo)?);
            if let Ok(mut head) = repo.head() {
                let ctx = filters.driver_context_mut();
                ctx.ref_name = head.referent_name().map(|name| name.as_bstr().to_owned());
                ctx.treeish = head.peel_to_commit_in_place().ok().map(|commit| commit.id);
            }
            filters
        };
        let filter_process_delay = if boolean(
            self,
            "gitoxide.core.filterProcessDelay",
            &gitoxide::Core::FILTER_PROCESS_DELAY,
            true,
        )? {
            gix_filter::driver::apply::Delay::Allow
        } else {
            gix_filter::driver::apply::Delay::Forbid
        };
        Ok(gix_worktree_state::checkout::Options {
            filter_process_delay,
            filters,
            attributes: self
                .assemble_attribute_globals(git_dir, attributes_source, self.attributes)?
                .0,
            fs: capabilities,
            thread_limit,
            destination_is_initially_empty: false,
            overwrite_existing: false,
            keep_going: false,
            stat_options: self.stat_options().map_err(|err| match err {
                config::stat_options::Error::ConfigCheckStat(err) => {
                    config::checkout_options::Error::ConfigCheckStat(err)
                }
                config::stat_options::Error::ConfigBoolean(err) => config::checkout_options::Error::ConfigBoolean(err),
            })?,
        })
    }

    #[cfg(feature = "excludes")]
    pub(crate) fn assemble_exclude_globals(
        &self,
        git_dir: &std::path::Path,
        overrides: Option<gix_ignore::Search>,
        source: gix_worktree::stack::state::ignore::Source,
        buf: &mut Vec<u8>,
    ) -> Result<gix_worktree::stack::state::Ignore, config::exclude_stack::Error> {
        let excludes_file = match self.excludes_file().transpose()? {
            Some(user_path) => Some(user_path),
            None => self.xdg_config_path("ignore")?,
        };
        Ok(gix_worktree::stack::state::Ignore::new(
            overrides.unwrap_or_default(),
            gix_ignore::Search::from_git_dir(git_dir, excludes_file, buf)?,
            None,
            source,
        ))
    }
    // TODO: at least one test, maybe related to core.attributesFile configuration.
    #[cfg(feature = "attributes")]
    pub(crate) fn assemble_attribute_globals(
        &self,
        git_dir: &std::path::Path,
        source: gix_worktree::stack::state::attributes::Source,
        attributes: crate::open::permissions::Attributes,
    ) -> Result<(gix_worktree::stack::state::Attributes, Vec<u8>), config::attribute_stack::Error> {
        use gix_attributes::Source;
        let configured_or_user_attributes = match self
            .trusted_file_path("core", None, Core::ATTRIBUTES_FILE.name)
            .transpose()?
        {
            Some(attributes) => Some(attributes),
            None => {
                if attributes.git {
                    self.xdg_config_path("attributes").ok().flatten().map(Cow::Owned)
                } else {
                    None
                }
            }
        };
        let attribute_files = [gix_attributes::Source::GitInstallation, gix_attributes::Source::System]
            .into_iter()
            .filter(|source| match source {
                Source::GitInstallation => attributes.git_binary,
                Source::System => attributes.system,
                Source::Git | Source::Local => unreachable!("we don't offer turning this off right now"),
            })
            .filter_map(|source| source.storage_location(&mut Self::make_source_env(self.environment)))
            .chain(configured_or_user_attributes);
        let info_attributes_path = git_dir.join("info").join("attributes");
        let mut buf = Vec::new();
        let mut collection = gix_attributes::search::MetadataCollection::default();
        let state = gix_worktree::stack::state::Attributes::new(
            gix_attributes::Search::new_globals(attribute_files, &mut buf, &mut collection)?,
            Some(info_attributes_path),
            source,
            collection,
        );
        Ok((state, buf))
    }

    #[cfg(feature = "attributes")]
    pub(crate) fn pathspec_defaults(
        &self,
    ) -> Result<gix_pathspec::Defaults, gix_pathspec::defaults::from_environment::Error> {
        use crate::config::tree::gitoxide;
        let res = gix_pathspec::Defaults::from_environment(&mut |name| {
            let key = [
                &gitoxide::Pathspec::ICASE,
                &gitoxide::Pathspec::GLOB,
                &gitoxide::Pathspec::NOGLOB,
                &gitoxide::Pathspec::LITERAL,
            ]
            .iter()
            .find(|key| key.environment_override().expect("set") == name)
            .expect("we must know all possible input variable names");

            let val = self
                .resolved
                .string("gitoxide", Some("pathspec".into()), key.name())
                .map(gix_path::from_bstr)?;
            Some(val.into_owned().into())
        });
        if res.is_err() && self.lenient_config {
            Ok(gix_pathspec::Defaults::default())
        } else {
            res
        }
    }

    #[cfg(any(feature = "attributes", feature = "excludes"))]
    pub(crate) fn xdg_config_path(
        &self,
        resource_file_name: &str,
    ) -> Result<Option<PathBuf>, gix_sec::permission::Error<PathBuf>> {
        std::env::var_os("XDG_CONFIG_HOME")
            .map(|path| (PathBuf::from(path), &self.environment.xdg_config_home))
            .or_else(|| {
                gix_path::env::home_dir().map(|mut p| {
                    (
                        {
                            p.push(".config");
                            p
                        },
                        &self.environment.home,
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
        gix_path::env::home_dir().and_then(|path| self.environment.home.check_opt(path))
    }
}

fn boolean(
    me: &Cache,
    full_key: &str,
    key: &'static config::tree::keys::Boolean,
    default: bool,
) -> Result<bool, boolean::Error> {
    debug_assert_eq!(
        full_key,
        key.logical_name(),
        "BUG: key name and hardcoded name must match"
    );
    Ok(me
        .apply_leniency(me.resolved.boolean_by_key(full_key).map(|v| key.enrich_error(v)))?
        .unwrap_or(default))
}
