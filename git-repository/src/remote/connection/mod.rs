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
    pub(crate) transport: T,
    pub(crate) progress: P,
}

mod access {
    use crate::remote::Connection;
    use crate::Remote;

    /// Access and conversion
    impl<'a, 'repo, T, P> Connection<'a, 'repo, T, P> {
        /// Drop the transport and additional state to regain the original remote.
        pub fn remote(&self) -> &Remote<'repo> {
            self.remote
        }
    }
}

mod list_refs;
