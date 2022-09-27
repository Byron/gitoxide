use crate::config::Cache;
use crate::{remote, repository::identity};
use git_lock::acquire::Fail;
use std::convert::TryInto;
use std::time::Duration;

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
}
