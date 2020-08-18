#![forbid(unsafe_code)]
use bstr::BStr;

pub mod packet_line;

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct RemoteProgress<'a> {
    #[cfg_attr(feature = "serde1", serde(borrow))]
    pub action: &'a BStr,
    pub percent: Option<u32>,
    pub step: Option<usize>,
    pub max: Option<usize>,
}

#[doc(inline)]
pub use packet_line::Borrowed as PacketLine;

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum Protocol {
    V1,
    V2,
}

pub mod client {
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

    pub mod git {
        use std::io;

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
        }
    }

    pub trait Connection {
        /// a listing of the Server capabilities, as received with the first request
        /// These are provided in both V1 and V2
        fn cached_capabilities(&self) -> &[&str];
    }

    pub fn connect(url: &[u8], version: crate::Protocol) -> Result<Box<dyn Connection>, Error> {
        let url = git_url::parse(url)?;
        Ok(match url.protocol {
            git_url::Protocol::File => Box::new(
                crate::client::file::connect(url.path.to_path()?, version)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?,
            ),
            git_url::Protocol::Ssh => Box::new(
                crate::client::ssh::connect(
                    &url.host.as_ref().expect("host is present in url"),
                    url.path.to_path()?,
                    version,
                    url.user.as_ref().map(|u| u.as_str()),
                    url.port,
                )
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?,
            ),
            _ => unimplemented!("all protocol connections"),
        })
    }
}

#[doc(inline)]
pub use client::connect;
