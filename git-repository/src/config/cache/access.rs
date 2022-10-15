use std::{convert::TryInto, path::PathBuf, time::Duration};

use git_lock::acquire::Fail;

use crate::config::checkout_options;
use crate::{config::Cache, remote, repository::identity};

/// Access
impl Cache {
    pub(crate) fn personas(&self) -> &identity::Personas {
        self.personas
            .get_or_init(|| identity::Personas::from_config_and_env(&self.resolved, self.git_prefix))
    }

    pub(crate) fn url_rewrite(&self) -> &remote::url::Rewrite {
        self.url_rewrite
            .get_or_init(|| remote::url::Rewrite::from_config(&self.resolved, self.filter_config_section))
    }

    #[cfg(any(feature = "blocking-network-client", feature = "async-network-client"))]
    pub(crate) fn url_scheme(
        &self,
    ) -> Result<&remote::url::SchemePermission, remote::url::scheme_permission::init::Error> {
        self.url_scheme.get_or_try_init(|| {
            remote::url::SchemePermission::from_config(&self.resolved, self.git_prefix, self.filter_config_section)
        })
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
    pub(crate) fn excludes_file(&self) -> Result<Option<PathBuf>, git_config::path::interpolate::Error> {
        let home = self.home_dir();
        let install_dir = crate::path::install_dir().ok();
        let ctx = crate::config::cache::interpolate_context(install_dir.as_deref(), home.as_deref());
        self.apply_leniency(
            self.resolved
                .path_filter("core", None, "excludesFile", &mut self.filter_config_section.clone())
                .map(|p| p.interpolate(ctx).map(|p| p.into_owned()))
                .transpose(),
        )
    }

    pub(crate) fn apply_leniency<T, E>(&self, res: Result<Option<T>, E>) -> Result<Option<T>, E> {
        match res {
            Ok(v) => Ok(v),
            Err(_err) if self.lenient_config => Ok(None),
            Err(err) => Err(err),
        }
    }

    /// Collect everything needed to checkout files into a worktree.
    /// Note that some of the options being returned will be defaulted so safe settings, the caller might have to override them
    /// depending on the use-case.
    pub(crate) fn checkout_options(&self) -> Result<git_worktree::index::checkout::Options, checkout_options::Error> {
        fn checkout_thread_limit_from_config(
            config: &git_config::File<'static>,
        ) -> Result<Option<usize>, checkout_options::Error> {
            config
                .integer("checkout", None, "workers")
                .map(|val| match val {
                    Ok(v) if v < 0 => Ok(0),
                    Ok(v) => Ok(v.try_into().expect("positive i64 can always be usize on 64 bit")),
                    Err(err) => Err(checkout_options::Error::Configuration {
                        key: "checkout.workers",
                        source: err,
                    }),
                })
                .transpose()
        }

        let thread_limit = self.apply_leniency(checkout_thread_limit_from_config(&self.resolved))?;
        Ok(git_worktree::index::checkout::Options {
            fs: Default::default(),
            thread_limit,
            destination_is_initially_empty: false,
            overwrite_existing: false,
            keep_going: false,
            trust_ctime: true,
            check_stat: true,
            attribute_globals: Default::default(),
        })
    }

    /// Return a path by using the `$XDF_CONFIG_HOME` or `$HOME/.config/…` environment variables locations.
    pub fn xdg_config_path(
        &self,
        resource_file_name: &str,
    ) -> Result<Option<PathBuf>, git_sec::permission::Error<PathBuf>> {
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
            .and_then(|path| self.home_env.check_opt(path))
    }
}
