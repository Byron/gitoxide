pub use crate::client::non_io_types::connect::Error;

pub(crate) mod function {
    use std::convert::TryInto;

    use crate::client::{non_io_types::connect::Error, Transport};

    /// A general purpose connector connecting to a repository identified by the given `url`.
    ///
    /// This includes connections to
    /// [local repositories][crate::client::file::connect()],
    /// [repositories over ssh][crate::client::ssh::connect()],
    /// [git daemons][crate::client::git::connect()],
    /// and if compiled in connections to [git repositories over https][crate::client::http::connect()].
    ///
    /// Use `desired_version` to set the desired protocol version to use when connecting, but note that the server may downgrade it.
    pub fn connect<Url, E>(url: Url, desired_version: crate::Protocol) -> Result<Box<dyn Transport + Send>, Error>
    where
        Url: TryInto<git_url::Url, Error = E>,
        git_url::parse::Error: From<E>,
    {
        let mut url = url.try_into().map_err(git_url::parse::Error::from)?;
        Ok(match url.scheme {
            git_url::Scheme::Ext(_) => return Err(Error::UnsupportedScheme(url.scheme)),
            git_url::Scheme::File => {
                if url.user().is_some() || url.host().is_some() || url.port.is_some() {
                    return Err(Error::UnsupportedUrlTokens {
                        url: url.to_bstring(),
                        scheme: url.scheme,
                    });
                }
                Box::new(
                    crate::client::blocking_io::file::connect(url.path, desired_version)
                        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?,
                )
            }
            git_url::Scheme::Ssh => Box::new({
                let path = std::mem::take(&mut url.path);
                crate::client::blocking_io::ssh::connect(
                    url.host().expect("host is present in url"),
                    path,
                    desired_version,
                    url.user(),
                    url.port,
                )
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?
            }),
            git_url::Scheme::Git => {
                if url.user().is_some() {
                    return Err(Error::UnsupportedUrlTokens {
                        url: url.to_bstring(),
                        scheme: url.scheme,
                    });
                }
                Box::new({
                    let path = std::mem::take(&mut url.path);
                    crate::client::git::connect(
                        url.host().expect("host is present in url"),
                        path,
                        desired_version,
                        url.port,
                    )
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?
                })
            }
            #[cfg(not(any(feature = "http-client-curl", feature = "http-client-reqwest")))]
            git_url::Scheme::Https | git_url::Scheme::Http => return Err(Error::CompiledWithoutHttp(url.scheme)),
            #[cfg(any(feature = "http-client-curl", feature = "http-client-reqwest"))]
            git_url::Scheme::Https | git_url::Scheme::Http => Box::new(crate::client::http::connect(
                &url.to_bstring().to_string(),
                desired_version,
            )),
        })
    }
}
