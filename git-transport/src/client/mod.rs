#[cfg(all(not(feature = "blocking-client"), feature = "async-client"))]
mod async_io;
#[cfg(all(not(feature = "blocking-client"), feature = "async-client"))]
pub use async_io::{ExtendedBufRead, HandleProgress, RequestWriter, SetServiceResponse, Transport, TransportV2Ext};

mod traits;
pub use traits::TransportWithoutIO;

#[cfg(feature = "blocking-client")]
mod blocking_io;
#[cfg(all(feature = "blocking-client", feature = "http-client-curl"))]
pub use blocking_io::http;
#[cfg(feature = "blocking-client")]
pub use blocking_io::{
    connect, file, ssh, ExtendedBufRead, HandleProgress, RequestWriter, SetServiceResponse, Transport, TransportV2Ext,
};
#[cfg(feature = "blocking-client")]
#[doc(inline)]
pub use connect::connect;

///
pub mod capabilities;
#[doc(inline)]
pub use capabilities::Capabilities;

mod non_io_types;
pub use non_io_types::{Error, Identity, MessageKind, WriteMode};

///
#[cfg(any(feature = "blocking-client", feature = "async-client"))]
pub mod git;
