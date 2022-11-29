use std::{borrow::Cow, convert::TryInto, path::PathBuf, time::Duration};

use git_lock::acquire::Fail;

use crate::{
    bstr::BStr,
    config::{cache::util::ApplyLeniencyDefault, checkout_options, Cache},
    remote,
    repository::identity,
};

/// Access
impl Cache {
    pub(crate) fn diff_algorithm(&self) -> Result<git_diff::blob::Algorithm, crate::config::diff::algorithm::Error> {
        use crate::config::diff::algorithm::Error;
        self.diff_algorithm
            .get_or_try_init(|| {
                let name = self
                    .resolved
                    .string("diff", None, "algorithm")
                    .unwrap_or_else(|| Cow::Borrowed("myers".into()));
                if name.eq_ignore_ascii_case(b"myers") || name.eq_ignore_ascii_case(b"default") {
                    Ok(git_diff::blob::Algorithm::Myers)
                } else if name.eq_ignore_ascii_case(b"minimal") {
                    Ok(git_diff::blob::Algorithm::MyersMinimal)
                } else if name.eq_ignore_ascii_case(b"histogram") {
                    Ok(git_diff::blob::Algorithm::Histogram)
                } else if name.eq_ignore_ascii_case(b"patience") {
                    if self.lenient_config {
                        Ok(git_diff::blob::Algorithm::Histogram)
                    } else {
                        Err(Error::Unimplemented {
                            name: name.into_owned(),
                        })
                    }
                } else {
                    Err(Error::Unknown {
                        name: name.into_owned(),
                    })
                }
                .with_lenient_default(self.lenient_config)
            })
            .copied()
    }

    /// Returns a user agent for use with servers.
    #[cfg(any(feature = "async-network-client", feature = "blocking-network-client"))]
    pub(crate) fn user_agent_tuple(&self) -> (&'static str, Option<Cow<'static, str>>) {
        let agent = self
            .user_agent
            .get_or_init(|| {
                self.resolved
                    .string_by_key("gitoxide.userAgent")
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| crate::env::agent().into())
            })
            .to_owned();
        ("agent", Some(git_protocol::agent(agent).into()))
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
    pub(crate) fn url_scheme(
        &self,
    ) -> Result<&remote::url::SchemePermission, remote::url::scheme_permission::init::Error> {
        self.url_scheme
            .get_or_try_init(|| remote::url::SchemePermission::from_config(&self.resolved, self.filter_config_section))
    }

    /// Returns (file-timeout, pack-refs timeout)
    pub(crate) fn lock_timeout(
        &self,
    ) -> Result<(git_lock::acquire::Fail, git_lock::acquire::Fail), git_config::value::Error> {
        enum Kind {
            RefFiles,
            RefPackFile,
        }
        let mut out: [git_lock::acquire::Fail; 2] = Default::default();

        for (idx, kind) in [Kind::RefFiles, Kind::RefPackFile].iter().enumerate() {
            let (key, default_ms) = match kind {
                Kind::RefFiles => ("filesRefLockTimeout", 100),
                Kind::RefPackFile => ("packedRefsTimeout", 1000),
            };
            let mk_default = || Fail::AfterDurationWithBackoff(Duration::from_millis(default_ms));
            let mut fnp = self.filter_config_section;

            let lock_mode = match self.resolved.integer_filter("core", None, key, &mut fnp) {
                Some(Ok(val)) if val < 0 => Fail::AfterDurationWithBackoff(Duration::from_secs(u64::MAX)),
                Some(Ok(val)) if val == 0 => Fail::Immediately,
                Some(Ok(val)) => Fail::AfterDurationWithBackoff(Duration::from_millis(
                    val.try_into().expect("i64 can be repsented by u64"),
                )),
                Some(Err(_)) if self.lenient_config => mk_default(),
                Some(Err(err)) => return Err(err),
                None => mk_default(),
            };
            out[idx] = lock_mode;
        }
        Ok((out[0], out[1]))
    }

    /// The path to the user-level excludes file to ignore certain files in the worktree.
    pub(crate) fn excludes_file(&self) -> Option<Result<PathBuf, git_config::path::interpolate::Error>> {
        self.trusted_file_path("core", None, "excludesFile")?
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
    ) -> Option<Result<Cow<'_, std::path::Path>, git_config::path::interpolate::Error>> {
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
        match res {
            Some(Ok(v)) => Ok(Some(v)),
            Some(Err(_err)) if self.lenient_config => Ok(None),
            Some(Err(err)) => Err(err),
            None => Ok(None),
        }
    }

    /// Collect everything needed to checkout files into a worktree.
    /// Note that some of the options being returned will be defaulted so safe settings, the caller might have to override them
    /// depending on the use-case.
    pub(crate) fn checkout_options(
        &self,
        git_dir: &std::path::Path,
    ) -> Result<git_worktree::index::checkout::Options, checkout_options::Error> {
        fn checkout_thread_limit_from_config(
            config: &git_config::File<'static>,
        ) -> Option<Result<usize, checkout_options::Error>> {
            config.integer("checkout", None, "workers").map(|val| match val {
                Ok(v) if v < 0 => Ok(0),
                Ok(v) => Ok(v.try_into().expect("positive i64 can always be usize on 64 bit")),
                Err(err) => Err(checkout_options::Error::Configuration {
                    key: "checkout.workers",
                    source: err,
                }),
            })
        }

        fn boolean(me: &Cache, full_key: &'static str, default: bool) -> Result<bool, checkout_options::Error> {
            let mut tokens = full_key.split('.');
            let section = tokens.next().expect("section");
            let key = tokens.next().expect("key");
            assert!(tokens.next().is_none(), "core.<key>");
            Ok(me
                .apply_leniency(me.resolved.boolean(section, None, key))
                .map_err(|err| checkout_options::Error::Configuration {
                    key: full_key,
                    source: err,
                })?
                .unwrap_or(default))
        }

        fn assemble_attribute_globals(
            me: &Cache,
            _git_dir: &std::path::Path,
        ) -> Result<git_attributes::MatchGroup, checkout_options::Error> {
            let _attributes_file = match me.trusted_file_path("core", None, "attributesFile").transpose()? {
                Some(attributes) => Some(attributes.into_owned()),
                None => me.xdg_config_path("attributes").ok().flatten(),
            };
            // TODO: implement git_attributes::MatchGroup::<git_attributes::Attributes>::from_git_dir(), similar to what's done for `Ignore`.
            Ok(Default::default())
        }

        let thread_limit = self.apply_leniency(checkout_thread_limit_from_config(&self.resolved))?;
        Ok(git_worktree::index::checkout::Options {
            fs: git_worktree::fs::Capabilities {
                precompose_unicode: boolean(self, "core.precomposeUnicode", false)?,
                ignore_case: boolean(self, "core.ignoreCase", false)?,
                executable_bit: boolean(self, "core.fileMode", true)?,
                symlink: boolean(self, "core.symlinks", true)?,
            },
            thread_limit,
            destination_is_initially_empty: false,
            overwrite_existing: false,
            keep_going: false,
            trust_ctime: boolean(self, "core.trustCTime", true)?,
            check_stat: self
                .resolved
                .string("core", None, "checkStat")
                .map_or(true, |v| v.as_ref() != "minimal"),
            attribute_globals: assemble_attribute_globals(self, git_dir)?,
        })
    }
    pub(crate) fn xdg_config_path(
        &self,
        resource_file_name: &str,
    ) -> Result<Option<PathBuf>, git_sec::permission::Error<PathBuf>> {
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
    /// We never fail for here even if the permission is set to deny as we `git-config` will fail later
    /// if it actually wants to use the home directory - we don't want to fail prematurely.
    pub(crate) fn home_dir(&self) -> Option<PathBuf> {
        std::env::var_os("HOME")
            .map(PathBuf::from)
            .and_then(|path| self.home_env.check_opt(path))
    }
}
