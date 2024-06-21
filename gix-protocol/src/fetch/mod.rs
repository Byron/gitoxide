mod arguments;
pub use arguments::Arguments;

///
#[allow(clippy::empty_docs)]
pub mod delegate;
#[cfg(any(feature = "async-client", feature = "blocking-client"))]
pub use delegate::Delegate;
pub use delegate::{Action, DelegateBlocking};

mod error;
pub use error::Error;
///
#[allow(clippy::empty_docs)]
pub mod response;
pub use response::Response;

mod handshake;
pub use handshake::upload_pack as handshake;

#[cfg(test)]
mod tests;
