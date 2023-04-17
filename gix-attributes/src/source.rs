use std::{borrow::Cow, ffi::OsString, path::Path};

use crate::Source;

impl Source {
    /// Produce a storage location for the this source while potentially querying environment variables using `env_var(<name>)`,
    /// or `None` if the storage location could not be obtained.
    ///
    /// Note that local sources are returned as relative paths to be joined with the base in a separate step.
    pub fn storage_location(self, env_var: &mut dyn FnMut(&str) -> Option<OsString>) -> Option<Cow<'static, Path>> {
        use Source::*;
        Some(match self {
            GitInstallation => gix_path::env::installation_config_prefix()?
                .join("gitattributes")
                .into(),
            System => {
                if env_var("GIT_ATTR_NOSYSTEM").is_some() {
                    return None;
                } else {
                    gix_path::env::system_prefix()?.join("etc/gitattributes").into()
                }
            }
            Git => return gix_path::env::xdg_config("attributes", env_var).map(Cow::Owned),
            Local => Cow::Borrowed(Path::new("info/attributes")),
        })
    }
}
