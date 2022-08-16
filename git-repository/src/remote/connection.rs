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

mod refs {
    use crate::remote::Connection;
    use git_protocol::transport::client::Transport;

    impl<'repo, T> Connection<'repo, T>
    where
        T: Transport,
    {
        /// List all references on the remote that have been filtered through our remote's [`refspecs`][crate::Remote::refspecs()]
        /// for _fetching_.
        ///
        /// This comes in the form of information of all matching tips on the remote and the object they point to, along with
        /// with the local tracking branch of these tips (if available).
        ///
        /// Note that this doesn't fetch the objects mentioned in the tips nor does it make any change to underlying repository.
        pub fn refs(&mut self) -> ! {
            todo!()
        }
    }
}
