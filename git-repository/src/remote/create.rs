use crate::{config, remote, Remote, Repository};
use git_refspec::RefSpec;
use std::convert::TryInto;

/// Initialization
impl<'repo> Remote<'repo> {
    pub(crate) fn from_preparsed_config(
        name: Option<String>,
        url: Option<git_url::Url>,
        push_url: Option<git_url::Url>,
        fetch_specs: Vec<RefSpec>,
        push_specs: Vec<RefSpec>,
        repo: &'repo Repository,
    ) -> Result<Self, remote::init::Error> {
        debug_assert!(
            url.is_some() || push_url.is_some(),
            "BUG: fetch or push url must be set at least"
        );
        let (url_alias, push_url_alias) = rewrite_urls(&repo.config, url.as_ref(), push_url.as_ref())?;
        Ok(Remote {
            name: name.to_owned().into(),
            url,
            url_alias,
            push_url,
            push_url_alias,
            fetch_specs,
            push_specs,
            apply_url_aliases: true,
            repo,
        })
    }

    pub(crate) fn from_fetch_url<Url, E>(url: Url, repo: &'repo Repository) -> Result<Self, remote::init::Error>
    where
        Url: TryInto<git_url::Url, Error = E>,
        git_url::parse::Error: From<E>,
    {
        let url = url.try_into().map_err(|err| remote::init::Error::Url(err.into()))?;
        let (url_alias, _) = rewrite_urls(&repo.config, Some(&url), None)?;
        Ok(Remote {
            name: None,
            url: Some(url),
            url_alias,
            push_url: None,
            push_url_alias: None,
            fetch_specs: Vec::new(),
            push_specs: Vec::new(),
            apply_url_aliases: true,
            repo,
        })
    }
}

pub(crate) fn rewrite_urls(
    config: &config::Cache,
    url: Option<&git_url::Url>,
    push_url: Option<&git_url::Url>,
) -> Result<(Option<git_url::Url>, Option<git_url::Url>), remote::init::Error> {
    let rewrite = |url: Option<&git_url::Url>, direction: remote::Direction| {
        url.and_then(|url| config.url_rewrite().rewrite_url(url, direction))
            .map(|url| {
                git_url::parse(&url).map_err(|err| remote::init::Error::RewrittenUrlInvalid {
                    kind: match direction {
                        remote::Direction::Fetch => "fetch",
                        remote::Direction::Push => "push",
                    },
                    source: err,
                    rewritten_url: url,
                })
            })
            .transpose()
    };

    let url_alias = rewrite(url, remote::Direction::Fetch)?;
    let push_url_alias = rewrite(push_url, remote::Direction::Push)?;

    Ok((url_alias, push_url_alias))
}
