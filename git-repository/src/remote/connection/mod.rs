#![allow(missing_docs, dead_code)]

use crate::Remote;

pub struct Connection<'a, 'repo, T> {
    pub(crate) remote: &'a Remote<'repo>,
    pub(crate) transport: T,
}

mod access {
    use crate::remote::Connection;
    use crate::Remote;

    /// Access and conversion
    impl<'a, 'repo, T> Connection<'a, 'repo, T> {
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

mod refs;
