#[cfg(feature = "async-client")]
mod async_io;
#[cfg(feature = "async-client")]
pub use async_io::{
    connect, ExtendedBufRead, HandleProgress, ReadlineBufRead, RequestWriter, SetServiceResponse, Transport,
    TransportV2Ext,
};

mod traits;
pub use traits::TransportWithoutIO;

#[cfg(feature = "blocking-client")]
mod blocking_io;
#[cfg(feature = "http-client")]
pub use blocking_io::http;
#[cfg(feature = "blocking-client")]
pub use blocking_io::{
    connect, file, ssh, ExtendedBufRead, HandleProgress, ReadlineBufRead, RequestWriter, SetServiceResponse, Transport,
    TransportV2Ext,
};
#[cfg(feature = "blocking-client")]
#[doc(inline)]
pub use connect::function::connect;

///
pub mod capabilities;
#[doc(inline)]
pub use capabilities::Capabilities;

mod non_io_types;
pub use git_sec::identity::Account;
pub use non_io_types::{Error, MessageKind, WriteMode};

///
#[cfg(any(feature = "blocking-client", feature = "async-client"))]
pub mod git;
