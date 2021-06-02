use crate::client::Transport;
use quick_error::quick_error;
quick_error! {
    /// The error used in [`connect()`].
    #[derive(Debug)]
    #[allow(missing_docs)]
    pub enum Error {
        Url(err: git_url::parse::Error) {
            display("The URL could not be parsed")
            from()
            source(err)
        }
        PathConversion(err: bstr::Utf8Error) {
            display("The git repository paths could not be converted to UTF8")
            from()
            source(err)
        }
        Connection(err: Box<dyn std::error::Error + Send + Sync>) {
            display("connection failed")
            from()
            source(&**err)
        }
        UnsupportedUrlTokens(url: bstr::BString, scheme: git_url::Scheme) {
            display("The url '{}' contains information that would not be used by the '{}' protocol", url, scheme)
        }
        UnsupportedScheme(scheme: git_url::Scheme) {
            display("The '{}' protocol is currently unsupported", scheme)
        }
        #[cfg(not(feature = "http-client-curl"))]
        CompiledWithoutHttp(scheme: git_url::Scheme) {
            display("'{}' is not compiled in. Compile with the 'http-client-curl' cargo feature", scheme)
        }
    }
}

/// A general purpose connector connecting to a repository identified by the given `url`.
///
/// This includes connections to
/// [local repositories][crate::client::file::connect()],
/// [repositories over ssh][crate::client::ssh::connect()],
/// [git daemons][crate::client::git::connect()],
/// and if compiled in connections to [git repositories over https][crate::client::http::connect()].
///
/// Use `desired_version` to set the desired protocol version to use when connecting, but not that the server may downgrade it.
pub fn connect(url: &[u8], desired_version: crate::Protocol) -> Result<Box<dyn Transport>, Error> {
    let urlb = url;
    let url = git_url::parse(urlb)?;
    Ok(match url.scheme {
        git_url::Scheme::Radicle => return Err(Error::UnsupportedScheme(url.scheme)),
        git_url::Scheme::File => {
            if url.user.is_some() || url.host.is_some() || url.port.is_some() {
                return Err(Error::UnsupportedUrlTokens(urlb.into(), url.scheme));
            }
            Box::new(
                crate::client::blocking_io::file::connect(url.path, desired_version)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?,
            )
        }
        git_url::Scheme::Ssh => Box::new(
            crate::client::blocking_io::ssh::connect(
                &url.host.as_ref().expect("host is present in url"),
                url.path,
                desired_version,
                url.user.as_deref(),
                url.port,
            )
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?,
        ),
        git_url::Scheme::Git => {
            if url.user.is_some() {
                return Err(Error::UnsupportedUrlTokens(urlb.into(), url.scheme));
            }
            Box::new(
                crate::client::git::connect(
                    &url.host.as_ref().expect("host is present in url"),
                    url.path,
                    desired_version,
                    url.port,
                )
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?,
            )
        }
        #[cfg(not(feature = "http-client-curl"))]
        git_url::Scheme::Https | git_url::Scheme::Http => return Err(Error::CompiledWithoutHttp(url.scheme)),
        #[cfg(feature = "http-client-curl")]
        git_url::Scheme::Https | git_url::Scheme::Http => {
            use bstr::ByteSlice;
            Box::new(
                crate::client::http::connect(urlb.to_str()?, desired_version)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?,
            )
        }
    })
}
