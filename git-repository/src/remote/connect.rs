#![allow(missing_docs)]

use crate::remote::{connection, Connection};
use crate::{remote, Progress, Remote};
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
        #[error("Could not verify that file:// url is a valid git directory before attempting to use it")]
        FileUrl(#[from] git_discover::is_git::Error),
    }
}
pub use error::Error;

/// Establishing connections to remote hosts
impl<'repo> Remote<'repo> {
    /// Create a new connection using `transport` to communicate, with `progress` to indicate changes.
    ///
    /// Note that this method expects the `transport` to be created by the user, which would involve the [`url()`][Self::url()].
    /// It's meant to be used when async operation is needed with runtimes of the user's choice.
    pub fn to_connection_with_transport<T, P>(&self, transport: T, progress: P) -> Connection<'_, 'repo, T, P>
    where
        T: Transport,
        P: Progress,
    {
        Connection {
            remote: self,
            transport,
            progress,
            state: connection::State::Connected,
        }
    }

    /// Connect to the url suitable for `direction` and return a handle through which operations can be performed.
    #[cfg(any(feature = "blocking-network-client", feature = "async-network-client-async-std"))]
    #[git_protocol::maybe_async::maybe_async]
    pub async fn connect<P>(
        &self,
        direction: remote::Direction,
        progress: P,
    ) -> Result<Connection<'_, 'repo, Box<dyn Transport + Send>, P>, Error>
    where
        P: Progress,
    {
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

        let url = self.processed_url(direction)?;
        let transport = git_protocol::transport::connect(url, protocol).await?;
        Ok(self.to_connection_with_transport(transport, progress))
    }

    fn processed_url(&self, direction: remote::Direction) -> Result<git_url::Url, Error> {
        let mut url = self.url(direction).ok_or(Error::MissingUrl { direction })?.to_owned();
        if url.scheme == git_url::Scheme::File {
            let mut dir = git_path::from_bstr(url.path.as_ref());
            let kind = git_discover::is_git(dir.as_ref()).or_else(|_| {
                dir.to_mut().push(git_discover::DOT_GIT_DIR);
                git_discover::is_git(dir.as_ref())
            })?;
            let (git_dir, _work_dir) = git_discover::repository::Path::from_dot_git_dir(dir.into_owned(), kind)
                .into_repository_and_work_tree_directories();
            url.path = git_path::into_bstr(git_dir).into_owned();
        }
        Ok(url)
    }
}
