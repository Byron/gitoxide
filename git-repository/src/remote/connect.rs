use git_protocol::transport::client::Transport;

use crate::{remote::Connection, Progress, Remote};

mod error {
    use crate::{bstr::BString, remote};

    /// The error returned by [connect()][crate::Remote::connect()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        SchemePermission(#[from] remote::url::scheme_permission::init::Error),
        #[error("Protocol {scheme:?} of url {url:?} is denied per configuration")]
        ProtocolDenied { url: BString, scheme: git_url::Scheme },
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

/// Establishing connections to remote hosts (without performing a git-handshake).
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
            authenticate: None,
            transport_config: None,
            transport,
            progress,
        }
    }

    /// Connect to the url suitable for `direction` and return a handle through which operations can be performed.
    ///
    /// Note that the `protocol.version` configuration key affects the transport protocol used to connect,
    /// with `2` being the default.
    ///
    /// The transport used for connection can be configured via `transport_mut().configure()` assuming the actually
    /// used transport is well known. If that's not the case, the transport can be created by hand and passed to
    /// [to_connection_with_transport()][Self::to_connection_with_transport()].
    #[cfg(any(feature = "blocking-network-client", feature = "async-network-client-async-std"))]
    #[git_protocol::maybe_async::maybe_async]
    pub async fn connect<P>(
        &self,
        direction: crate::remote::Direction,
        progress: P,
    ) -> Result<Connection<'_, 'repo, Box<dyn Transport + Send>, P>, Error>
    where
        P: Progress,
    {
        let (url, version) = self.sanitized_url_and_version(direction)?;
        let transport = git_protocol::transport::connect(url, version).await?;
        Ok(self.to_connection_with_transport(transport, progress))
    }

    /// Produce the sanitized URL and protocol version to use as obtained by querying the repository configuration.
    ///
    /// This can be useful when using custom transports to allow additional configuration.
    pub fn sanitized_url_and_version(
        &self,
        direction: crate::remote::Direction,
    ) -> Result<(git_url::Url, git_protocol::transport::Protocol), Error> {
        fn sanitize(mut url: git_url::Url) -> Result<git_url::Url, Error> {
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

        use git_protocol::transport::Protocol;
        let version = self
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
        if !self.repo.config.url_scheme()?.allow(&url.scheme) {
            return Err(Error::ProtocolDenied {
                url: url.to_bstring(),
                scheme: url.scheme,
            });
        }
        Ok((sanitize(url)?, version))
    }
}
