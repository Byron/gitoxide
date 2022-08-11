use crate::{remote, Remote};
use git_refspec::RefSpec;

/// Builder methods
impl Remote<'_> {
    /// By default, `url.<base>.insteadOf|pushInsteadOf` configuration variables will be applied to rewrite fetch and push
    /// urls unless `toggle` is `false`.
    pub fn apply_url_aliases(mut self, toggle: bool) -> Self {
        self.apply_url_aliases = toggle;
        self
    }
}

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

    /// Return the url used for the given `direction` with rewrites from `url.<base>.insteadOf|pushInsteadOf` applied unless
    /// [`apply_url_aliases(false)`][Self::apply_url_aliases()] was called before.
    /// For pushing, this is the `remote.<name>.pushUrl` or the `remote.<name>.url` used for fetching, and for fetching it's
    /// the `remote.<name>.url`.
    /// Note that it's possible to only have the push url set, in which case there will be no way to fetch from the remote as
    /// the push-url isn't used for that.
    pub fn url(&self, direction: remote::Direction) -> Option<&git_url::Url> {
        match direction {
            remote::Direction::Fetch => self
                .apply_url_aliases
                .then(|| self.url_alias.as_ref())
                .flatten()
                .or(self.url.as_ref()),
            remote::Direction::Push => self
                .apply_url_aliases
                .then(|| self.push_url_alias.as_ref())
                .flatten()
                .or(self.push_url.as_ref())
                .or_else(|| self.url(remote::Direction::Fetch)),
        }
    }
}
