use std::{borrow::Cow, convert::TryFrom};

pub use error::Error;

use crate::config::cache::util::IgnoreEmptyPath;
use crate::{
    bstr::{ByteSlice, ByteVec},
    config::{
        tree::{credential, Core, Credential, Key},
        Snapshot,
    },
};

mod error {
    use crate::bstr::BString;

    /// The error returned by [`Snapshot::credential_helpers()`][super::Snapshot::credential_helpers()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("Could not parse 'useHttpPath' key in section {section}")]
        InvalidUseHttpPath {
            section: BString,
            source: gix_config::value::Error,
        },
        #[error("core.askpass could not be read")]
        CoreAskpass(#[from] gix_config::path::interpolate::Error),
    }
}

impl Snapshot<'_> {
    /// Returns the configuration for all git-credential helpers from trusted configuration that apply
    /// to the given `url` along with an action preconfigured to invoke the cascade with.
    /// This includes `url` which may be altered to contain a user-name as configured.
    ///
    /// These can be invoked to obtain credentials. Note that the `url` is expected to be the one used
    /// to connect to a remote, and thus should already have passed the url-rewrite engine.
    ///
    /// # Deviation
    ///
    /// - Invalid urls can't be used to obtain credential helpers as they are rejected early when creating a valid `url` here.
    /// - Parsed urls will automatically drop the port if it's the default, i.e. `http://host:80` becomes `http://host` when parsed.
    ///   This affects the prompt provided to the user, so that git will use the verbatim url, whereas we use `http://host`.
    /// - Upper-case scheme and host will be lower-cased automatically when parsing into a url, so prompts differ compared to git.
    /// - A **difference in prompt might affect the matching of getting existing stored credentials**, and it's a question of this being
    ///   a feature or a bug.
    // TODO: when dealing with `http.*.*` configuration, generalize this algorithm as needed and support precedence.
    pub fn credential_helpers(
        &self,
        mut url: gix_url::Url,
    ) -> Result<
        (
            gix_credentials::helper::Cascade,
            gix_credentials::helper::Action,
            gix_prompt::Options<'static>,
        ),
        Error,
    > {
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
                    Some(pattern) => gix_url::parse(pattern).ok().and_then(|mut pattern| {
                        normalize(&mut pattern);
                        let is_http = matches!(pattern.scheme, gix_url::Scheme::Https | gix_url::Scheme::Http);
                        let scheme = &pattern.scheme;
                        let host = pattern.host();
                        let ports = is_http
                            .then(|| (pattern.port_or_default(), url.port_or_default()))
                            .unwrap_or((pattern.port, url.port));
                        let path = (!(is_http && pattern.path_is_root())).then_some(&pattern.path);

                        if !path.map_or(true, |path| path == &url.path) {
                            return None;
                        }
                        if pattern.user().is_some() && pattern.user() != url.user() {
                            return None;
                        }
                        (scheme == &url.scheme && host_matches(host, url.host()) && ports.0 == ports.1).then_some((
                            section,
                            &credential::UrlParameter::HELPER,
                            &credential::UrlParameter::USERNAME,
                            &credential::UrlParameter::USE_HTTP_PATH,
                        ))
                    }),
                    None => Some((
                        section,
                        &Credential::HELPER,
                        &Credential::USERNAME,
                        &Credential::USE_HTTP_PATH,
                    )),
                };
                if let Some((section, helper_key, username_key, use_http_path_key)) = section {
                    for value in section.values(helper_key.name) {
                        if value.trim().is_empty() {
                            programs.clear();
                        } else {
                            programs.push(gix_credentials::Program::from_custom_definition(value.into_owned()));
                        }
                    }
                    if let Some(Some(user)) = (!url_had_user_initially).then(|| {
                        section
                            .value(username_key.name)
                            .filter(|n| !n.trim().is_empty())
                            .and_then(|n| {
                                let n: Vec<_> = Cow::into_owned(n).into();
                                n.into_string().ok()
                            })
                    }) {
                        url.set_user(Some(user));
                    }
                    if let Some(toggle) = section
                        .value(use_http_path_key.name)
                        .map(|val| {
                            gix_config::Boolean::try_from(val)
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

        let allow_git_env = self.repo.options.permissions.env.git_prefix.is_allowed();
        let allow_ssh_env = self.repo.options.permissions.env.ssh_prefix.is_allowed();
        let prompt_options = gix_prompt::Options {
            askpass: self
                .trusted_path(Core::ASKPASS.logical_name().as_str())
                .transpose()
                .ignore_empty()?
                .map(|c| Cow::Owned(c.into_owned())),
            ..Default::default()
        }
        .apply_environment(allow_git_env, allow_ssh_env, allow_git_env);
        Ok((
            gix_credentials::helper::Cascade {
                programs,
                use_http_path,
                // The default ssh implementation uses binaries that do their own auth, so our passwords aren't used.
                query_user_only: url.scheme == gix_url::Scheme::Ssh,
                ..Default::default()
            },
            gix_credentials::helper::Action::get_for_url(url.to_bstring()),
            prompt_options,
        ))
    }
}

fn host_matches(pattern: Option<&str>, host: Option<&str>) -> bool {
    match (pattern, host) {
        (Some(pattern), Some(host)) => {
            let lfields = pattern.split('.');
            let rfields = host.split('.');
            if lfields.clone().count() != rfields.clone().count() {
                return false;
            }
            lfields
                .zip(rfields)
                .all(|(pat, value)| gix_glob::wildmatch(pat.into(), value.into(), gix_glob::wildmatch::Mode::empty()))
        }
        (None, None) => true,
        (Some(_), None) | (None, Some(_)) => false,
    }
}

fn normalize(url: &mut gix_url::Url) {
    if !url.path_is_root() && url.path.ends_with(b"/") {
        url.path.pop();
    }
}
