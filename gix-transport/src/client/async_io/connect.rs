pub use crate::client::non_io_types::connect::{Error, Options};

#[cfg(feature = "async-std")]
pub(crate) mod function {
    use std::convert::TryInto;

    use crate::client::{git, non_io_types::connect::Error};

    /// A general purpose connector connecting to a repository identified by the given `url`.
    ///
    /// This includes connections to
    /// [git daemons][crate::client::git::connect()] only at the moment.
    ///
    /// Use `options` to further control specifics of the transport resulting from the connection.
    pub async fn connect<Url, E>(
        url: Url,
        options: super::Options,
    ) -> Result<Box<dyn crate::client::Transport + Send>, Error>
    where
        Url: TryInto<gix_url::Url, Error = E>,
        gix_url::parse::Error: From<E>,
    {
        let mut url = url.try_into().map_err(gix_url::parse::Error::from)?;
        Ok(match url.scheme {
            gix_url::Scheme::Git => {
                if url.user().is_some() {
                    return Err(Error::UnsupportedUrlTokens {
                        url: url.to_bstring(),
                        scheme: url.scheme,
                    });
                }
                let path = std::mem::take(&mut url.path);
                Box::new(
                    git::Connection::new_tcp(
                        url.host().expect("host is present in url"),
                        url.port,
                        path,
                        options.version,
                        options.trace,
                    )
                    .await
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?,
                )
            }
            scheme => return Err(Error::UnsupportedScheme(scheme)),
        })
    }
}
