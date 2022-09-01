use crate::config::Snapshot;
use std::borrow::Cow;

use std::convert::TryFrom;

mod error {
    use crate::bstr::BString;

    /// The error returned by [Snapshot::credential_helpers()][super::Snapshot::credential_helpers()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("Could not parse 'useHttpPath' key in section {section}")]
        InvalidUseHttpPath {
            section: BString,
            source: git_config::value::Error,
        },
    }
}
use crate::bstr::ByteVec;
pub use error::Error;

impl Snapshot<'_> {
    /// Returns the configuration for all git-credential helpers that apply to the given `url` along with an action
    /// preconfigured to invoke the cascade with. This includes `url` which may be altered to contain a user-name
    /// as configured.
    ///
    /// These can be invoked to obtain credentials. Note that the `url` is expected to be the one used
    /// to connect to a remote, and thus should already have passed the url-rewrite engine.
    pub fn credential_helpers(
        &self,
        mut url: git_url::Url,
    ) -> Result<(git_credentials::helper::Cascade, git_credentials::helper::Action), Error> {
        let mut programs = Vec::new();
        let mut use_http_path = false;
        let url_had_user_initially = url.user().is_some();

        if let Some(credential_sections) = self
            .repo
            .config
            .resolved
            .sections_by_name_and_filter("credential", &mut self.repo.filter_config_section())
        {
            for section in credential_sections {
                match section.header().subsection_name() {
                    Some(_) => {}
                    None => {
                        for value in section.values("helper") {
                            programs.push(git_credentials::Program::from_custom_definition(value.into_owned()));
                        }
                        if let Some(Some(user)) = (!url_had_user_initially).then(|| {
                            section.value("username").and_then(|n| {
                                let n: Vec<_> = Cow::into_owned(n).into();
                                n.into_string().ok()
                            })
                        }) {
                            url.set_user(Some(user));
                        }
                        if let Some(toggle) = section
                            .value("useHttpPath")
                            .map(|val| {
                                git_config::Boolean::try_from(val)
                                    .map_err(|err| Error::InvalidUseHttpPath {
                                        source: err,
                                        section: section.header().to_bstring(),
                                    })
                                    .map(|b| b.0)
                            })
                            .transpose()?
                        {
                            use_http_path = toggle;
                        }
                    }
                }
            }
        }

        Ok((
            git_credentials::helper::Cascade {
                programs,
                use_http_path,
                ..Default::default()
            },
            git_credentials::helper::Action::get_for_url(url.to_bstring()),
        ))
    }
}
