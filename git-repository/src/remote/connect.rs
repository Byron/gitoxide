#![allow(missing_docs)]

use crate::remote::Connection;
use crate::{remote, Remote};
use git_protocol::transport;
use git_protocol::transport::client::Transport;

mod error {
    use crate::bstr::BString;
    use crate::remote;

    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error(transparent)]
        Connect(#[from] git_protocol::transport::client::connect::Error),
        #[error("The {} url was missing - don't know where to establish a connection to", direction.as_str())]
        MissingUrl { direction: remote::Direction },
        #[error("Protocol named {given:?} is not a valid protocol. Choose between 1 and 2")]
        UnknownProtocol { given: BString },
    }
}

pub use error::Error;

/// Establishing connections to remote hosts
impl<'repo> Remote<'repo> {
    /// Create a new connection into `direction` using `transport` to communicate.
    ///
    /// Note that this method expects the `transport` to be created by the user, which would involve the [`url()`][Self::url()].
    /// It's meant to be used when async operation is needed with runtimes of the user's choice.
    pub fn into_connection_with_transport<T>(self, transport: T, direction: remote::Direction) -> Connection<'repo, T>
    where
        T: Transport,
    {
        Connection {
            remote: self,
            direction,
            transport,
        }
    }

    /// Connect to the url suitable for `direction` and return a handle through which operations can be performed.
    #[cfg(feature = "blocking-network-client")]
    pub fn into_connection(
        self,
        direction: remote::Direction,
    ) -> Result<Connection<'repo, Box<dyn Transport + Send>>, Error> {
        use git_protocol::transport::Protocol;
        let protocol = self
            .repo
            .config
            .resolved
            .integer("protocol", None, "version")
            .unwrap_or(Ok(2))
            .map_err(|err| Error::UnknownProtocol { given: err.input })
            .and_then(|num| {
                Ok(match num {
                    1 => Protocol::V1,
                    2 => Protocol::V2,
                    num => {
                        return Err(Error::UnknownProtocol {
                            given: num.to_string().into(),
                        })
                    }
                })
            })?;

        let url = self.url(direction).ok_or(Error::MissingUrl { direction })?.to_owned();
        let transport = transport::connect(url, protocol)?;
        Ok(self.into_connection_with_transport(transport, direction))
    }
}
