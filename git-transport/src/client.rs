pub mod file {
    use crate::client::git;
    use quick_error::quick_error;
    use std::{path::Path, process};

    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            Tbd {
                display("tbd")
            }
        }
    }

    pub fn connect(
        _path: &Path,
        _version: crate::Protocol,
    ) -> Result<git::Connection<process::ChildStdout, process::ChildStdin>, Error> {
        unimplemented!("file connection")
    }
}

pub mod ssh {
    use crate::client::git;
    use quick_error::quick_error;
    use std::{path::Path, process};

    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            Tbd {
                display("tbd")
            }
        }
    }

    pub fn connect(
        _host: &str,
        _path: &Path,
        _version: crate::Protocol,
        _user: Option<&str>,
        _port: Option<u16>,
    ) -> Result<git::Connection<process::ChildStdout, process::ChildStdin>, Error> {
        unimplemented!("file connection")
    }
}

pub mod http {
    use crate::client::git;
    use quick_error::quick_error;
    use std::{path::Path, process};

    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            Tbd {
                display("tbd")
            }
        }
    }

    pub fn connect(
        _host: &str,
        _path: &Path,
        _version: crate::Protocol,
        _user: Option<&str>,
        _port: Option<u16>,
        _secure: bool,
    ) -> Result<git::Connection<process::ChildStdout, process::ChildStdin>, Error> {
        unimplemented!("file connection")
    }
}

pub mod git {
    use std::{io, net::TcpStream, path::Path};

    pub struct Connection<R, W> {
        _read: R,
        _write: W,
    }

    impl<R, W> crate::client::Connection for Connection<R, W>
    where
        R: io::Read,
        W: io::Write,
    {
        fn cached_capabilities(&self) -> &[&str] {
            unimplemented!("cached capabilities")
        }

        fn command_capabilities(&self, _command: &str, _out: &mut Vec<&str>) -> bool {
            unimplemented!("command capabilities")
        }
    }

    use quick_error::quick_error;
    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            Tbd {
                display("tbd")
            }
        }
    }

    pub fn connect(
        _host: &str,
        _path: &Path,
        _version: crate::Protocol,
        _port: Option<u16>,
    ) -> Result<Connection<TcpStream, TcpStream>, Error> {
        unimplemented!("file connection")
    }
}

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
    }
}

pub trait Connection {
    /// a listing of the Server capabilities, as received with the first request
    /// These are provided in both V1 and V2
    fn cached_capabilities(&self) -> &[&str];

    /// List capabilities for the given `command`, if any. Return true if some were added, false otherwise.
    /// This allows to use the command-like interface of protocol V2.
    fn command_capabilities(&self, command: &str, out: &mut Vec<&str>) -> bool;
}

/// A general purpose connector with just the default configuration.
pub fn connect(url: &[u8], version: crate::Protocol) -> Result<Box<dyn Connection>, Error> {
    let urlb = url;
    let url = git_url::parse(urlb)?;
    Ok(match url.protocol {
        git_url::Protocol::File => {
            if url.user.is_some() || url.host.is_some() || url.port.is_some() {
                return Err(Error::UnsupportedUrlTokens(urlb.into(), url.protocol));
            }
            Box::new(
                crate::client::file::connect(url.path.to_path()?, version)
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
                    url.path.to_path()?,
                    version,
                    url.port,
                )
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?,
            )
        }
        git_url::Protocol::Https | git_url::Protocol::Http => Box::new(
            crate::client::http::connect(
                &url.host.as_ref().expect("host is present in url"),
                url.path.to_path()?,
                version,
                url.user.as_deref(),
                url.port,
                url.protocol == git_url::Protocol::Https,
            )
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?,
        ),
    })
}
