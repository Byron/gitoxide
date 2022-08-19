#![allow(missing_docs, dead_code)]

use crate::Remote;

pub(crate) enum State {
    Connected,
    HandshakeWithRefs {
        outcome: git_protocol::fetch::handshake::Outcome,
        refs: Vec<git_protocol::fetch::Ref>,
    },
}

pub struct Connection<'a, 'repo, T, P> {
    pub(crate) remote: &'a Remote<'repo>,
    pub(crate) transport: T,
    pub(crate) progress: P,
    pub(crate) state: State,
}

mod access {
    use crate::remote::Connection;
    use crate::Remote;

    /// Access and conversion
    impl<'a, 'repo, T, P> Connection<'a, 'repo, T, P> {
        /// Obtain the transport from this instance.
        pub fn into_transport(self) -> T {
            self.transport
        }

        /// Drop the transport and additional state to regain the original remote.
        pub fn remote(&self) -> &Remote<'repo> {
            self.remote
        }
    }
}

mod list_refs;
