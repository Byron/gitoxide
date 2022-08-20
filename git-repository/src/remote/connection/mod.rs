#![allow(missing_docs, dead_code)]

use crate::Remote;

pub(crate) enum State {
    Connected,
    HandshakeWithRefs {
        outcome: git_protocol::fetch::handshake::Outcome,
        refs: Vec<git_protocol::fetch::Ref>,
    },
}

pub struct Connection<'a, 'repo, T, P>
where
    T: git_protocol::transport::client::Transport,
{
    pub(crate) remote: &'a Remote<'repo>,
    pub(crate) transport: T,
    pub(crate) progress: P,
    pub(crate) state: State,
}

mod impls {
    use crate::remote::Connection;
    use git_protocol::transport::client::Transport;

    impl<'a, 'repo, T, P> Drop for Connection<'a, 'repo, T, P>
    where
        T: Transport,
    {
        fn drop(&mut self) {
            git_protocol::fetch::indicate_end_of_interaction(&mut self.transport).ok();
        }
    }
}

mod access {
    use crate::remote::Connection;
    use crate::Remote;

    /// Access and conversion
    impl<'a, 'repo, T, P> Connection<'a, 'repo, T, P>
    where
        T: git_protocol::transport::client::Transport,
    {
        /// Drop the transport and additional state to regain the original remote.
        pub fn remote(&self) -> &Remote<'repo> {
            self.remote
        }
    }
}

mod list_refs;
