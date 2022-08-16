pub use crate::client::non_io_types::connect::Error;

#[cfg(any(feature = "async-std"))]
pub(crate) mod function {
    use crate::client::git;
    use crate::client::non_io_types::connect::Error;
    use bstr::BStr;
    use std::convert::TryInto;

    /// A general purpose connector connecting to a repository identified by the given `url`.
    ///
    /// This includes connections to
    /// [git daemons][crate::client::git::connect()] only at the moment.
    ///
    /// Use `desired_version` to set the desired protocol version to use when connecting, but note that the server may downgrade it.
    pub async fn connect<Url, E>(
        url: &BStr,
        desired_version: crate::Protocol,
    ) -> Result<impl crate::client::Transport + Send, Error>
    where
        Url: TryInto<git_url::Url, Error = E>,
        git_url::parse::Error: From<E>,
    {
        let mut url = url.try_into().map_err(git_url::parse::Error::from)?;
        Ok(match url.scheme {
            git_url::Scheme::Git => {
                if url.user().is_some() {
                    return Err(Error::UnsupportedUrlTokens {
                        url: urlb.into(),
                        scheme: url.scheme,
                    });
                }
                let path = std::mem::take(&mut url.path);
                git::Connection::new_tcp(
                    url.host().expect("host is present in url"),
                    url.port,
                    path,
                    desired_version,
                )
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?
            }
            scheme => return Err(Error::UnsupportedScheme(scheme)),
        })
    }
}
