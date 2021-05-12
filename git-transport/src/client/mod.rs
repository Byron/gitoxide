#[cfg(feature = "blocking-client")]
mod blocking;
#[cfg(feature = "blocking-client")]
mod _blocking_api_exports {
    pub use super::blocking::capabilities;
    #[doc(inline)]
    pub use super::blocking::capabilities::Capabilities;
    #[doc(inline)]
    pub use super::blocking::connect::connect;
    #[cfg(feature = "http-client-curl")]
    pub use super::blocking::http;
    pub use super::blocking::request::{ExtendedBufRead, HandleProgress, RequestWriter};
    pub use super::blocking::{file, git, ssh};

    pub use super::blocking::{Error, Identity, MessageKind, SetServiceResponse, Transport, TransportV2Ext, WriteMode};
}
#[cfg(feature = "blocking-client")]
pub use _blocking_api_exports::*;
