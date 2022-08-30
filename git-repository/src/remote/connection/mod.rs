use crate::Remote;

pub(crate) struct HandshakeWithRefs {
    #[allow(dead_code)]
    outcome: git_protocol::fetch::handshake::Outcome,
    refs: Vec<git_protocol::fetch::Ref>,
}

pub type CredentialsFn<'a> = Box<dyn FnMut(git_credentials::helper::Action) -> git_credentials::protocol::Result + 'a>;

/// A type to represent an ongoing connection to a remote host, typically with the connection already established.
///
/// It can be used to perform a variety of operations with the remote without worrying about protocol details,
/// much like a remote procedure call.
pub struct Connection<'a, 'repo, T, P> {
    pub(crate) remote: &'a Remote<'repo>,
    pub(crate) credentials: Option<CredentialsFn<'a>>,
    pub(crate) transport: T,
    pub(crate) progress: P,
}

mod access {
    use crate::remote::connection::CredentialsFn;
    use crate::remote::Connection;
    use crate::Remote;

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
            self.credentials = Some(Box::new(helper));
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
        // TODO: take url as parameter
        pub fn configured_credentials() -> CredentialsFn<'a> {
            // TODO: actually fetch this from configuration
            Box::new(git_protocol::credentials::builtin) as CredentialsFn<'_>
        }
        /// Drop the transport and additional state to regain the original remote.
        pub fn remote(&self) -> &Remote<'repo> {
            self.remote
        }
    }
}

mod list_refs;
