use crate::Remote;

pub(crate) struct HandshakeWithRefs {
    #[allow(dead_code)]
    outcome: git_protocol::fetch::handshake::Outcome,
    refs: Vec<git_protocol::fetch::Ref>,
}

/// A type to represent an ongoing connection to a remote host, typically with the connection already established.
///
/// It can be used to perform a variety of operations with the remote without worrying about protocol details,
/// much like a remote procedure call.
pub struct Connection<'a, 'repo, T, P> {
    pub(crate) remote: &'a Remote<'repo>,
    pub(crate) credentials:
        Option<Box<dyn FnMut(git_credentials::helper::Action) -> git_credentials::protocol::Result + 'a>>,
    pub(crate) transport: T,
    pub(crate) progress: P,
}

mod access {
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
        pub fn credentials(
            mut self,
            helper: impl FnMut(git_credentials::helper::Action) -> git_credentials::protocol::Result + 'a,
        ) -> Self {
            self.credentials = Some(Box::new(helper));
            self
        }
    }

    /// Access
    impl<'a, 'repo, T, P> Connection<'a, 'repo, T, P> {
        /// Drop the transport and additional state to regain the original remote.
        pub fn remote(&self) -> &Remote<'repo> {
            self.remote
        }
    }
}

mod list_refs;
