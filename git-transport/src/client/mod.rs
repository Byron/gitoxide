#[cfg(all(not(feature = "blocking-client"), feature = "async-client"))]
pub use async_io::{ExtendedBufRead, SetServiceResponse, Transport};
#[cfg(all(feature = "blocking-client", feature = "http-client-curl"))]
pub use blocking_io::http;
#[cfg(feature = "blocking-client")]
pub use blocking_io::{connect, file, git, ssh, ExtendedBufRead, SetServiceResponse, Transport, TransportV2Ext};
#[cfg(feature = "blocking-client")]
#[doc(inline)]
pub use connect::connect;

#[cfg(all(not(feature = "blocking-client"), feature = "async-client"))]
mod async_io;
#[cfg(feature = "blocking-client")]
mod blocking_io;

///
pub mod capabilities;
#[doc(inline)]
pub use capabilities::Capabilities;

mod non_io_types;
pub use non_io_types::{Error, HandleProgress, Identity, MessageKind, WriteMode};

///
#[cfg(feature = "blocking-client")]
pub mod request;
#[doc(inline)]
#[cfg(feature = "blocking-client")]
pub use request::RequestWriter;
