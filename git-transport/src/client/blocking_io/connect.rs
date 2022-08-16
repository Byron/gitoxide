pub use crate::client::non_io_types::connect::Error;

pub(crate) mod function {
    use crate::client::non_io_types::connect::Error;
    use crate::client::Transport;
    use bstr::BStr;

    /// A general purpose connector connecting to a repository identified by the given `url`.
    ///
    /// This includes connections to
    /// [local repositories][crate::client::file::connect()],
    /// [repositories over ssh][crate::client::ssh::connect()],
    /// [git daemons][crate::client::git::connect()],
    /// and if compiled in connections to [git repositories over https][crate::client::http::connect()].
    ///
    /// Use `desired_version` to set the desired protocol version to use when connecting, but note that the server may downgrade it.
    pub fn connect(url: &BStr, desired_version: crate::Protocol) -> Result<Box<dyn Transport + Send>, Error> {
        let urlb = url;
        let mut url = git_url::parse(urlb)?;
        Ok(match url.scheme {
            git_url::Scheme::Radicle => return Err(Error::UnsupportedScheme(url.scheme)),
            git_url::Scheme::File => {
                if url.user().is_some() || url.host().is_some() || url.port.is_some() {
                    return Err(Error::UnsupportedUrlTokens {
                        url: urlb.into(),
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
                        url: urlb.into(),
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
            #[cfg(not(feature = "http-client-curl"))]
            git_url::Scheme::Https | git_url::Scheme::Http => return Err(Error::CompiledWithoutHttp(url.scheme)),
            #[cfg(feature = "http-client-curl")]
            git_url::Scheme::Https | git_url::Scheme::Http => Box::new(
                crate::client::http::connect(urlb, desired_version)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?,
            ),
        })
    }
}
