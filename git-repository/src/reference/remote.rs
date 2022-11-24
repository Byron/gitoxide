use std::convert::TryInto;

use crate::{remote, Reference};

/// Remotes
impl<'repo> Reference<'repo> {
    /// Find the unvalidated name of our remote for `direction` as configured in `branch.<name>.remote|pushRemote` respectively.
    /// If `Some(<name>)` it can be used in [`Repository::find_remote(…)`][crate::Repository::find_remote()], or if `None` then
    /// [Repository::remote_default_name()][crate::Repository::remote_default_name()] could be used in its place.
    ///
    /// Return `None` if no remote is configured.
    ///
    /// # Note
    ///
    /// - it's recommended to use the [`remote(…)`][Self::remote()] method as it will configure the remote with additional
    ///   information.
    /// - `branch.<name>.pushRemote` falls back to `branch.<name>.remote`.
    pub fn remote_name(&self, direction: remote::Direction) -> Option<remote::Name<'repo>> {
        let name = self.name().shorten();
        let config = &self.repo.config.resolved;
        (direction == remote::Direction::Push)
            .then(|| {
                config
                    .string("branch", Some(name), "pushRemote")
                    .or_else(|| config.string("remote", None, "pushDefault"))
            })
            .flatten()
            .or_else(|| config.string("branch", Some(name), "remote"))
            .and_then(|name| name.try_into().ok())
    }

    /// Like [`remote_name(…)`][Self::remote_name()], but configures the returned `Remote` with additional information like
    ///
    /// - `branch.<name>.merge` to know which branch on the remote side corresponds to this one for merging when pulling.
    ///
    /// It also handles if the remote is a configured URL, which has no name.
    pub fn remote(
        &self,
        direction: remote::Direction,
    ) -> Option<Result<crate::Remote<'repo>, remote::find::existing::Error>> {
        // TODO: use `branch.<name>.merge`
        self.remote_name(direction).map(|name| match name {
            remote::Name::Symbol(name) => self.repo.find_remote(name.as_ref()).map_err(Into::into),
            remote::Name::Url(url) => git_url::parse(url.as_ref()).map_err(Into::into).and_then(|url| {
                self.repo
                    .remote_at(url)
                    .map_err(|err| remote::find::existing::Error::Find(remote::find::Error::Init(err)))
            }),
        })
    }
}
