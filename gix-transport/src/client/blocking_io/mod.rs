///
#[allow(clippy::empty_docs)]
pub mod connect;

///
#[allow(clippy::empty_docs)]
pub mod file;
///
#[cfg(feature = "http-client")]
pub mod http;

mod bufread_ext;
pub use bufread_ext::{ExtendedBufRead, HandleProgress, ReadlineBufRead};

mod request;
pub use request::RequestWriter;

///
#[allow(clippy::empty_docs)]
pub mod ssh;

mod traits;
pub use traits::{SetServiceResponse, Transport, TransportV2Ext};
