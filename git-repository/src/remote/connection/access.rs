use crate::{
    remote::{connection::AuthenticateFn, Connection},
    Remote,
};

/// Builder
impl<'a, 'repo, T, P> Connection<'a, 'repo, T, P> {
    /// Set a custom credentials callback to provide credentials if the remotes require authentication.
    ///
    /// Otherwise we will use the git configuration to perform the same task as the `git credential` helper program,
    /// which is calling other helper programs in succession while resorting to a prompt to obtain credentials from the
    /// user.
    ///
    /// A custom function may also be used to prevent accessing resources with authentication.
    pub fn with_credentials(
        mut self,
        helper: impl FnMut(git_credentials::helper::Action) -> git_credentials::protocol::Result + 'a,
    ) -> Self {
        self.authenticate = Some(Box::new(helper));
        self
    }
}

/// Access
impl<'a, 'repo, T, P> Connection<'a, 'repo, T, P> {
    /// A utility to return a function that will use this repository's configuration to obtain credentials, similar to
    /// what `git credential` is doing.
    ///
    /// It's meant to be used by users of the [`with_credentials()`][Self::with_credentials()] builder to gain access to the
    /// default way of handling credentials, which they can call as fallback.
    pub fn configured_credentials(
        &self,
        url: git_url::Url,
    ) -> Result<AuthenticateFn<'a>, crate::config::credential_helpers::Error> {
        let (mut cascade, _action_with_normalized_url, prompt_opts) =
            self.remote.repo.config_snapshot().credential_helpers(url)?;
        Ok(Box::new(move |action| cascade.invoke(action, prompt_opts.clone())) as AuthenticateFn<'_>)
    }
    /// Return the underlying remote that instantiate this connection.
    pub fn remote(&self) -> &Remote<'repo> {
        self.remote
    }

    /// Provide a mutable transport to allow configuring it with [`configure()`][git_protocol::transport::client::TransportWithoutIO::configure()]
    pub fn transport_mut(&mut self) -> &mut T {
        &mut self.transport
    }
}
