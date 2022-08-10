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
    pub fn remote_name(&self, direction: remote::Direction) -> Option<Cow<'repo, str>> {
        let name = self.name().shorten().to_str().ok()?;
        (direction == remote::Direction::Push)
            .then(|| self.repo.config.resolved.string("branch", Some(name), "pushRemote"))
            .flatten()
            .or_else(|| self.repo.config.resolved.string("branch", Some(name), "remote"))
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
        let name = self.remote_name(direction)?;
        // TODO: use `branch.<name>.merge`
        self.repo.find_remote(name.as_ref()).into()
    }
}
