use crate::bstr::BStr;
use crate::{remote, Remote};
use std::convert::TryInto;

/// Builder methods
impl Remote<'_> {
    /// Set the `url` to be used when pushing data to a remote.
    pub fn push_url<Url, E>(self, url: Url) -> Result<Self, remote::init::Error>
    where
        Url: TryInto<git_url::Url, Error = E>,
        git_url::parse::Error: From<E>,
    {
        self.push_url_inner(url, true)
    }

    /// Set the `url` to be used when pushing data to a remote, without applying rewrite rules in case these could be faulty,
    /// eliminating one failure mode.
    pub fn push_url_without_url_rewrite<Url, E>(self, url: Url) -> Result<Self, remote::init::Error>
    where
        Url: TryInto<git_url::Url, Error = E>,
        git_url::parse::Error: From<E>,
    {
        self.push_url_inner(url, false)
    }

    fn push_url_inner<Url, E>(mut self, push_url: Url, should_rewrite_urls: bool) -> Result<Self, remote::init::Error>
    where
        Url: TryInto<git_url::Url, Error = E>,
        git_url::parse::Error: From<E>,
    {
        let push_url = push_url
            .try_into()
            .map_err(|err| remote::init::Error::Url(err.into()))?;
        self.push_url = push_url.into();

        let (_, push_url_alias) = should_rewrite_urls
            .then(|| remote::create::rewrite_urls(&self.repo.config, None, self.push_url.as_ref()))
            .unwrap_or(Ok((None, None)))?;
        self.push_url_alias = push_url_alias;

        Ok(self)
    }

    /// Add `spec` as refspec for `direction` to our list if it's unique.
    pub fn with_refspec<'a>(
        mut self,
        spec: impl Into<&'a BStr>,
        direction: remote::Direction,
    ) -> Result<Self, git_refspec::parse::Error> {
        use remote::Direction::*;
        let spec = git_refspec::parse(
            spec.into(),
            match direction {
                Push => git_refspec::parse::Operation::Push,
                Fetch => git_refspec::parse::Operation::Fetch,
            },
        )?
        .to_owned();
        let specs = match direction {
            Push => &mut self.push_specs,
            Fetch => &mut self.fetch_specs,
        };
        if !specs.contains(&spec) {
            specs.push(spec);
        }
        Ok(self)
    }
}
