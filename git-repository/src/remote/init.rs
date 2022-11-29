use std::convert::TryInto;

use git_refspec::RefSpec;

use crate::{config, remote, Remote, Repository};

mod error {
    use crate::bstr::BString;

    /// The error returned by [`Repository::remote_at(…)`][crate::Repository::remote_at()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        Url(#[from] git_url::parse::Error),
        #[error("The rewritten {kind} url {rewritten_url:?} failed to parse")]
        RewrittenUrlInvalid {
            kind: &'static str,
            rewritten_url: BString,
            source: git_url::parse::Error,
        },
    }
}
pub use error::Error;

use crate::bstr::BString;

/// Initialization
impl<'repo> Remote<'repo> {
    pub(crate) fn from_preparsed_config(
        name_or_url: Option<BString>,
        url: Option<git_url::Url>,
        push_url: Option<git_url::Url>,
        fetch_specs: Vec<RefSpec>,
        push_specs: Vec<RefSpec>,
        should_rewrite_urls: bool,
        repo: &'repo Repository,
    ) -> Result<Self, Error> {
        debug_assert!(
            url.is_some() || push_url.is_some(),
            "BUG: fetch or push url must be set at least"
        );
        let (url_alias, push_url_alias) = should_rewrite_urls
            .then(|| rewrite_urls(&repo.config, url.as_ref(), push_url.as_ref()))
            .unwrap_or(Ok((None, None)))?;
        Ok(Remote {
            name: name_or_url.map(Into::into),
            url,
            url_alias,
            push_url,
            push_url_alias,
            fetch_specs,
            push_specs,
            repo,
        })
    }

    pub(crate) fn from_fetch_url<Url, E>(
        url: Url,
        should_rewrite_urls: bool,
        repo: &'repo Repository,
    ) -> Result<Self, Error>
    where
        Url: TryInto<git_url::Url, Error = E>,
        git_url::parse::Error: From<E>,
    {
        let url = url.try_into().map_err(|err| Error::Url(err.into()))?;
        let (url_alias, _) = should_rewrite_urls
            .then(|| rewrite_urls(&repo.config, Some(&url), None))
            .unwrap_or(Ok((None, None)))?;
        Ok(Remote {
            name: None,
            url: Some(url),
            url_alias,
            push_url: None,
            push_url_alias: None,
            fetch_specs: Vec::new(),
            push_specs: Vec::new(),
            repo,
        })
    }
}

pub(crate) fn rewrite_url(
    config: &config::Cache,
    url: Option<&git_url::Url>,
    direction: remote::Direction,
) -> Result<Option<git_url::Url>, Error> {
    url.and_then(|url| config.url_rewrite().longest(url, direction))
        .map(|url| {
            git_url::parse(url.as_ref()).map_err(|err| Error::RewrittenUrlInvalid {
                kind: match direction {
                    remote::Direction::Fetch => "fetch",
                    remote::Direction::Push => "push",
                },
                source: err,
                rewritten_url: url,
            })
        })
        .transpose()
}

pub(crate) fn rewrite_urls(
    config: &config::Cache,
    url: Option<&git_url::Url>,
    push_url: Option<&git_url::Url>,
) -> Result<(Option<git_url::Url>, Option<git_url::Url>), Error> {
    let url_alias = rewrite_url(config, url, remote::Direction::Fetch)?;
    let push_url_alias = rewrite_url(config, push_url, remote::Direction::Push)?;

    Ok((url_alias, push_url_alias))
}
