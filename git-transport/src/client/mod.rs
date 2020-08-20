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

pub type Capabilities = Vec<String>;
pub type Refs = Vec<String>;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Io(err: io::Error) {
            display("An IO error occurred when talking to the server")
            from()
            source(err)
        }
    }
}

pub struct SetServiceResponse {
    /// The protocol the service can provide. May be different from the requested one
    actual_protocol: Protocol,
    capabilities: Capabilities,
    /// In protocol version one, this is set to a list of refs and their peeled counterparts.
    refs: Option<Box<dyn io::BufRead>>,
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
    fn set_service(&mut self, service: Service, protocol: crate::Protocol) -> Result<SetServiceResponse, Error>;
}

pub trait Transport {
    /// a listing of the Server capabilities, as received with the first request
    /// These are provided in both V1 and V2
    fn set_service(&self) -> &[&str];

    /// List capabilities for the given `command`, if any. Return true if some were added, false otherwise.
    /// This allows to use the command-like interface of protocol V2.
    fn command_capabilities(&self, command: &str, out: &mut Vec<&str>) -> bool;
}
