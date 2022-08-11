use crate::{remote, Remote};
use git_refspec::RefSpec;
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
}

/// Modification
impl Remote<'_> {
    /// Read `url.<base>.insteadOf|pushInsteadOf` configuration variables and apply them to our urls, changing them in place.
    ///
    /// This happens only once, and none of them is changed even if only one of them has an error.
    pub fn rewrite_urls(&mut self) -> Result<&mut Self, remote::init::Error> {
        let (url, push_url) =
            remote::create::rewrite_urls(&self.repo.config, self.url.as_ref(), self.push_url.as_ref())?;
        self.url_alias = url;
        self.push_url_alias = push_url;
        Ok(self)
    }
}

/// Accesss
impl Remote<'_> {
    /// Return the name of this remote or `None` if it wasn't persisted to disk yet.
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Return the set of ref-specs used for `direction`, which may be empty, in order of occurrence in the configuration.
    pub fn refspecs(&self, direction: remote::Direction) -> &[RefSpec] {
        match direction {
            remote::Direction::Fetch => &self.fetch_specs,
            remote::Direction::Push => &self.push_specs,
        }
    }

    /// Return the url used for the given `direction` with rewrites from `url.<base>.insteadOf|pushInsteadOf`, unless the instance
    /// was created with one of the `_without_url_rewrite()` methods.
    /// For pushing, this is the `remote.<name>.pushUrl` or the `remote.<name>.url` used for fetching, and for fetching it's
    /// the `remote.<name>.url`.
    /// Note that it's possible to only have the push url set, in which case there will be no way to fetch from the remote as
    /// the push-url isn't used for that.
    pub fn url(&self, direction: remote::Direction) -> Option<&git_url::Url> {
        match direction {
            remote::Direction::Fetch => self.url_alias.as_ref().or(self.url.as_ref()),
            remote::Direction::Push => self
                .push_url_alias
                .as_ref()
                .or(self.push_url.as_ref())
                .or_else(|| self.url(remote::Direction::Fetch)),
        }
    }
}
