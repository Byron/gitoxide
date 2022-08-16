#![allow(missing_docs, dead_code)]

use crate::Remote;

pub struct Connection<'repo, T> {
    pub(crate) remote: Remote<'repo>,
    pub(crate) transport: T,
}

mod access {
    use crate::remote::Connection;
    use crate::Remote;

    /// Conversion
    impl<'repo, T> Connection<'repo, T> {
        /// Dissolve this instance into its parts, `(Remote, Transport)`, the inverse of
        /// [`into_connection_with_transport()`][Remote::into_connection_with_transport()].
        pub fn into_parts(self) -> (Remote<'repo>, T) {
            (self.remote, self.transport)
        }

        /// Drop the transport and additional state to regain the original remote.
        pub fn into_remote(self) -> Remote<'repo> {
            self.remote
        }
    }
}

mod refs;
