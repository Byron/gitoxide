use crate::{remote, Reference};

/// Remotes
impl<'repo> Reference<'repo> {
    /// Find the name of our remote for `direction` as configured in `branch.<name>.remote|pushRemote` respectively.
    /// If `Some(<name>)` it can be used in [`Repository::find_remote(…)`][crate::Repository::find_remote()], or if `None` then
    /// [`Repository::remote_default_name()`][crate::Repository::remote_default_name()] could be used in its place.
    ///
    /// Return `None` if no remote is configured.
    ///
    /// # Note
    ///
    /// - it's recommended to use the [`remote(…)`][Self::remote()] method as it will configure the remote with additional
    ///   information.
    /// - `branch.<name>.pushRemote` falls back to `branch.<name>.remote`.
    pub fn remote_name(&self, direction: remote::Direction) -> Option<remote::Name<'repo>> {
        self.repo.branch_remote_name(self.name().shorten(), direction)
    }

    /// Like [`branch_remote(…)`](crate::Repository::branch_remote()), but automatically provides the reference name
    /// for configuration lookup.
    pub fn remote(
        &self,
        direction: remote::Direction,
    ) -> Option<Result<crate::Remote<'repo>, remote::find::existing::Error>> {
        self.repo.branch_remote(self.name().shorten(), direction)
    }
}
