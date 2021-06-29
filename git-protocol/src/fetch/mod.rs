mod arguments;
pub use arguments::Arguments;

///
pub mod command;
pub use command::Command;

/// Returns the name of the agent as key-value pair, commonly used in HTTP headers.
pub fn agent() -> (&'static str, Option<&'static str>) {
    ("agent", Some(concat!("git/oxide-", env!("CARGO_PKG_VERSION"))))
}

///
pub mod delegate;
#[cfg(any(feature = "async-client", feature = "blocking-client"))]
pub use delegate::Delegate;
pub use delegate::{Action, DelegateBlocking, LsRefsAction};

mod error;
pub use error::Error;

///
pub mod refs;
pub use refs::Ref;

///
pub mod response;
pub use response::Response;

#[cfg(any(feature = "async-client", feature = "blocking-client"))]
mod function;
#[cfg(any(feature = "async-client", feature = "blocking-client"))]
pub use function::fetch;

#[cfg(test)]
mod tests;
