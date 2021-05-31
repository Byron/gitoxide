#[cfg(feature = "blocking-client")]
mod blocking_io;
#[cfg(feature = "blocking-client")]
mod _blocking_api_exports {
    #[doc(inline)]
    pub use super::blocking_io::connect::connect;
    #[cfg(feature = "http-client-curl")]
    pub use super::blocking_io::http;
    pub use super::blocking_io::{
        file, git,
        request::{ExtendedBufRead, HandleProgress, RequestWriter},
        ssh, Error, Identity, MessageKind, SetServiceResponse, Transport, TransportV2Ext, WriteMode,
    };
}
#[cfg(feature = "blocking-client")]
pub use _blocking_api_exports::*;

///
pub mod capabilities;
#[doc(inline)]
pub use capabilities::Capabilities;
