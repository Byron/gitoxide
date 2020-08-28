use crate::client::Transport;
use bstr::ByteSlice;
use quick_error::quick_error;
quick_error! {
    #[derive(Debug)]
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
        UnsupportedUrlTokens(url: bstr::BString, scheme: git_url::Protocol) {
            display("The url '{}' contains information that would not be used by the '{}' protocol", url, scheme)
        }
        #[cfg(not(feature = "http-client-curl"))]
        CompiledWithoutHttp(scheme: git_url::Protocol) {
            display("'{}' is not compiled in. Compile with the 'http' cargo feature", scheme)
        }
    }
}

/// A general purpose connector with just the default configuration.
pub fn connect(url: &[u8], version: crate::Protocol) -> Result<Box<dyn Transport>, Error> {
    let urlb = url;
    let url = git_url::parse(urlb)?;
    Ok(match url.protocol {
        git_url::Protocol::File => {
            if url.user.is_some() || url.host.is_some() || url.port.is_some() {
                return Err(Error::UnsupportedUrlTokens(urlb.into(), url.protocol));
            }
            Box::new(
                crate::client::file::connect(url.path)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?,
            )
        }
        git_url::Protocol::Ssh => Box::new(
            crate::client::ssh::connect(
                &url.host.as_ref().expect("host is present in url"),
                url.path.to_path()?,
                version,
                url.user.as_deref(),
                url.port,
            )
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?,
        ),
        git_url::Protocol::Git => {
            if url.user.is_some() {
                return Err(Error::UnsupportedUrlTokens(urlb.into(), url.protocol));
            }
            Box::new(
                crate::client::git::connect(
                    &url.host.as_ref().expect("host is present in url"),
                    url.path,
                    version,
                    url.port,
                )
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?,
            )
        }
        #[cfg(not(feature = "http-client-curl"))]
        git_url::Protocol::Https | git_url::Protocol::Http => return Err(Error::CompiledWithoutHttp(url.protocol)),
        #[cfg(feature = "http-client-curl")]
        git_url::Protocol::Https | git_url::Protocol::Http => Box::new(
            crate::client::http::connect(urlb.to_str()?, version)
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?,
        ),
    })
}
