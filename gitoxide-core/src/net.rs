use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub enum Protocol {
    V1,
    V2,
}

impl FromStr for Protocol {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "1" => Protocol::V1,
            "2" => Protocol::V2,
            _ => return Err(format!("Unsupported protocol version '{}', choose '1' or '2'", s)),
        })
    }
}

#[cfg(any(feature = "blocking-client", feature = "async-client"))]
mod impls {
    use super::Protocol;
    use git_repository::protocol::transport;

    impl From<Protocol> for transport::Protocol {
        fn from(v: Protocol) -> Self {
            match v {
                Protocol::V1 => transport::Protocol::V1,
                Protocol::V2 => transport::Protocol::V2,
            }
        }
    }
}

impl Default for Protocol {
    fn default() -> Self {
        // Note that it's very important this remains V2, as V1 may block forver in stateful (i.e. non-http) connections when fetching
        // as we chose not to complicate matters by counting which arguments where sent (just yet).
        Protocol::V2
    }
}
#[cfg(feature = "async-client")]
mod async_io {
    use async_net::TcpStream;
    use futures_lite::FutureExt;
    use git_repository::{
        object::bstr::BString,
        protocol::{
            transport,
            transport::{client, client::connect::Error, client::git},
        },
    };
    use std::{io, time::Duration};

    async fn git_connect(
        host: &str,
        path: BString,
        desired_version: transport::Protocol,
        port: Option<u16>,
    ) -> Result<git::Connection<TcpStream, TcpStream>, Error> {
        let read = TcpStream::connect(&(host, port.unwrap_or(9418)))
            .or(async {
                async_io::Timer::after(Duration::from_secs(5)).await;
                Err(io::ErrorKind::TimedOut.into())
            })
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        let write = read.clone();
        Ok(git::Connection::new(
            read,
            write,
            desired_version,
            path,
            None::<(String, _)>,
            git::ConnectMode::Daemon,
        ))
    }

    pub async fn connect(
        url: &[u8],
        desired_version: transport::Protocol,
    ) -> Result<impl client::Transport + Send, Error> {
        let urlb = url;
        let url = git_repository::url::parse(urlb)?;
        Ok(match url.scheme {
            git_repository::url::Scheme::Git => {
                if url.user.is_some() {
                    return Err(Error::UnsupportedUrlTokens(urlb.into(), url.scheme));
                }
                git_connect(
                    url.host.as_ref().expect("host is present in url"),
                    url.path,
                    desired_version,
                    url.port,
                )
                .await?
            }
            scheme => return Err(Error::UnsupportedScheme(scheme)),
        })
    }
}
#[cfg(feature = "async-client")]
pub use self::async_io::connect;

#[cfg(feature = "blocking-client")]
pub use git_repository::protocol::transport::connect;
