#![forbid(unsafe_code)]

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
        use std::{path::PathBuf, process};

        quick_error! {
            #[derive(Debug)]
            pub enum Error {
                Url(err: git_url::parse::Error) {
                    display("The URL could not be parsed")
                    from()
                    source(err)
                }
            }
        }

        pub fn connect(
            _path: PathBuf,
            _version: crate::Protocol,
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

    use quick_error::quick_error;
    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            Url(err: git_url::parse::Error) {
                display("The URL could not be parsed")
                from()
                source(err)
            }
            ExpandPath(err: git_url::expand_path::Error) {
                display("The git repository paths could not be expanded")
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
        fn cached_capabilities(&self) -> &[&str];
    }

    pub fn connect(url: &[u8], version: crate::Protocol) -> Result<Box<dyn Connection>, Error> {
        let url = git_url::parse(url)?;
        Ok(match url.protocol {
            git_url::Protocol::File => Box::new(
                crate::client::file::connect(url.expand_user()?, version)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?,
            ),
            _ => unimplemented!("all protocol connections"),
        })
    }
}

#[doc(inline)]
pub use client::connect;
