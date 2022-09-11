use super::{Cache, Error};
use crate::{remote, repository::identity};

mod incubate;
pub(crate) use incubate::StageOne;

mod init;

impl std::fmt::Debug for Cache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cache").finish_non_exhaustive()
    }
}

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

    #[cfg(any(feature = "blocking-network-client", feature = "async-network-client-async-std"))]
    pub(crate) fn url_scheme(
        &self,
    ) -> Result<&remote::url::SchemePermission, remote::url::scheme_permission::init::Error> {
        self.url_scheme.get_or_try_init(|| {
            remote::url::SchemePermission::from_config(&self.resolved, self.git_prefix, self.filter_config_section)
        })
    }
}

mod util;
pub(crate) use util::interpolate_context;
