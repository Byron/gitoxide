use crate::{Protocol, Service};
use quick_error::quick_error;
use std::io;

pub mod connect;
pub mod file;
pub mod git;
#[cfg(feature = "http-curl")]
pub mod http;
pub mod ssh;
#[doc(inline)]
pub use connect::connect;

pub mod capabilities {
    use bstr::{BStr, BString, ByteSlice};
    use quick_error::quick_error;
    use std::convert::TryFrom;

    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            MissingDelimitingNullByte {
                display("Capabilities were missing entirely as there was no 0 byte")
            }
            NoCapabilities {
                display("there was not a single capability behind the delimiter")
            }
        }
    }
    pub struct Capabilities(BString);
    pub struct Capability<'a>(&'a BStr);

    impl<'a> Capability<'a> {
        pub fn name(&self) -> &BStr {
            self.0
                .splitn(2, |b| *b == b'=')
                .next()
                .expect("there is always a single item")
                .as_bstr()
        }
        pub fn value(&self) -> Option<&BStr> {
            self.0.splitn(2, |b| *b == b'=').nth(1).map(|s| s.as_bstr())
        }
    }

    impl Capabilities {
        pub fn iter(&self) -> impl Iterator<Item = Capability> {
            self.0.split(|b| *b == b' ').map(|c| Capability(c.as_bstr()))
        }
    }

    impl TryFrom<&[u8]> for Capabilities {
        type Error = Error;

        fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
            let delimiter = value.find_byte(0).ok_or(Error::MissingDelimitingNullByte)?;
            if delimiter + 1 == value.len() {
                return Err(Error::NoCapabilities);
            }
            let capabilities = &value[delimiter + 1..];
            Ok(Capabilities(capabilities.as_bstr().to_owned()))
        }
    }
}
pub use capabilities::Capabilities;

pub type Refs = Vec<String>;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Io(err: io::Error) {
            display("An IO error occurred when talking to the server")
            from()
            source(err)
        }
        Capabilities(err: capabilities::Error) {
            display("Capabilities could not be parsed")
            from()
            source(err)
        }
        LineDecode(err: git_packetline::decode::Error) {
            display("A packet line could not be decoded")
            from()
            source(err)
        }
        ExpectedLine(message: &'static str) {
            display("A {} line was expected, but there was none", message)
        }
        ExpectedDataLine {
            display("Expected a data line, but got a delimiter")
        }
    }
}

pub struct SetServiceResponse {
    /// The protocol the service can provide. May be different from the requested one
    pub actual_protocol: Protocol,
    pub capabilities: Capabilities,
    /// In protocol version one, this is set to a list of refs and their peeled counterparts.
    pub refs: Option<Box<dyn io::BufRead>>,
}

/// All methods provided here must be called in the correct order according to the communication protocol used to connect to them.
/// It does, however, know just enough to be able to provide a higher-level interface than would otherwise be possible.
/// Thus the consumer of this trait will not have to deal with packet lines at all.
/// Generally, whenever a `Read` trait or `Write` trait is produced, it must be exhausted..
pub trait TransportSketch {
    /// Initiate connection to the given service.
    /// Returns the service capabilities according according to the actual Protocol it supports,
    /// and possibly a list of refs to be obtained.
    /// This means that asking for an unsupported protocol will result in a protocol downgrade to the given one.
    /// using the `read_line(…)` function of the given BufReader. It must be exhausted, that is, read to the end,
    /// before the next method can be invoked.
    fn set_service(&mut self, service: Service) -> Result<SetServiceResponse, Error>;

    //TODO: A way to terminate the connection gracefully with 'flush' (V1) and noop in V2
}

pub trait Transport {}
