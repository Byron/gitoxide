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
    use git_repository::protocol::transport::{client, client::connect::Error};

    pub async fn connect(url: &[u8], desired_version: super::Protocol) -> Result<Box<dyn client::Transport>, Error> {
        let urlb = url;
        let url = git_repository::url::parse(urlb)?;
        Ok(match url.scheme {
            git_repository::url::Scheme::Git => {
                if url.user.is_some() {
                    return Err(Error::UnsupportedUrlTokens(urlb.into(), url.scheme));
                }
                todo!("create git connection with async tcp streams")
                // Box::new(
                // crate::client::git::connect(
                //     &url.host.as_ref().expect("host is present in url"),
                //     url.path,
                //     desired_version,
                //     url.port,
                // )
                // .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?,
                // )
            }
            scheme => return Err(Error::UnsupportedScheme(scheme)),
        })
    }
}
#[cfg(feature = "async-client")]
pub use async_io::connect;

#[cfg(feature = "blocking-client")]
pub use git_repository::protocol::transport::connect;
