use crate::bstr::{ByteSlice, ByteVec};
use crate::{remote, Reference};
use std::borrow::Cow;

/// Remotes
impl<'repo> Reference<'repo> {
    /// Find the name of our remote for `direction` as configured in `branch.<name>.remote|pushRemote` respectively.
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
    // TODO: return &BStr instead, after all these can also be paths
    pub fn remote_name(&self, direction: remote::Direction) -> Option<Cow<'repo, str>> {
        let name = self.name().shorten().to_str().ok()?;
        let config = &self.repo.config.resolved;
        (direction == remote::Direction::Push)
            .then(|| {
                config
                    .string("branch", Some(name), "pushRemote")
                    .or_else(|| config.string("remote", None, "pushDefault"))
            })
            .flatten()
            .or_else(|| config.string("branch", Some(name), "remote"))
            .and_then(|name| match name {
                Cow::Borrowed(n) => n.to_str().ok().map(Cow::Borrowed),
                Cow::Owned(n) => Vec::from(n).into_string().ok().map(Cow::Owned),
            })
    }

    /// Like [`remote_name(…)`][Self::remote_name()], but configures the returned `Remote` with additional information like
    ///
    /// - `branch.<name>.merge` to know which branch on the remote side corresponds to this one for merging when pulling.
    pub fn remote(
        &self,
        direction: remote::Direction,
    ) -> Option<Result<crate::Remote<'repo>, remote::find::existing::Error>> {
        let name_or_url = self.remote_name(direction)?;
        // TODO: use `branch.<name>.merge`
        name_or_url
            .contains('/')
            .then(|| {
                git_url::parse(name_or_url.as_ref().into())
                    .map_err(Into::into)
                    .and_then(|url| {
                        self.repo
                            .remote_at(url)
                            .map_err(|err| remote::find::existing::Error::Find(remote::find::Error::Init(err)))
                    })
            })
            .or_else(|| Some(self.repo.find_remote(name_or_url.as_ref()).into()))
    }
}
