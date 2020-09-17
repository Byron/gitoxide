use crate::client::Transport;
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
        UnsupportedUrlTokens(url: bstr::BString, scheme: git_url::Scheme) {
            display("The url '{}' contains information that would not be used by the '{}' protocol", url, scheme)
        }
        #[cfg(not(feature = "http-client-curl"))]
        CompiledWithoutHttp(scheme: git_url::Scheme) {
            display("'{}' is not compiled in. Compile with the 'http' cargo feature", scheme)
        }
    }
}

/// Would be so nice if this wasn't necessary
mod box_impl {
    use crate::{
        client::{self, Error, Identity, MessageKind, RequestWriter, SetServiceResponse, WriteMode},
        Protocol, Service,
    };
    use std::ops::{Deref, DerefMut};

    impl<T: client::Transport + ?Sized> client::Transport for Box<T> {
        fn handshake(&mut self, service: Service) -> Result<SetServiceResponse<'_>, Error> {
            self.deref_mut().handshake(service)
        }

        fn set_identity(&mut self, identity: Identity) -> Result<(), Error> {
            self.deref_mut().set_identity(identity)
        }

        fn request(&mut self, write_mode: WriteMode, on_into_read: MessageKind) -> Result<RequestWriter<'_>, Error> {
            self.deref_mut().request(write_mode, on_into_read)
        }

        fn close(&mut self) -> Result<(), Error> {
            self.deref_mut().close()
        }

        fn to_url(&self) -> String {
            self.deref().to_url()
        }

        fn desired_protocol_version(&self) -> Protocol {
            self.deref().desired_protocol_version()
        }

        fn is_stateful(&self) -> bool {
            self.deref().is_stateful()
        }
    }
}

/// A general purpose connector with just the default configuration.
pub fn connect(url: &[u8], version: crate::Protocol) -> Result<Box<dyn Transport>, Error> {
    let urlb = url;
    let url = git_url::parse(urlb)?;
    Ok(match url.scheme {
        git_url::Scheme::File => {
            if url.user.is_some() || url.host.is_some() || url.port.is_some() {
                return Err(Error::UnsupportedUrlTokens(urlb.into(), url.scheme));
            }
            Box::new(
                crate::client::file::connect(url.path, version)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?,
            )
        }
        git_url::Scheme::Ssh => Box::new(
            crate::client::ssh::connect(
                &url.host.as_ref().expect("host is present in url"),
                url.path,
                version,
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
                    version,
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
                crate::client::http::connect(urlb.to_str()?, version)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?,
            )
        }
    })
}
