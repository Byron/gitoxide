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
use crate::bstr::{ByteSlice, ByteVec};
pub use error::Error;
use git_url::Url;

impl Snapshot<'_> {
    /// Returns the configuration for all git-credential helpers that apply to the given `url` along with an action
    /// preconfigured to invoke the cascade with. This includes `url` which may be altered to contain a user-name
    /// as configured.
    ///
    /// These can be invoked to obtain credentials. Note that the `url` is expected to be the one used
    /// to connect to a remote, and thus should already have passed the url-rewrite engine.
    ///
    /// # Deviation
    ///
    /// - Invalid urls can't be used to obtain credential helpers as they are rejected early when creating a valid `url` here.
    /// - Parsed urls will automatically drop the port if it's the default, i.e. `http://host:80` becomes `http://host` when parsed.
    ///   This affects the prompt provided to the user, so that git will use the verbatim url, whereas we use `http://host`.
    pub fn credential_helpers(
        &self,
        mut url: git_url::Url,
    ) -> Result<(git_credentials::helper::Cascade, git_credentials::helper::Action), Error> {
        let mut programs = Vec::new();
        let mut use_http_path = false;
        let url_had_user_initially = url.user().is_some();
        normalize(&mut url);

        if let Some(credential_sections) = self
            .repo
            .config
            .resolved
            .sections_by_name_and_filter("credential", &mut self.repo.filter_config_section())
        {
            for section in credential_sections {
                let section = match section.header().subsection_name() {
                    Some(pattern) => git_url::parse(pattern).ok().and_then(|mut pattern| {
                        normalize(&mut pattern);
                        let matches = if matches!(pattern.scheme, git_url::Scheme::Https | git_url::Scheme::Http)
                            && pattern.path_is_root()
                        {
                            pattern.scheme == url.scheme
                                && pattern.host() == url.host()
                                && pattern.port_or_default() == url.port_or_default()
                        } else {
                            pattern == url
                        };
                        matches.then(|| section)
                    }),
                    None => Some(section),
                };
                if let Some(section) = section {
                    for value in section.values("helper") {
                        if value.trim().is_empty() {
                            programs.clear();
                        } else {
                            programs.push(git_credentials::Program::from_custom_definition(value.into_owned()));
                        }
                    }
                    if let Some(Some(user)) = (!url_had_user_initially).then(|| {
                        section
                            .value("username")
                            .filter(|n| !n.trim().is_empty())
                            .and_then(|n| {
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

fn normalize(url: &mut Url) {
    if !url.path_is_root() && url.path.ends_with(b"/") {
        url.path.pop();
    }
}
